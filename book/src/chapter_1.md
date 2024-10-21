# Chapter 1

```rust
use tyrell::{ClaudeRequest, ContentType, Role};

fn main() {
    let chat = ClaudeRequest::builder()
        .model("claude-3-opus-20240229")
        .add_message(Role::User, vec![ContentType::Text {
            text: "who was the 16th president of the United States?".to_string()
        }])
        .max_tokens(200)
        .build()
        .unwrap();


    let response = chat.call().await.unwrap();

    println!("{:?}", response);
}
```
