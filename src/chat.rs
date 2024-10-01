use serde;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Role {
    User,
    System,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserMessage {
    role: Option<Role>,
    content: String,
}

impl UserMessage {
    pub fn new(content: String) -> Self {
        Self {
            role: Some(Role::User),
            content,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SystemMessage {
    role: Option<Role>,
    content: Option<String>,
}

impl Default for SystemMessage {
    fn default() -> Self {
        Self {
            role: Some(Role::System),
            content: Some(
                "You are a helpful, knowledgeable assistant. \
                    Respond concisely and clearly to the user's queries, \
                    focusing on being accurate and informative. When applicable, \
                    explain complex topics in simple terms. Maintain a professional \
                    and respectful tone at all times. If the user asks \
                    for specific technical advice or code, prioritize efficiency \
                    and best practices."
                    .to_string(),
            ),
        }
    }
}

impl SystemMessage {
    pub fn new(content: String) -> Self {
        Self {
            role: Some(Role::System),
            content: Some(content),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Message {
    System(SystemMessage),
    User(UserMessage),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Model {
    #[serde(rename = "claude-3-5-sonnet-20240620")]
    Claude35Sonnet,
    #[serde(rename = "claude-3-opus-20240229")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet-20240229")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-haiku-20240307")]
    Claude3Haiku,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Chat {
    pub model: Model,
    pub max_tokens: u32,
    pub messages: Vec<Message>,
}

impl Chat {
    pub fn builder(model: Model) -> ChatBuilder {
        ChatBuilder {
            model,
            max_tokens: Some(1024),
            messages: Some(Vec::new()),
        }
    }

    // pub fn insert(&m// ut self, context: MessageContext) -> &mut Self {
    //     let mut tera = Tera::default();

    //     self.messages
    //         .iter_mut(|message| tera.add_raw_template(message));
    //     self

    //     // let source = "Hello {{ name }}";
    //     // tera.add_raw_template("hello", source).unwrap();
    //     //
    //     // let mut context = Chyperpriorontext::new();
    //     // context.insert("name", "Rust");
    //     //
    //     // println!("{}", tera.render("hello", &context).unwrap());
    // }
}

pub struct ChatBuilder {
    pub model: Model,
    pub max_tokens: Option<u32>,
    pub messages: Option<Vec<Message>>,
}

impl ChatBuilder {
    pub fn max_tokens(&mut self, count: u32) -> &mut Self {
        self.max_tokens = Some(count);
        self
    }

    pub fn system(&mut self, content: impl Into<String>) -> &mut Self {
        self.messages
            .get_or_insert(Vec::new())
            .push(Message::System(SystemMessage::new(content.into())));
        self
    }

    pub fn message(&mut self, content: impl Into<String>) -> &mut Self {
        self.messages
            .get_or_insert(Vec::new())
            .push(Message::User(UserMessage::new(content.into())));
        self
    }

    pub fn build(&self) -> Chat {
        let chat = Chat {
            model: self.model.clone(),
            max_tokens: self.max_tokens.clone().expect("needs max tokens set"),
            messages: self.messages.clone().expect("messages"),
        };

        chat
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_chat_creation() {
        let chat = Chat::builder(Model::Claude3Haiku)
            .message("you are a robot")
            .build();
        assert_eq!(chat.model, Model::Claude3Haiku);
    }

    #[test]
    fn test_chat_equality() {
        let first_chat = Chat::builder(Model::Claude3Opus)
            .message("what is 2 + 2?")
            .build();

        let second_chat = Chat::builder(Model::Claude3Opus)
            .message("what is 2 + 2?")
            .build();

        assert_eq!(first_chat, second_chat);
    }

    #[test]
    fn test_chaining_messages() {
        let chat = Chat::builder(Model::Claude3Sonnet)
            .max_tokens(10)
            .system("you are a math wiz")
            .message("what is 2 + 2?")
            .message("what is 3 + 3?")
            .build();

        assert_eq!(chat.max_tokens, 10);
        assert_eq!(
            chat.messages,
            vec![
                Message::System(SystemMessage::new("you are a math wiz".to_string())),
                Message::User(UserMessage::new("what is 2 + 2?".to_string())),
                Message::User(UserMessage::new("what is 3 + 3?".to_string()))
            ]
        );
    }

    #[test]
    fn test_templating_no_render() {
        let chat = Chat::builder(Model::Claude3Haiku)
            .max_tokens(10)
            .system("You are a math wiz")
            .message("what is {{ a }} + {{ b }}?")
            .build();

        assert_eq!(
            chat.messages[1],
            Message::User(UserMessage::new("what is {{ a }} + {{ b }}?".to_string()))
        );
    }

    #[test]
    fn test_templating_with_render() {
        let chat = Chat::builder(Model::Claude3Haiku)
            .max_tokens(10)
            .system("You are a math wiz")
            .message("what is {{ a }} + {{ b }}?")
            .build();
    }
}
