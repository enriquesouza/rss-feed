# Playbook do algoritmo do X — para @enriquesouza_ e @AizzyAi

> Pesquisa: 2026-08-03. Fontes primárias: `github.com/xai-org/x-algorithm` (repo aberto
> em jan/2026, era Grok, Rust+Python; último drop relevante 15/mai/2026) e os pesos do
> heavy-ranker de 2023 (`twitter/the-algorithm-ml`). Os pesos numéricos exatos de 2026
> são retidos (`crate::params`), mas a taxonomia de sinais foi confirmada no código novo.

## O que mudou na era Grok (2025→2026)

- Repo novo: **`xai-org/x-algorithm`** — Home Mixer (Rust), Thunder (in-network store),
  **Phoenix** (transformer derivado do Grok-1 para retrieval + ranking), Grox (VLM de
  qualidade/spam/safety). "Eliminamos todo feature hand-engineered" — TweepCred e
  SimClusters saíram do caminho de ranking.
- Score = Σ (peso × P(ação)) sobre ~15-19 ações previstas: like, reply, repost, quote,
  click, profile click, video quality view, photo expand, **share por DM e copy-link**
  (sinais de primeira classe), dwell, **follow-from-feed** (o positivo raro mais forte),
  e negativos: not-interested, mute, block, report.
- **Penalidade de link em post: removida oficialmente** (Nikita Bier + Musk, dez/2025;
  `CLICK_WEIGHT` positivo no código). O post ainda precisa valer sozinho — URL seca não
  gera engajamento previsto.
- Todo post nasce **`MediumRisk`** até o Grok liberar (triagem da 1ª hora) e passa por
  `banger_initial_screen.py` — score de qualidade < 0.4 → distribuição mínima.
- Bloom filter de "já visto": 1 impressão por usuário por post (auto-repost é desperdício).
- Reply é pontuada 0-3 pelo Grok; reply de esforço zero ≈ 0. Existe um filtro nomeado:
  **"Reply Spam Found for lower than 1000 follower bucket"** — nossa faixa. Zero reply
  preguiçosa.

## Pesos de referência (2023 — únicos números publicados)

| Sinal | Peso |
|---|---|
| Reply que o autor engaja de volta | **+75.0** |
| Reply | **+13.5** |
| Profile click → like/reply | +12.0 |
| Conversation click + dwell ≥2min | +10.0 |
| Retweet | +1.0 |
| Like | +0.5 |
| Negative feedback (show less/mute/block) | **−74.0** |
| Report | **−369.0** |
| Autor Premium/verificado | ×4 in-network, ×2 out-of-network |
| Author diversity | 2º post seguido ≈ metade; 3º ≈ esmagado |

## As 15 regras (conta pequena, nicho AI/dev)

1. **Otimize para reply, não para like** — feche com pergunta genuína ou afirmação
   testável (reply 13.5 vs like 0.5).
2. **Responda toda reply, rápido** — reply-que-o-autor-engaja = +75 (150× um like),
   e na 1ª hora conta dobrado (janela de triagem).
3. **30-45 min/dia de replies técnicas** em 5-10 contas grandes do nicho — substância,
   informação nova. Profile click gerado vale +12.
4. **Nunca reply de esforço zero** — bucket de spam específico para <1000 seguidores.
5. **Link pode no post** — mas escreva o insight no texto; o link vai por último.
   (Fallback: link na 1ª reply continua grátis.)
6. **3-5 posts/dia no máximo, espaçados** (manhã/meio-dia/noite) — author-diversity
   decay corta o 2º post seguido pela metade.
7. **Perfil = landing page** — post fixado forte, bio com credenciais. Follow-from-feed
   é o sinal positivo mais forte do modelo 2026; o clique no perfil é o funil.
8. **1 vídeo nativo ≥60-90s/semana** (screen-capture do que está construindo).
   Video Quality View é sinal dedicado; link de YouTube gera zero.
9. **Imagem/diagrama em post técnico denso** — photo expand é ação rastreada;
   embeddings multimodais do Grok "leem" a mídia (≈2× em testes de criadores).
10. **Escreva para dwell** — gancho de 2-4 linhas, depois substância; thread para
    conteúdo profundo.
11. **Faça conteúdo salvável** — share por DM e copy-link são sinais de primeira
    classe: cheat-sheets, benchmarks, configs, "steal this prompt".
12. **Evite gatilhos de negative feedback** — sem engagement bait, all-caps, rage bait,
    >2 hashtags, tag-spam (−74 / report −369; Grox pune tom combativo).
13. **1ª hora é triagem, 24h é a pista** — não delete post "lento" cedo.
14. **Mantenha o Premium** — multiplicadores ×4/×2 no código de 2023; gap prático
    grande medido por criadores em 2026.
15. **Não tente burlar** — pods não enganam predição de conteúdo (Grok lê o post),
    bloom filter mata auto-repost, e o modelo existe exatamente para achar "bangers"
    de contas pequenas.

## Como isso liga no nosso sistema

- O prompt (`src/prompts/x_posts.yml`) já aplica as regras de conteúdo: final
  reply-friendly, insight antes de link, tom construtivo, zero bait.
- O admin (`cargo run --bin admin`) mostra o resumo destas regras e mede o resultado:
  registre métricas dos posts postados e acompanhe ER e views/dia por tema.
- Cadência: o gerador entrega 3 rascunhos/dia — poste espaçado (regra 6) e gaste o
  tempo economizado em replies (regras 2-3), que é onde o algoritmo paga mais.
