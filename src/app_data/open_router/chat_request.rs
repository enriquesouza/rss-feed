use serde::{Deserialize, Serialize};

use super::{
    ChatMessage, Plugin, ProviderPreferences, ResponseFormat, StreamOptions, ToolDefinition,
    UsageConfig,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,

    /// Fallback routing: uma lista de modelos alternativos caso o modelo principal caia ou falhe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,

    /// Preferências do provedor: permite negar treinamento de dados (data_collection: "deny"),
    /// ordenar provedores por preço/latência ou exigir que suportem ferramentas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<ProviderPreferences>,

    /// Formato da resposta: obriga o modelo a retornar um JSON estruturado (ex: json_schema)
    /// em vez de texto livre.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// Condições de parada: lista de palavras ou tokens que, se gerados, fazem o modelo
    /// parar imediatamente de escrever.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// Força o modelo a usar uma ferramenta específica ou deixa ele escolher ("auto" ou "none").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<serde_json::Value>,

    /// Se true, permite que o modelo chame múltiplas ferramentas ao mesmo tempo (ex: buscar 3 links de uma vez).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// Plugins OpenRouter: ativa extensões de middleware, como "response-healing" para consertar
    /// JSONs quebrados, ou pesquisa na web (web search).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<Plugin>>,

    /// Configurações de raciocínio (reasoning): permite ativar ou ajustar o nível de "pensamento"
    /// interno para modelos que suportam chain-of-thought (ex: modelos r1, o1, o3-mini).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<serde_json::Value>,

    /// Identificador do usuário (anonimizado): usado pelo OpenRouter para monitorar abusos,
    /// rastrear origens e aplicar moderação de taxa por usuário final.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,

    /// Nucleus sampling (Top P): altera a criatividade selecionando apenas os tokens que
    /// somam probabilidade P (ex: 0.9 = foca nos 90% mais prováveis).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Top K sampling: limita a escolha do próximo token aos K tokens mais prováveis
    /// (ex: 40 = ignora tokens muito fora do comum).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// Penalidade de repetição: penaliza o modelo por repetir os mesmos tokens ou frases,
    /// ajudando a evitar loops ou texto redundante (ex: 1.1 ou 1.2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f32>,

    /// Min P sampling: variante do Top P, define um limite mínimo relativo de probabilidade
    /// que um token deve ter em relação ao mais provável.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_a: Option<f32>,

    /// Semente determinística (Seed): tenta forçar o modelo a gerar a mesma resposta se
    /// você passar a mesma entrada e o mesmo número de seed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
}
