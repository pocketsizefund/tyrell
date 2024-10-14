use tyrell::{Role, ClaudeRequest, ContentType, ToolChoice, Tool, ToolBuilder};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use test_log::test;

#[test(tokio::test)]
async fn test_super_bowl_extraction_auto_tool_choice() {
    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    struct SuperBowl {
        year: u16,
        winner: String,
        loser: String,
        winner_score: u8,
        loser_score: u8,
        total_points_scored: Option<u8>
    }

    impl ToolBuilder for SuperBowl {
        fn name() -> &'static str {
            "extract_super_bowl_info"
        }

        fn description() -> Option<&'static str> {
            Some("Extract Super Bowl information from text")
        }
    }

    let tool = Tool::new::<SuperBowl>();

    let request = ClaudeRequest::builder()
        .model("claude-3-opus-20240229")
        .add_message(
            Role::User,
            vec![ContentType::Text { 
                text: "Extract the Super Bowl information from this text: The Green Bay Packers beat the Miami Dolphins in the 1982 Super Bowl 31-10.".to_string() 
            }],
        )
        .max_tokens(200)
        .tools(vec![tool])
        .tool_choice(ToolChoice::Specific{name: "extract_super_bowl_info".to_string(), disable_parallel_tool_use: Some(false)})
        .build()
        .unwrap();

    println!("{:#?}", request);
    println!("{:#?}", serde_json::to_string(&request).unwrap());

    let response = request.call().await.unwrap();

    assert_eq!(response.role, Role::Assistant);
    assert!(!response.content.is_empty());

}
