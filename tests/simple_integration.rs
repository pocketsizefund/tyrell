use anyhow::Result;
use pretty_assertions::assert_eq;
use tyrell::{ClaudeRequest, ContentType, Model, Role};

use test_log::test;

#[test(tokio::test)]
async fn test_simple_api_request() {
    let chat = ClaudeRequest::builder()
        .model(Model::Haiku3)
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

#[test(tokio::test)]
async fn test_tool_use_request_body_valid() -> Result<()> {
    let chat = ClaudeRequest::builder()
        .model(Model::Sonnet35)
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

    let expected = r#"{
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
      }
    }""#;

    let serialized = serde_json::to_string(&chat)?;

    assert_eq!(expected, serialized);

    Ok(())
}
