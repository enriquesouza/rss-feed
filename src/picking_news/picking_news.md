# Documentação Técnica: Motor de Seleção (`src/picking_news`)

Este documento destrincha o módulo primário de Curadoria do RSS Feed. Ele ocorre logo antes do *Clustering* e é responsável pelo "Filtro Primitivo": define quem é importante (Hacks, Core Devs) e quem é lixo (Previsões de preço genéricas), aplicando o que chamamos de "Score de Notícia" e "Cap de Frequência de Fontes".

---

## 1. `pick_news_for_ai.rs` (O Filtro Mestre)

### `pick_news_for_ai`
- **Motivo de criação:** Selecionar brutalmente o *Top %* das notícias baseadas em slots e regras numéricas restritas, descartando centenas de links menos importantes para não ofuscar o peso de notícias críticas. Promove um enquadramento rígido onde os itens Técnicos/Securança não sofrem concorrência com itens do dia a dia.
- **Quem chama:** `main.rs` (logo após realizar o *fetch* de toda a internet).
- **Motivo da chamada:** Extrair a lista final compacta (geralmente `[~80]` itens) num universo de centenas de novas matérias.
- **Necessidade (0 a 10):** 10. Sem este corte, o fluxo enviaria um lixo aleatório de mercado de criptomoedas, encarecendo a *call* do LLM sem necessidade.
- **Dava pra ser mais simples?:** Sim, poderíamos ignorar completamente limites rígidos pra "Tech" vs "General" e só dar `sort` absoluto. Porém o domínio da base técnica ficaria silenciado em dias de alto volume de publicações idiotas. O agrupamento separado blinda a seriedade do pipeline.

### `add_picked_news`
- **Motivo de criação:** Função utilitária que varre a lista primária populando os slots enquanto valida, no meio do caminho, se a notícia passa no corte mínimo e se aquele portal de *news* específico já não entopiu o fluxo.
- **Quem chama:** `pick_news_for_ai`.
- **Motivo da chamada:** Modularizar as inserções restritivas que previnem lixo no vetor final.
- **Necessidade (0 a 10):** 8. Dava para embutir nativamente num loop `for` no `engine`, mas isolar o método deixa muito mais limpo as checagens atômicas de saturação do `source_map`.

---

## 2. `score_news.rs` (Atribuidor de Peso)

### `score_news`
- **Motivo de criação:** Avaliação pura da semântica. Se conter hack ganha mais, se for `[bullish]` perde, se for portal genérico perde. Gera um Score base.
- **Quem chama:** Usada largamente como avaliador atômico, indiretamente referenciada por `compare_news_priority` e também pelo `Clustering Engine`.
- **Motivo da chamada:** Ponderar friamente o grau de importância humana de uma narrativa via dicionários locais YAML.
- **Necessidade (0 a 10):** 10. A inteligência do curador humano (você) mora nesta matemática de pontuação `+= X` e `-= Y`.
- **Dava pra ser mais simples?:** Difícil. Poderia ser uma LLM Call isolada para cada item (ex: pedir pra IA dar nota 0 a 100), mas ia custar milhares de centavos e demorar horas por depender do tempo de resposta da rede. Essa checagem local via *string/YAML* é infalível e livre de custos.

### `compare_news_priority`
- **Motivo de criação:** Empacotamento pro `sort_by()`, gerando subordinação descendente do score mais denso para o menor.

---

## 3. `check_news.rs` (Censores Semânticos)

### `is_tech_or_security`
- **Motivo de criação:** Censor rigoroso. Diferencia notícias vitais (Atualização de compilador `rust`, invasão do protocolo financeiro via de `smart-contract`, CVEs, relatórios Post-mortem) de coisas normais de blockchain.
- **Quem chama:** `add_picked_news` e `source_limit`.
- **Necessidade (0 a 10):** 10. Essencial para o modo de segurança rígido que isolamos como cota obrigatória pro bot mandar.
- **Dava pra ser mais simples?:** Sim, seria facilmente trocado se tudo caísse de bandeja num Classificador Externo da HuggingFace, mas demandaria latência e GPU rodando na máquina. A abordagem atual via `YAML` resolve e tem O(1).

### `is_low_value`
- **Motivo de criação:** O lixeiro do pipeline. Identifica lixo tático como *"analista aponta a tendência X"*, *"Top investidor misterioso fez X"* etc.
- **Necessidade (0 a 10):** 9. LLMs tem muita dificuldade em ignorar "notícias bombásticas" criadas via *clickbait*. O Rust precisa banir isso com `-120 points` antes do LLM ver o texto.

---

## 4. `limit_news_per_source.rs` (Radares de Domínios)

### `is_busy_source`
- **Motivo de criação:** Mapear domínios que floodam (inundam) o feed do leitor. Nomes como *Cointelegraph*, *Decrypt*, *NewsBTC* soltam 300 posts por dia, criando poluição.
- **Quem chama:** `is_tech_or_security` e `source_limit`.
- **Necessidade (0 a 10):** 8.

### `source_limit`
- **Motivo de criação:** Limitar quantidade contígua de matérias reportadas pelo mesmo jornal pro OpenRouter não fazer resumos baseados no olhar enviesado de um portal só, incentivando diversidade.
- **Regras:** Itens extremamente essenciais/Técnicos tem `cap = 4`, domínios inibidos tem `cap = 3`, portais triviais normais `cap = 2`.
- **Necessidade (0 a 10):** 9. Apenas isso garante diversidade de URL real pra quem lê o sumário matinal.
- **Dava pra ser mais simples?:** Poderia adotar cap igual = `1` para toda e qualquer fonte da internet. Simples, e agressivo, mas portais incrivelmente bons (ex: RustLang Blog) ficariam limitadíssimos mesmo para eventos épicos. Esse cap inteligente condicional de acordo com a credibilidade/urgência valeu cada linha de código.
EOF
