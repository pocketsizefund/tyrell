use test_log::test;
use tyrell::{ClaudeRequest, ContentType, Role};

#[test(tokio::test)]
async fn test_qa() {
    let chat = ClaudeRequest::builder()
        .model("claude-3-opus-20240229")
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
    println!("{:#?}", response);

    assert_eq!(response.role, Role::Assistant);
    assert!(!response.content.is_empty());
}
