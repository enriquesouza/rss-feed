# Documentação Técnica: Motor de Agrupamento (`src/grouping_news`)

Este documento destrincha o núcleo do algoritmo de agrupamento de notícias do RSS Feed. Este módulo é responsável por pegar centenas de requisições isoladas, decifrar quais falam sobre o mesmo evento, aglutiná-las em "Clusters" (tópicos), pontuá-las rigorosamente para a edição, e descartar o lixo excedente.

---

## 1. `group_related_news.rs` (O Coração do Motor)

### `group_related_news`
- **Motivo de criação:** Orquestrar todo o pipeline. Ele itera o array de notícias e agrupa as que falam da mesma história para poupar tempo e contexto do LLM.
- **Quem chama:** `main.rs` (no loop principal de orquestração).
- **Motivo da chamada:** Converter uma lista plana de `NewsItem` em uma lista filtrada e agrupada de `StoryGroup`.
- **Necessidade (0 a 10):** 10. Ele é a espinha dorsal de todo este módulo.
- **Dava pra ser mais simples?:** Sim. Poderíamos não agrupar nada e mandar todas as notícias cruas pro LLM e pedir pra ele agrupar. O problema: o limite de tokens do LLM iria estourar, o custo financeiro explodir, e ele sofreria de "alucinação" grave com 150 links ao mesmo tempo. 

### `can_join_group`
- **Motivo de criação:** Decidir, em tempo real, se a notícia `X` pertence ao grupo de notícias `Y`.
- **Quem chama:** Módulo interno `group_related_news`.
- **Motivo da chamada:** Decidir se funde o dado no cluster já existente com uma resposta Booleana.
- **Necessidade (0 a 10):** 10. Se a junção falhar, uma mesma história aparecerá 4 vezes no canal do Telegram pra irritar o leitor.
- **Dava pra ser mais simples?:** Sim. Substituir toda a lógica artesanal do Rust de "vetor de palavras e interseção" por *Vector Embeddings* de IA (usar NLP, gerar vetor do título usando API externa e comparar similaridade por cosseno). Mas esse cálculo atual usando Rust nativo consome `0 MB` de banda extra e é brutalmente computado em nano-segundos.

### `add_to_group`
- **Motivo de criação:** Anexar mutuamente os tokens, tags e os artigos.
- **Quem chama:** Módulo interno `group_related_news`.
- **Motivo da chamada:** Atualizar os metadados do Cluster em que a notícia foi admitida, acumulando "peso" de tokens pro grupo absorver suas palavras chave.
- **Necessidade (0 a 10):** 8.
- **Dava pra ser mais simples?:** Em vez de iterar para fundir tokens garantindo formato único, daria pra simplesmente engolir todos (mesmo duplicados) e usar `BTreeSet` pro Rust cuidar de remoção de repetidos magicamente, reduzindo laços for manuais.

---

## 2. `group_checks.rs` (A Matemática de Confluência)

### `same_story`
- **Motivo de criação:** Burlar cálculos custosos e admitir junção logo de cara se os dois jornalistas usaram o título/URL literalmente idênticos.
- **Quem chama:** `can_join_group`.
- **Motivo da chamada:** Fast-track. Cortar processamento.
- **Necessidade (0 a 10):** 9. Previne redundância boba e garante `true` sem gastar CPU pra cruzamentos semânticos falhos caso os links coincidam.
- **Dava pra ser mais simples?:** Impossível ser mais simples (é um `==`).

### `shared_word_count`
- **Motivo de criação:** Calcular a interseção bruta. Dado `[btc, sobe]` e `[btc, despenca]`, o overlap é 1 (`btc`).
- **Quem chama:** `can_join_group`.
- **Motivo da chamada:** Aferir quantidade quantitativa de palavras raiz em comum para autorizar o agrupamento.
- **Necessidade (0 a 10):** 10.
- **Dava pra ser mais simples?:** Poderia ser feito em 1 linha de código usando métodos nativos de `.intersection()` da estrutura `HashSet` do Rust puro.

### `has_clear_shared_word`
- **Motivo de criação:** Imunizar o bot. Uma matéria "*Mercado cripto cai*" não deve ser inserida num grupo de Cripto junto da matéria "*Ações da bolsa cripto*" apenas porque cruzou a palavra genérica "cripto". Essa função barra *matches* apenas sobre palavras genéricas.
- **Quem chama:** `can_join_group`.
- **Motivo da chamada:** Segurança garantidora. Desempata overlap perigoso de `1` interseção.
- **Necessidade (0 a 10):** 9. Diferenciador crítico perante agrupamento de algoritmos burros.
- **Dava pra ser mais simples?:** Em vez de checar contra um arquivo e dicionário, poderia ser aplicado calculo TF-IDF universal em tempo real, onde as palavras que chegam demais no Feed são automaticamente punidas pela própria matemática sem `if`. Mas implementar o cálculo num script leve seria trabalhoso.

---

## 3. `score_news_group.rs` (Garantindo Qualidade)

### `score_group`
- **Motivo de criação:** Definir nota numérica absoluta do tópico. Bônus por diversidade (+500 pra *Hack* e vulnerabilidade; +50 por cada site diferente discutindo isso, etc.).
- **Quem chama:** `group_related_news`.
- **Motivo da chamada:** Base do cálculo final de Rank.
- **Necessidade (0 a 10):** 10. Representa a "regra de negócio editorial" isolada num ponto.

### `count_sources`
- **Motivo de criação:** Desempates finos de scoring. Valorizam clusters que tem pelo menos uma notícia "ainda fresca", e dizem em quantos sub-hospedeiros isso aparece.
- **Necessidade (0 a 10):** 6. 
- **Dava pra ser mais simples?:** Empates poderiam ignorar a data e irem por nome cego decrescente, mas valorizar frescor é desejável em robô de Daily Morning/Daily Evening. O loop interno de domains poderia ser ignorado simplesmente contando o `.len()` da propriedade de links mas seria fraudável por 2 domínios da `coindesk`.

---

## 4. `find_topic_words.rs` (Extração em Texto Bruto NLP)

### `find_words`
- **Motivo de criação:** Limpa o "O Cão do Ethereum latiu para os mineradores de Rust" em `["cao", "eth", "latiu", "minerador", "rust"]`.
- **Quem chama:** `group_related_news`.
- **Motivo da chamada:** Extrair tokens rastreáveis usando NLP artesanal.
- **Necessidade (0 a 10):** 10. Base inteira que desabilita erro ortográfico.
- **Dava pra ser mais simples?:** Importar uma biblioteca GIGANTE de *Natural Language Toolkit* (NLTK/Tokenizers) mas engessaria o binário brutalmente pra fazer algo incrivelmente estático ("limpeza manual de caracteres e dicionário estático").

*(Helpers auxiliares:)* `clean_words_text`, `base_word`, `is_ignored_word`, `is_common_word` alimentam essa mecânica acima e apenas desviam via Config YAML para checagem em mapa de O(1), e recebem todas nota 9/10 pois foram essenciais para a redução drástica das linhas Hardcoded que migrei na fase anterior de "YAML-ficação".

---

## 5. `find_group_name.rs` e `format_group_for_ai.rs` (Estruturação e Output)

### `group_name` e `max_groups`
- **Motivo de criação:** Direcionar as gavetas macro que o LLM usam em seus delimitadores de Prompt. O cap impede que os "top 18 eventos pro LLM não sejam 18 reportagens sobre a alta maluca do Bitcoin", barrando, por exemplo, um limite de 10 na gaveta Genérica.
- **Necessidade (0 a 10):** 8.

### `format_group_for_ai`
- **Motivo de criação:** Serializar a classe/struct polimórfica cheia de métodos do Rust de volta para o texto simples que o OpenRouter entende (uma enorme String de XML/HTML de alerta).
- **Quem chama:** `main.rs`.
- **Necessidade (0 a 10):** 10. Todo LLM respira Markdown/String, a representação binária não quer dizer nada na payload.
- **Dava pra ser mais simples?:** Dificilmente. Poderia usar macros de serialize `serde_json`, mas o LLM é treinado para ler marcações ricas como texto XML `[Fontes: 4]` então a montagem `format!` atual é superior.
