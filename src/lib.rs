//! Claude Rust SDK
//!
//! This SDK provides a way to interact with the Claude API using a simple builder pattern.

use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

/// Available Claude Models.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Model {
    #[serde(rename = "claude-3-5-sonnet-20240620")]
    Sonnet35,
    #[serde(rename = "claude-3-opus-20240229")]
    Opus3,
    #[serde(rename = "claude-3-sonnet-20240229")]
    Sonnet3,
    #[serde(rename = "claude-3-haiku-20240307")]
    Haiku3,
}

/// Represents the role of a message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

/// Represents the source of an image in a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

/// Represents the type of content in a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentType {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { source: ImageSource },
    #[serde(rename = "tool_use")]
    ToolUse(ToolUse),
    #[serde(rename = "tool_result")]
    ToolResult(ToolResult),
}

/// Represents a message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Vec<ContentType>,
}

/// Represents the JSON-Schema input
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputSchema {
    #[serde(rename = "type")]
    schema_type: String,
    properties: Value,
    required: Vec<String>,
}

/// Represents a tool that can be used by the model.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: InputSchema,
}

/// Trait for creating a tool with a struct-based input schema.
pub trait ToolBuilder: JsonSchema {
    fn name() -> &'static str;
    fn description() -> Option<&'static str>;
}

impl Tool {
    /// Creates a new Tool with a struct-based input schema.
    pub fn new<T: ToolBuilder>() -> Self {
        let schema = schemars::schema_for!(T);
        let schema = schema.schema.object.unwrap();

        let properties = serde_json::to_value(schema.properties).unwrap();
        let required = schema.required.into_iter().collect();

        Tool {
            name: T::name().to_string(),
            description: T::description().map(|s| s.to_string()),
            input_schema: InputSchema {
                schema_type: "object".to_string(),
                properties,
                required,
            },
        }
    }
}

/// Represents the model's use of a tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUse {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub id: String,
    pub name: String,
    pub input: Value,
}

/// Represents the result of a tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    #[serde(rename = "type")]
    pub result_type: String,
    pub tool_use_id: String,
    pub content: String,
}

/// Represents how the model should use the provided tools.
#[derive(Debug, Clone, Deserialize)]
pub enum ToolChoice {
    None,
    Auto {
        disable_parallel_tool_use: Option<bool>,
    },
    Any {
        disable_parallel_tool_use: Option<bool>,
    },
    Specific {
        name: String,
        disable_parallel_tool_use: Option<bool>,
    },
}

impl Serialize for ToolChoice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ToolChoice::None => {
                let json = json!({});
                json.serialize(serializer)
            }
            ToolChoice::Auto {
                disable_parallel_tool_use,
            } => {
                let mut json = json!({
                    "type": "auto"
                });
                if let Some(disable) = disable_parallel_tool_use {
                    json["disable_parallel_tool_use"] = json!(disable);
                }
                json.serialize(serializer)
            }
            ToolChoice::Any {
                disable_parallel_tool_use,
            } => {
                let mut json = json!({
                    "type": "any"
                });
                if let Some(disable) = disable_parallel_tool_use {
                    json["disable_parallel_tool_use"] = json!(disable);
                }
                json.serialize(serializer)
            }
            ToolChoice::Specific {
                name,
                disable_parallel_tool_use,
            } => {
                let mut json = json!({
                    "type": "tool",
                    "name": name
                });
                if let Some(disable) = disable_parallel_tool_use {
                    json["disable_parallel_tool_use"] = json!(disable);
                }
                json.serialize(serializer)
            }
        }
    }
}

/// Represents the usage statistics for an API call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

/// Represents the stopping reason in the API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "snake_case")]
pub enum StopReason {
    MaxTokens,
    ToolUse,
}

/// Represents the response from the Claude API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: Role,
    pub content: Vec<ContentType>,
    pub model: Model,
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}
/* {"id": "msg_01RhY4TxxRHM2b3N81ijdJms",
"role": "assistant",
"content": [
    {
      "type": "tool_use",
      "id": "toolu_01CQ1Yq17jrrMpF5uiAMt4bU",
      "name": "extract_super_bowl_info",
      "input": {
        "winner": "Green Bay Packers",
        "winner_score": 31, "loser": "Miami Dolphins",
        "loser_score": 10, "year": 1982
      }
    }
  ],
} */

// {
//     "type": "message",
//     "content": [
//       {"type": "text",
//        "text": "The 16th President of the United States was Abraham Lincoln. He served as the nation's president from March 4, 1861, until his assassination for"
//       }
//     ],
// }
/// Builder for creating a request to the Claude API.
#[derive(Debug, Clone, Default)]
pub struct ClaudeRequestBuilder {
    pub model: Option<Model>,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub metadata: Option<HashMap<String, String>>,
    pub stop_sequences: Option<Vec<String>>,
    pub stream: Option<bool>,
    pub system: Option<String>,
    pub temperature: Option<f32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
    pub tools: Option<Vec<Tool>>,
    pub tool_choice: Option<ToolChoice>,
}

impl ClaudeRequestBuilder {
    /// Creates a new ClaudeRequestBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the model for the request.
    pub fn model(mut self, model: Model) -> Self {
        self.model = Some(model);
        self
    }

    /// Adds a message to the request.
    pub fn add_message(mut self, role: Role, content: Vec<ContentType>) -> Self {
        self.messages.push(Message { role, content });
        self
    }

    /// Sets the maximum number of tokens to generate.
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Adds metadata to the request.
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Sets custom stop sequences for the request.
    pub fn stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    /// Sets whether to stream the response.
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Sets the system prompt for the request.
    pub fn system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Sets the temperature for the request.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the top_k parameter for the request.
    pub fn top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Sets the top_p parameter for the request.
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Adds tool definitions to the request.
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Sets how the model should use the provided tools.
    pub fn tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }

    /// Builds the final request object.
    pub fn build(self) -> Result<ClaudeRequest, String> {
        if self.model.is_none() {
            return Err("Model must be specified".to_string());
        }
        if self.messages.is_empty() {
            return Err("At least one message must be added".to_string());
        }
        if self.max_tokens.is_none() {
            return Err("Max tokens must be specified".to_string());
        }

        Ok(ClaudeRequest {
            model: self.model.unwrap(),
            messages: self.messages,
            max_tokens: self.max_tokens.unwrap(),
            metadata: self.metadata,
            stop_sequences: self.stop_sequences,
            stream: self.stream,
            system: self.system,
            temperature: self.temperature,
            top_k: self.top_k,
            top_p: self.top_p,
            tools: self.tools,
            tool_choice: self.tool_choice,
        })
    }
}

/// Represents a complete request to the Claude API.
#[derive(Debug, Clone, Deserialize)]
pub struct ClaudeRequest {
    pub model: Model,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
}

impl Serialize for ClaudeRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ClaudeRequest", 13)?;
        state.serialize_field("model", &self.model)?;
        state.serialize_field("messages", &self.messages)?;
        state.serialize_field("max_tokens", &self.max_tokens)?;
        if let Some(ref metadata) = self.metadata {
            state.serialize_field("metadata", metadata)?;
        }
        if let Some(ref stop_sequences) = self.stop_sequences {
            state.serialize_field("stop_sequences", stop_sequences)?;
        }
        if let Some(stream) = self.stream {
            state.serialize_field("stream", &stream)?;
        }
        if let Some(ref system) = self.system {
            state.serialize_field("system", system)?;
        }
        if let Some(temperature) = self.temperature {
            state.serialize_field("temperature", &temperature)?;
        }
        if let Some(top_k) = self.top_k {
            state.serialize_field("top_k", &top_k)?;
        }
        if let Some(top_p) = self.top_p {
            state.serialize_field("top_p", &top_p)?;
        }
        if let Some(ref tools) = self.tools {
            state.serialize_field("tools", tools)?;
        }
        if let Some(ref tool_choice) = self.tool_choice {
            state.serialize_field("tool_choice", tool_choice)?;
        }
        state.end()
    }
}

impl ClaudeRequest {
    /// Creates a new ClaudeRequestBuilder to start building a request.
    pub fn builder() -> ClaudeRequestBuilder {
        ClaudeRequestBuilder::new()
    }

    /// Invoke the Claude Chat API.
    pub async fn call(&self) -> Result<String> {
        let api_key = std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        headers.insert("x-api-key", HeaderValue::from_str(&api_key)?);

        let body = serde_json::to_string(&self)?;

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();

        let text = response
            .text()
            .await
            .context("Failed to get response text")?;

        if status.is_success() {
            Ok(text)
        } else {
            Err(anyhow::anyhow!(
                "API request failed with status: {}. Error: {}",
                status,
                text
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use schemars::JsonSchema;

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    struct GetStockPrice {
        ticker: String,
    }

    impl ToolBuilder for GetStockPrice {
        fn name() -> &'static str {
            "get_stock_price"
        }

        fn description() -> Option<&'static str> {
            Some("Get the current stock price for a given ticker symbol.")
        }
    }

    #[test]
    fn test_request_builder() {
        let stock_price_tool = Tool::new::<GetStockPrice>();

        let request = ClaudeRequest::builder()
            .model(Model::Opus3)
            .add_message(
                Role::User,
                vec![ContentType::Text {
                    text: "What's the current stock price of Apple?".to_string(),
                }],
            )
            .max_tokens(100)
            .temperature(0.7)
            .tools(vec![stock_price_tool])
            .tool_choice(ToolChoice::Auto {
                disable_parallel_tool_use: Some(true),
            })
            .build()
            .expect("Failed to build request");

        assert_eq!(request.model, Model::Opus3);
        assert_eq!(request.max_tokens, 100);
        assert_eq!(request.temperature, Some(0.7));
        assert!(request.tools.is_some());
        assert!(matches!(
            request.tool_choice,
            Some(ToolChoice::Auto {
                disable_parallel_tool_use: Some(true)
            })
        ));
    }

    #[test]
    fn test_minimal_valid_request() {
        let request = ClaudeRequest::builder()
            .model(Model::Opus3)
            .add_message(
                Role::User,
                vec![ContentType::Text {
                    text: "Hello".to_string(),
                }],
            )
            .max_tokens(10)
            .build();

        assert!(request.is_ok());
    }

    #[test]
    fn test_request_with_all_params() {
        let request = ClaudeRequest::builder()
            .model(Model::Haiku3)
            .add_message(
                Role::User,
                vec![ContentType::Text {
                    text: "Hello".to_string(),
                }],
            )
            .max_tokens(10)
            .temperature(0.7)
            .top_k(10)
            .top_p(0.9)
            .stream(true)
            .system("You are a helpful assistant.")
            .stop_sequences(vec!["STOP".to_string()])
            .metadata(std::collections::HashMap::new())
            .build();

        assert!(request.is_ok());
    }

    #[test]
    fn test_multiple_messages() {
        let request = ClaudeRequest::builder()
            .model(Model::Sonnet35)
            .add_message(
                Role::User,
                vec![ContentType::Text {
                    text: "Hello".to_string(),
                }],
            )
            .add_message(
                Role::Assistant,
                vec![ContentType::Text {
                    text: "Hi there!".to_string(),
                }],
            )
            .add_message(
                Role::User,
                vec![ContentType::Text {
                    text: "How are you?".to_string(),
                }],
            )
            .max_tokens(10)
            .build();

        assert!(request.is_ok());
        assert_eq!(request.unwrap().messages.len(), 3);
    }

    #[test]
    fn test_metadata() {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("key".to_string(), "value".to_string());

        let request = ClaudeRequest::builder()
            .model(Model::Haiku3)
            .add_message(
                Role::User,
                vec![ContentType::Text {
                    text: "Hello".to_string(),
                }],
            )
            .max_tokens(10)
            .metadata(metadata.clone())
            .build();

        assert!(request.is_ok());
        assert_eq!(request.unwrap().metadata, Some(metadata));
    }

    #[test]
    fn test_create_struct_tool() {
        #[derive(Debug, Serialize, Deserialize, JsonSchema)]
        struct Calculator {
            operation: String,
            operands: Vec<f64>,
        }

        impl ToolBuilder for Calculator {
            fn name() -> &'static str {
                "calculator"
            }

            fn description() -> Option<&'static str> {
                Some("A simple calculator")
            }
        }

        let tool = Tool::new::<Calculator>();

        assert_eq!(tool.name, "calculator");
        assert_eq!(tool.description, Some("A simple calculator".to_string()));
    }

    #[test]
    fn test_add_tools_to_request() {
        #[derive(Debug, Serialize, Deserialize, JsonSchema)]
        struct Calculator {
            operation: String,
            operands: Vec<f64>,
        }

        impl ToolBuilder for Calculator {
            fn name() -> &'static str {
                "calculator"
            }

            fn description() -> Option<&'static str> {
                Some("A simple calculator")
            }
        }

        let tool = Tool::new::<Calculator>();

        let request = ClaudeRequest::builder()
            .model(Model::Opus3)
            .add_message(
                Role::User,
                vec![ContentType::Text {
                    text: "Hello".to_string(),
                }],
            )
            .max_tokens(10)
            .tools(vec![tool])
            .build();

        assert!(request.is_ok());
        assert!(request.unwrap().tools.is_some());
    }

    #[test]
    fn test_tool_choice_options() {
        let request = ClaudeRequest::builder()
            .model(Model::Sonnet3)
            .add_message(
                Role::User,
                vec![ContentType::Text {
                    text: "Hello".to_string(),
                }],
            )
            .max_tokens(10)
            .tool_choice(ToolChoice::Auto {
                disable_parallel_tool_use: Some(true),
            })
            .build();

        assert!(request.is_ok());
        assert!(matches!(
            request.unwrap().tool_choice,
            Some(ToolChoice::Auto {
                disable_parallel_tool_use: Some(true)
            })
        ));
    }

    #[test]
    fn test_tool_use_request_body_valid() -> Result<()> {
        let chat = ClaudeRequest::builder()
            .model(Model::Sonnet35)
            .max_tokens(200)
            .add_message(
                Role::Assistant,
                vec![ContentType::Text {
                    text: "You're an NFL expert extract the game info.".to_string(),
                }],
            )
            .add_message(
                Role::User,
                vec![ContentType::Text {
                text: "The Green Bay Packers beat the Miami Dolphins in the 1982 Super Bowl 31-10."
                    .to_string(),
            }],
            )
            .build();

        let expected = serde_json::json!({
             "model": "claude-3-opus-20240229",
             "messages": [
               {
                 "role": "assistant",
                 "content": [
                   "You're an NFL expert extract the game info."
                 ]
               },
               {
                 "role": "user",
                 "content": [
                   {
                     "type": "text",
                     "text": "The Green Bay Packers beat the Miami Dolphins in the 1982 Super Bowl 31-10."
                   }
                 ]
               }
             ],
             "max_tokens":200,
             "tools": [
               {
                 "name": "extract_super_bowl_info",
                 "description": "Extract Super Bowl information from text",
                   "input_schema": {
                     "type": "object",
                     "properties": {
                       "loser": {
                         "type": "string"
                       },
                       "loser_score": {
                         "format": "uint8",
                         "minimum": 0.0,
                         "type": "integer"
                       },
                       "total_points_scored": {
                         "format": "uint8",
                         "minimum": 0.0,
                         "type": ["integer","null"]
                       },
                       "winner": {
                         "type": "string"
                       },
                       "winner_score": {
                         "format": "uint8",
                         "minimum": 0.0,
                         "type": "integer"
                       },
                       "year": {
                         "format": "uint16",
                         "minimum": 0.0,
                         "type": "integer"
                       }
                     },
                     "required": [
                       "loser",
                       "loser_score",
                       "winner",
                       "winner_score",
                       "year"
                     ]
                   }
                 }
              ],
            "tool_choice": {
              "disable_parallel_tool_use": false,
              "name": "extract_super_bowl_info",
              "type": "tool"
            }
        });

        // let expected: serde_json::Value = serde_json::from_str(expected)?;

        Ok(())
    }
}
