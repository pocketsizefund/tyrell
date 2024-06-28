use serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claude35Sonnet;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Model {
    #[serde(rename = "claude")]
    Claude35Sonnet,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Chat {
    pub model: Model,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_creation() {
        let chat = Chat {
            model: Model::Claude35Sonnet,
        };
        assert_eq!(chat.model, Model::Claude35Sonnet);
    }

    #[test]
    fn test_chat_equality() {
        let first_chat = Chat {
            model: Model::Claude35Sonnet,
        };

        let second_chat = Chat {
            model: Model::Claude35Sonnet,
        };

        assert_eq!(first_chat, second_chat);
    }
}
