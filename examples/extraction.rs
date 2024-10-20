use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use tyrell::{ClaudeRequest, ContentType, Model, Role, Tool, ToolBuilder, ToolChoice};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct SuperBowl {
    year: u16,
    winner: String,
    loser: String,
    winner_score: u8,
    loser_score: u8,
    total_points_scored: Option<u8>,
}

impl ToolBuilder for SuperBowl {
    fn name() -> &'static str {
        "extract_super_bowl_info"
    }

    fn description() -> Option<&'static str> {
        Some("Extract Super Bowl information from text")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tool = Tool::new::<SuperBowl>();

    let chat = ClaudeRequest::builder()
        .model(Model::Sonnet35)
        .add_message(
            Role::Assistant,
            vec![ContentType::Text {
                text: "You are an NFL historian. Extract the information from the text".to_string(),
            }],
        )
        .add_message(
            Role::User,
            vec![ContentType::Text {
                text: "The Green Bay Packers beat the Miami Dolphins in the 1982 Super Bowl 31-10."
                    .to_string(),
            }],
        )
        .max_tokens(200)
        .tools(vec![tool])
        .tool_choice(ToolChoice::Specific {
            // TODO: should name be checked that it matches
            // the tool?
            name: "extract_super_bowl_info".to_string(),
            disable_parallel_tool_use: Some(false),
        })
        .build()
        .unwrap();

    let body = serde_json::to_string_pretty(&chat)?;
    println!("{:#?}", body);
    let mut file = File::create("testing.json")?;
    file.write_all(body.as_bytes())?;

    let response = chat.call().await.unwrap();

    println!("{:#?}", response);

    Ok(())
}
