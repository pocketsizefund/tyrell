use tyrell::{ClaudeRequest, ContentType, Role};

use test_log::test;

#[test(tokio::test)]
async fn test_simple_api_request() {
    let chat = ClaudeRequest::builder()
        .model("claude-3-opus-20240229")
        .add_message(
            Role::User,
            vec![ContentType::Text {
                text: "Say hello!".to_string(),
            }],
        )
        .max_tokens(10)
        .build()
        .unwrap();

    let response = chat.call().await;

    println!("{:?}", response);
    assert!(response.is_ok());
}
