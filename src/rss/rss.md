# Documentação Técnica: Módulo de Ingestão de RSS (`src/rss`)

Este módulo é a porta de entrada de todo o ecossistema do robô. Sem ele, a curadoria e os algoritmos avançados de NLP do pipeline morreriam de inanição. Ele é responsável exclusivamente por extrair o XML em formato bruto da internet, interpretá-lo e aplicar delimitações de janela temporal (dias passados).

---

## 1. `fetch.rs` (O Trator de Ingestão HTTP)

### `get_rss_news`
- **Motivo de criação:** Atuar como maestro na orquestração de *Networking*. Em vez de iterar sequencialmente 40 URLs diferentes (o que levaria muito tempo), ele mapeia todas simultaneamente para uso com `join_all`.
- **Quem chama:** `main.rs` (logo após o boot do timer e no início do *loop* principal).
- **Motivo da chamada:** Extrair 100% de todo vetor de notícias aglutinado. 
- **Necessidade (0 a 10):** 10. Ele gerencia o ciclo de vida assíncrono das corotinas `tokio` em uma única variável de ponteiro.
- **Dava pra ser mais simples?:** Sim, poderíamos ignorar `join_all` e o *async/await* e fazer loops `for url in feeds` simples em modo bloqueante (onde ele esperaria um jornal carregar pra dps tentar o próximo). O custo computacional desabaria, mas a ingestão de dados pularia de "3 segundos" para "35 segundos" com timeouts. Com *Futures*/Corrotinas é muito mais produtivo, por isso vale a pena o design atual.

### `fetch_news_from_web`
- **Motivo de criação:** O *Worker* individual. Acessa uma URI estrita, executa a request `GET`, aguarda o corpo do stream HTTP, aciona o interpretador `feed_rs`, retalha as `Entry` XML nativas pro nosso formato proprietário de Struct `ChannelRow` descartando a imensa gordura de campos nativos dos RSS (guid, autores corporativos, meta-tags que não nos interessam).
- **Quem chama:** Módulo interno pela função `get_rss_news`.
- **Motivo da chamada:** Extração de sub-elementos pontuais e sanitização de um jornal por vez.
- **Necessidade (0 a 10):** 10.

---

## 2. `lookback.rs` (Janela de Residente Temporal)

### `lookback_days_for_feed`
- **Motivo de criação:** Solucionar o problema de assimetria dos meios de comunicação. Jornais de Varejo publicam 20 vezes ao dia (a janela tem que ser `1` dia). Blogs técnicos super focados e essenciais como *TrailOfBits* e *Helius* postam uma vez a cada trimestre. Exigir 1 dia deles resultaria na nossa Curadoria deixando passar relatórios sérios.
- **Quem chama:** Módulo interno `fetch_news_from_web`.
- **Motivo da chamada:** Decidir quantos dias subtrair da data `Hoje` como regra de expiração de artigos no XML que estejam sendo escaneados pela request. 
- **Necessidade (0 a 10):** 9. 
- **Dava pra ser mais simples?:** Poderia abolir o cálculo customizado estático das chaves YAML atrelados aos jornais técnicos e puramente estender a busca para "10 dias para *todos* os feeds". Mas isso explodiria a taxa de re-análises diárias que a Curadoria precisaria fazer de sites spam. A abordagem híbrida e delegada pelo `curation.yml` é a mais balanceada possível.
EOF
