use serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ClaudeModel {
    Config {
        name: &'static str,
        max_tokens: usize,
    },
}

impl ClaudeModel {
    const SONNET35: Self = Self::Config {
        name: "claude-3-5-sonnet-20240620",
        max_tokens: 4096,
    };

    const OPUS3: Self = Self::Config {
        name: "claude-3-opus-20240229",
        max_tokens: 4096,
    };

    const SONNET3: Self = Self::Config {
        name: "claude-3-sonnet-20240229",
        max_tokens: 4096,
    };

    const HAIKU3: Self = Self::Config {
        name: "claude-3-haiku-20240307",
        max_tokens: 4096,
    };
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Chat {
    pub model: ClaudeModel,
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_chat_creation() {
        let chat = Chat {
            model: ClaudeModel::SONNET35,
        };
        assert_eq!(chat.model, ClaudeModel::SONNET35);
    }

    #[test]
    fn test_chat_equality() {
        let first_chat = Chat {
            model: ClaudeModel::SONNET35,
        };

        let second_chat = Chat {
            model: ClaudeModel::SONNET35,
        };

        assert_eq!(first_chat, second_chat);
    }

    #[test]
    fn test_chat_serialization() {
        let chat = Chat {
            model: ClaudeModel::SONNET35,
        };

        let chat_serialized = match serde_json::to_string(&chat) {
            Ok(result) => result,
            Err(err) => {
                println!("{:?}", err);
                String::new()
            }
        };

        let pattern = Regex::new(r"\n|\s+").unwrap();

        let expected = r#"{
          "model": {
            "Config": {
              "name": "claude-3-5-sonnet-20240620",
              "max_tokens": 4096
            }
          }
        }"#;
        let expected = pattern.replace_all(expected, "").to_string();

        assert_eq!(chat_serialized, expected);
    }
}
