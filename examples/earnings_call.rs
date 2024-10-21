use anyhow::Result;
use jsonxf::pretty_print;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use tyrell::{ClaudeRequest, ContentType, Model, Role, Tool, ToolBuilder, ToolChoice};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EarningsCallAnalysis {
    /// Company name
    company_name: String,
    /// Stock ticker symbol
    ticker: String,
    /// Date of the earnings call
    call_date: String,
    /// Fiscal quarter and year (e.g., "Q2 2023")
    fiscal_period: String,
    /// Reported earnings per share (EPS)
    reported_eps: f64,
    /// Analyst consensus EPS estimate
    estimated_eps: f64,
    /// Reported revenue
    reported_revenue: f64,
    /// Analyst consensus revenue estimate
    estimated_revenue: f64,
    /// Year-over-year revenue growth rate
    yoy_revenue_growth: f64,
    /// Net income for the quarter
    net_income: f64,
    /// Key performance indicators (KPIs) mentioned in the call
    kpis: Vec<KPI>,
    /// Notable quotes from executives
    key_quotes: Vec<String>,
    /// Forward-looking statements or guidance
    guidance: Option<Guidance>,
    /// Major announcements or updates
    announcements: Vec<String>,
    /// Sentiment analysis of the call
    sentiment: CallSentiment,
    /// Potential risk factors mentioned
    risk_factors: Vec<String>,
    /// Analyst questions and management responses
    qa_summary: Vec<QAItem>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KPI {
    /// Name of the KPI
    name: String,
    /// Value of the KPI
    value: String,
    /// Previous period's value, if mentioned
    previous_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Guidance {
    /// Expected revenue range for next quarter
    next_quarter_revenue: Option<(f64, f64)>,
    /// Expected EPS range for next quarter
    next_quarter_eps: Option<(f64, f64)>,
    /// Expected revenue range for full year
    full_year_revenue: Option<(f64, f64)>,
    /// Expected EPS range for full year
    full_year_eps: Option<(f64, f64)>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CallSentiment {
    /// Overall sentiment score (-1.0 to 1.0)
    overall_score: f64,
    /// Sentiment towards company performance
    performance_sentiment: String,
    /// Sentiment towards future outlook
    outlook_sentiment: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QAItem {
    /// Question asked by the analyst
    question: String,
    /// Summary of management's response
    response_summary: String,
}

impl ToolBuilder for EarningsCallAnalysis {
    fn name() -> &'static str {
        "analyze_earnings_call"
    }

    fn description() -> Option<&'static str> {
        Some("Extract information from a quarterly earnings call")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tool = Tool::new::<EarningsCallAnalysis>();

    // https://s2.q4cdn.com/661678649/files/doc_financials/2024/q2/2Q24-Boeing-Earnings-Call-Transcript.pdf
    let earnings_call_transcript =
        fs::read_to_string("examples/boeing_2024q2_earnings_transcript.txt")?;

    let chat = ClaudeRequest::builder()
        .model(Model::Haiku3)
        .add_message(
            Role::Assistant,
            vec![ContentType::Text {
                text: "You are an expert financial analyst.".to_string(),
            }],
        )
        .add_message(
            Role::User,
            vec![ContentType::Text {
                text: format!(
                    "Analyze the this quarterly earnings call:\n\n{}",
                    earnings_call_transcript
                ),
            }],
        )
        .max_tokens(200)
        .tools(vec![tool])
        .tool_choice(ToolChoice::Specific {
            // TODO: should name be checked that it matches
            // the tool?
            name: "analyze_earnings_call".to_string(),
            disable_parallel_tool_use: Some(false),
        })
        .build()
        .unwrap();

    let response = chat.call().await.unwrap();
    let response = pretty_print(&response).unwrap();

    println!("{}", response);

    Ok(())
}
