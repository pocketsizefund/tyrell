use anyhow::Result;
use jsonxf::pretty_print;
use octocrab::models::issues::Issue;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{env, fmt, fs};
use tyrell::{ClaudeRequest, ContentType, Model, Role, Tool, ToolBuilder, ToolChoice};

#[derive(Debug, Serialize, Deserialize)]
struct ExtractedIssue {
    number: u64,
    title: String,
    created_at: String,
    labels: Vec<String>,
    assignees: Vec<String>,
    comments: u32,
}

impl fmt::Display for ExtractedIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Issue #{}: {}\n", self.number, self.title)?;
        write!(f, "Created: {}\n", self.created_at)?;
        write!(f, "Labels: {}\n", self.labels.join(", "))?;
        write!(f, "Assignees: {}\n", self.assignees.join(", "))?;
        write!(f, "Comments: {}\n", self.comments)
    }
}

fn extract_issue_info(issue: Issue) -> ExtractedIssue {
    ExtractedIssue {
        number: issue.number,
        title: issue.title,
        created_at: issue.created_at.to_rfc3339(),
        labels: issue.labels.into_iter().map(|label| label.name).collect(),
        assignees: issue.assignees.into_iter().map(|user| user.login).collect(),
        comments: issue.comments,
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Assignee {
    ChrisAddy,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CodeBlock(String);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct Task {
    /// Github Issue
    issue: String,
    /// Who should pick up the task, does not necessarily have to be the assignee
    /// on the ticket, you can choose someone who is free
    assignee: Assignee,
    /// Code suggestions in markdown code block
    suggestions: CodeBlock,
    /// For the given code suggestions, write as many tests
    /// as appropriate to sufficiently cover edge cases
    tests: CodeBlock,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TaskBoard {
    /// sort in descending order of priority based on
    /// how big an impact the change would have on the
    /// code base balanced with the ease of implementing
    /// bugs should generally be prioritized higher
    tasks: Vec<Task>,
}

impl ToolBuilder for TaskBoard {
    fn name() -> &'static str {
        "organize_tasks"
    }

    fn description() -> Option<&'static str> {
        Some("Organize tasks from our backlog, prioritize them in descending order, and create suggestions for implementation.")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let source_code = fs::read_to_string("src/lib.rs")?;

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

    for issue in &all_issues {
        println!("Issue #{}: {}", issue.number, issue.title);
    }

    let extracted_issues: String = all_issues
        .into_iter()
        .map(extract_issue_info)
        .map(|issue| issue.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    let tool = Tool::new::<TaskBoard>();

    let chat = ClaudeRequest::builder()
        .model(Model::Sonnet35)
        .add_message(
            Role::Assistant,
            vec![ContentType::Text {
                text: "You are a lead software engineer helping prioritize a backlog. You are given the current code base an our open github issues. Use this context to prioritize tasks and suggest implementations. You must give code suggestions and write tests.".to_string(),
            }],
        )
        .add_message(
            Role::User,
            vec![ContentType::Text {
                text: format!("code base: {}", source_code).to_string(),
            }],
        )
        .add_message(
            Role::User,
            vec![ContentType::Text {
                text: format!("open issues: {}", extracted_issues).to_string(),
            }],
        )
        .max_tokens(2048)
        .tools(vec![tool])
        .tool_choice(ToolChoice::Specific {
            name: "organize_tasks".to_string(),
            disable_parallel_tool_use: Some(false),
        })
        .build()
        .unwrap();

    let response = chat.call().await.unwrap();
    let response = pretty_print(&response).unwrap();

    println!("{}", response);

    Ok(())
}
