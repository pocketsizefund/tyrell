use jsonxf::pretty_print;
use tyrell::{ClaudeRequest, ContentType, Model, Role};

#[tokio::main]
async fn main() {
    let chat = ClaudeRequest::builder()
        .model(Model::Opus3)
        .add_message(
            Role::User,
            vec![ContentType::Text {
                text: "who was the 16th president of the USA?".to_string(),
            }],
        )
        .max_tokens(200)
        .build()
        .unwrap();

    let response = chat.call().await.unwrap();
    let response = pretty_print(&response).unwrap();

    println!("{}", response);
}
