use anyhow::Result;
use jsonxf::pretty_print;
use octocrab::models::issues::Issue;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{env, fs};
use tyrell::{ClaudeRequest, ContentType, Model, Role, Tool, ToolBuilder, ToolChoice};

// #[derive(Debug, Serialize, Deserialize, JsonSchema)]
// struct Issue {
//     year: u16,
//     winner: String,
//     loser: String,
//     winner_score: u8,
//     loser_score: u8,
//     total_points_scored: Option<u8>,
// }
//
// impl ToolBuilder for SuperBowl {
//     fn name() -> &'static str {
//         "extract_super_bowl_info"
//     }
//
//     fn description() -> Option<&'static str> {
//         Some("Extract Super Bowl information from text")
//     }
// }
//
#[tokio::main]
async fn main() -> Result<()> {
    let earnings_call_transcript = fs::read_to_string("src/lib.rs")?;

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let octocrab = octocrab::OctocrabBuilder::new()
        .personal_token(github_token)
        .build()?;

    let owner = "pocketsizefund";
    let repo = "tyrell";

    let mut page: u32 = 1;
    let mut all_issues = Vec::new();

    loop {
        let issues = octocrab
            .issues(owner, repo)
            .list()
            .state(octocrab::params::State::Open)
            .per_page(100)
            .page(page)
            .send()
            .await?;

        if issues.items.is_empty() {
            break;
        }

        all_issues.extend(issues.items);
        page += 1;
    }

    println!("Total issues fetched: {}", all_issues.len());

    // let extracted_issues: Vec<ExtractedIssue> = all_issues
    //     .into_iter()
    //     .map(|issue| extract_issue_info(issue))
    //     .collect();

    for issue in &all_issues {
        println!("Issue #{:?}: {:?}", issue.number, issue.title);
    }

    // let tool = Tool::new::<SuperBowl>();
    //
    // let chat = ClaudeRequest::builder()
    //     .model(Model::Sonnet35)
    //     .add_message(
    //         Role::Assistant,
    //         vec![ContentType::Text {
    //             text: "You are an NFL historian. Extract the information from the text".to_string(),
    //         }],
    //     )
    //     .add_message(
    //         Role::User,
    //         vec![ContentType::Text {
    //             text: "The Green Bay Packers beat the Miami Dolphins in the 1982 Super Bowl 31-10."
    //                 .to_string(),
    //         }],
    //     )
    //     .max_tokens(200)
    //     .tools(vec![tool])
    //     .tool_choice(ToolChoice::Specific {
    //         // TODO: should name be checked that it matches
    //         // the tool?
    //         name: "extract_super_bowl_info".to_string(),
    //         disable_parallel_tool_use: Some(false),
    //     })
    //     .build()
    //     .unwrap();
    //
    // let response = chat.call().await.unwrap();
    // let response = pretty_print(&response).unwrap();
    //
    // println!("{}", response);

    Ok(())
}
