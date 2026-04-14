# Documentação Técnica: Motor de Clustering (`src/clustering`)

Este documento destrincha o núcleo do algoritmo de agrupamento de notícias do RSS Feed. Este módulo é responsável por pegar centenas de requisições isoladas, decifrar quais falam sobre o mesmo evento, aglutiná-las em "Clusters" (tópicos), pontuá-las rigorosamente para a edição, e descartar o lixo excedente.

---

## 1. `engine.rs` (O Coração do Motor)

### `cluster_news_for_llm`
- **Motivo de criação:** Orquestrar todo o pipeline. Ele itera o array de notícias e agrupa as que falam da mesma história para poupar tempo e contexto do LLM.
- **Quem chama:** `main.rs` (no loop principal de orquestração).
- **Motivo da chamada:** Converter uma lista plana de `ChannelRow` em uma lista filtrada e agrupada de `TopicCluster`.
- **Necessidade (0 a 10):** 10. Ele é a espinha dorsal de todo este módulo.
- **Dava pra ser mais simples?:** Sim. Poderíamos não agrupar nada e mandar todas as notícias cruas pro LLM e pedir pra ele agrupar. O problema: o limite de tokens do LLM iria estourar, o custo financeiro explodir, e ele sofreria de "alucinação" grave com 150 links ao mesmo tempo. 

### `should_merge_topic_cluster`
- **Motivo de criação:** Decidir, em tempo real, se a notícia `X` pertence ao grupo de notícias `Y`.
- **Quem chama:** Módulo interno `cluster_news_for_llm`.
- **Motivo da chamada:** Decidir se funde o dado no cluster já existente com uma resposta Booleana.
- **Necessidade (0 a 10):** 10. Se a junção falhar, uma mesma história aparecerá 4 vezes no canal do Telegram pra irritar o leitor.
- **Dava pra ser mais simples?:** Sim. Substituir toda a lógica artesanal do Rust de "vetor de palavras e interseção" por *Vector Embeddings* de IA (usar NLP, gerar vetor do título usando API externa e comparar similaridade por cosseno). Mas esse cálculo atual usando Rust nativo consome `0 MB` de banda extra e é brutalmente computado em nano-segundos.

### `merge_topic_cluster`
- **Motivo de criação:** Anexar mutuamente os tokens, tags e os artigos.
- **Quem chama:** Módulo interno `cluster_news_for_llm`.
- **Motivo da chamada:** Atualizar os metadados do Cluster em que a notícia foi admitida, acumulando "peso" de tokens pro grupo absorver suas palavras chave.
- **Necessidade (0 a 10):** 8.
- **Dava pra ser mais simples?:** Em vez de iterar para fundir tokens garantindo formato único, daria pra simplesmente engolir todos (mesmo duplicados) e usar `BTreeSet` pro Rust cuidar de remoção de repetidos magicamente, reduzindo laços for manuais.

---

## 2. `overlap.rs` (A Matemática de Confluência)

### `same_story`
- **Motivo de criação:** Burlar cálculos custosos e admitir junção logo de cara se os dois jornalistas usaram o título/URL literalmente idênticos.
- **Quem chama:** `should_merge_topic_cluster`.
- **Motivo da chamada:** Fast-track. Cortar processamento.
- **Necessidade (0 a 10):** 9. Previne redundância boba e garante `true` sem gastar CPU pra cruzamentos semânticos falhos caso os links coincidam.
- **Dava pra ser mais simples?:** Impossível ser mais simples (é um `==`).

### `signature_overlap`
- **Motivo de criação:** Calcular a interseção bruta. Dado `[btc, sobe]` e `[btc, despenca]`, o overlap é 1 (`btc`).
- **Quem chama:** `should_merge_topic_cluster`.
- **Motivo da chamada:** Aferir quantidade quantitativa de palavras raiz em comum para autorizar o agrupamento.
- **Necessidade (0 a 10):** 10.
- **Dava pra ser mais simples?:** Poderia ser feito em 1 linha de código usando métodos nativos de `.intersection()` da estrutura `HashSet` do Rust puro.

### `has_specific_signature_overlap`
- **Motivo de criação:** Imunizar o bot. Uma matéria "*Mercado cripto cai*" não deve ser inserida num bucket de Cripto junto da matéria "*Ações da bolsa cripto*" apenas porque cruzou a palavra genérica "cripto". Essa funçao barra *matches* apenas sobre palavras genéricas.
- **Quem chama:** `should_merge_topic_cluster`.
- **Motivo da chamada:** Segurança garantidora. Desempata overlap perigoso de `1` interseção.
- **Necessidade (0 a 10):** 9. Diferenciador crítico perante clustering de algoritmos burros.
- **Dava pra ser mais simples?:** Em vez de checar contra um arquivo e dicionário, poderia ser aplicado calculo TF-IDF universal em tempo real, onde as palavras que chegam demais no Feed são automaticamente punidas pela própria matemática sem `if`. Mas implementar o cálculo num script leve seria trabalhoso.

---

## 3. `scoring.rs` (Garantindo Qualidade)

### `compare_topic_cluster_priority`
- **Motivo de criação:** Atuar no método `.sort_by()` do Rust. Ele que reordena tudo antes do truncamento (cap).
- **Quem chama:** `cluster_news_for_llm`.
- **Motivo da chamada:** Empurrar no vetor os hacks/acidentes e coisas cruéis pra cima (Top 1) na fila antes de jogar fora o restante.
- **Necessidade (0 a 10):** 10. É isso que define quem vive e quem morre se 100 matérias surgem pro limite do LLM.
- **Dava pra ser mais simples?:** É um Sort canônico. Essencial.

### `topic_cluster_priority`
- **Motivo de criação:** Definir nota numérica absoluta do tópico. Bônus por diversidade (+500 pra *Hack* e vulnerabilidade; +50 por cada site diferente discutindo isso, etc.).
- **Quem chama:** `compare_topic_cluster_priority`.
- **Motivo da chamada:** Base do cálculo final de Rank.
- **Necessidade (0 a 10):** 10. Representa a "regra de negócio editorial" isolada num ponto.

### `newest_cluster_datetime` / `distinct_source_count`
- **Motivo de criação:** Desempates finos de scoring. Valorizam clusters que tem pelo menos uma notícia "ainda fresca", e dizem em quantos sub-hospedeiros isso aparece.
- **Necessidade (0 a 10):** 6. 
- **Dava pra ser mais simples?:** Empates poderiam ignorar a data e irem por nome cego decrescente, mas valorizar frescor é desejável em robô de Daily Morning/Daily Evening. O loop interno de domains poderia ser ignorado simplesmente contando o `.len()` da propriedade de links mas seria fraudável por 2 domínios da `coindesk`.

---

## 4. `signatures.rs` (Extração em Texto Bruto NLP)

### `topic_signature`
- **Motivo de criação:** Limpa o "O Cão do Ethereum latiu para os mineradores de Rust" em `["cao", "eth", "latiu", "minerador", "rust"]`.
- **Quem chama:** `cluster_news_for_llm`.
- **Motivo da chamada:** Extrair tokens rastreáveis usando NLP artesanal.
- **Necessidade (0 a 10):** 10. Base inteira que desabilita erro ortográfico.
- **Dava pra ser mais simples?:** Importar uma biblioteca GIGANTE de *Natural Language Toolkit* (NLTK/Tokenizers) mas engessaria o binário brutalmente pra fazer algo incrivelmente estático ("limpeza manual de caracteres e dicionário estático").

*(Helpers auxiliares:)* `normalize_topic_text`, `canonical_topic_token`, `is_topic_stopword`, `is_generic_cluster_token` alimentam essa mecânica acima e apenas desviam via Config YAML para checagem em mapa de O(1), e recebem todas nota 9/10 pois foram essenciais para a redução drástica das linhas Hardcoded que migrei na fase anterior de "YAML-ficação".

---

## 5. `buckets.rs` e `formatter.rs` (Estruturação e Output)

### `infer_editorial_bucket` e `topic_bucket_cap`
- **Motivo de criação:** Direcionar as gavetas macro que o LLM usam em seus delimitadores de Prompt. O cap impede que os "top 18 eventos pro LLM não sejam 18 reportagens sobre a alta maluca do Bitcoin", barrando, por exemplo, um limite de 10 na gaveta Genérica.
- **Necessidade (0 a 10):** 8.

### `format_topic_cluster_for_llm`
- **Motivo de criação:** Serializar a classe/struct polimórfica cheia de métodos do Rust de volta para o texto simples que o OpenRouter entende (uma enorme String de XML/HTML de alerta).
- **Quem chama:** `main.rs`.
- **Necessidade (0 a 10):** 10. Todo LLM respira Markdown/String, a representação binária não quer dizer nada na payload.
- **Dava pra ser mais simples?:** Dificilmente. Poderia usar macros de serialize `serde_json`, mas o LLM é treinado para ler marcações ricas como texto XML `[Fontes: 4]` então a montagem `format!` atual é superior.
