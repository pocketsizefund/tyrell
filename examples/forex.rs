use anyhow::Result;
use jsonxf::pretty_print;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tyrell::{ClaudeRequest, ContentType, Model, Role, Tool, ToolBuilder};
use futures::future::join_all;

/// Represents a single economic indicator for a country's economy
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EconomicIndicator {
    /// Name of the economic indicator (e.g., "GDP", "Inflation Rate", "Unemployment Rate")
    name: String,
    /// Current value of the indicator, if available
    value: Option<f64>,
    /// Current trend of the indicator (e.g., "Increasing", "Decreasing", "Stable")
    trend: String,
}

/// Comprehensive analysis of a country's economic situation
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EconomyAnalysis {
    /// Name of the country being analyzed
    country: String,
    /// List of relevant economic indicators for the country
    economic_indicators: Vec<EconomicIndicator>,
    /// Current stance of the country's monetary policy (e.g., "Hawkish", "Dovish", "Neutral")
    monetary_policy_stance: String,
    /// List of significant events affecting the country's economy
    notable_events: Vec<String>,
    /// Overall sentiment or outlook for the country's economy
    overall_economic_sentiment: String,
}

impl ToolBuilder for EconomyAnalysis {
    fn name() -> &'static str {
        "analyze_economy"
    }

    fn description() -> Option<&'static str> {
        Some("Analyze economic news and extract key information about a country's economy")
    }
}

async fn analyze_news(news: &str, country: &str) -> String {
    let tool = Tool::new::<EconomyAnalysis>();

    let chat = ClaudeRequest::builder()
        .model(Model::Sonnet35)
        .add_message(
            Role::Assistant,
            vec![ContentType::Text {
                text: "You are an expert economic analyst specializing in forex markets.".to_string(),
            }],
        )
        .add_message(
            Role::User,
            vec![ContentType::Text {
                text: format!("Analyze this news article about the {} economy and extract key information:\n\n{}", country, news),
            }],
        )
        .max_tokens(512)
        .tools(vec![tool])
        .tool_choice(tyrell::ToolChoice::Specific {
            name: "analyze_economy".to_string(),
            disable_parallel_tool_use: Some(false),
        })
        .build().expect("failed to build request");

    chat.call().await.expect("failed to call Claude")
 
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct CurrencyPair {
    /// The forex currency pair in standard format, e.g., "USD/EUR", "JPY/GBP"
    pair: String,
    /// A detailed explanation of why this pair was chosen, including relevant economic factors and potential opportunities
    rationale: String,
    /// An assessment of the trade's risk level (e.g., "Low", "Medium", "High") with a brief explanation of contributing factors
    risk_assessment: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct ForexRecommendation {
    /// A list of recommended currency pairs to trade based on the provided economic analyses
    pairs: Vec<CurrencyPair>,
}

impl ToolBuilder for ForexRecommendation {
    fn name() -> &'static str {
        "recommend_forex_trades"
    }

    fn description() -> Option<&'static str> {
        Some("Recommend forex currency pairs to trade based on economic analyses")
    }
}

async fn recommend_forex_trades(analyses: String) -> String {
    let tool = Tool::new::<ForexRecommendation>();

    let analyses_json = serde_json::to_string(&analyses).expect("failed to convert to string");

    let chat = ClaudeRequest::builder()
        .model(Model::Sonnet35)
        .add_message(
            Role::Assistant,
            vec![ContentType::Text {
                text: "You are an expert forex trader with deep knowledge of global economics.".to_string(),
            }],
        )
        .add_message(
            Role::User,
            vec![ContentType::Text {
                text: format!("Based on these economic analyses, recommend forex currency pairs to trade:\n\n{}", analyses_json),
            }],
        )
        .max_tokens(512)
        .tools(vec![tool])
        .tool_choice(tyrell::ToolChoice::Specific {
            name: "recommend_forex_trades".to_string(),
            disable_parallel_tool_use: Some(false),
        })
        .build().expect("failed to call claude");

    chat.call().await.expect("failed to call claude")
}


#[tokio::main]
async fn main() -> Result<()> {
    let european_central_bank = r#"
    ECB’s Villeroy Says Risk of Undershooting Inflation Target Now as Great as Overshoot
    The eurozone’s central bank Thursday lowered its key interest rate, marking the first
    back-to-back cut in borrowing costs since 2011
    The risk that eurozone inflation will fall below the European Central Bank’s target 
    is now as great as the risk that it will exceed it, the governor of the Bank of France 
    said Friday.
    The eurozone’s central bank Thursday lowered its key interest rate, marking the first 
    back-to-back cut in borrowing costs since 2011 as the outlook for the economy weakens.
    In a statement, François Villeroy de Galhau said that would not be the last rate cut.
    “The risk of sustainably missing our target by going down now exists as much as that 
    of exceeding it,” he said. “We should continue to reduce the restrictiveness 
    of our monetary policy as appropriate.”
    Villeroy said eurozone inflation is set to reach the ECB’s 2% target 
    “earlier than expected in 2025” while there is no clear sign of a pickup in economic growth.
    “The persistent moderation of private investment and consumption, with in particular 
    the recent rise in household savings rates, justifies this new drop in interest rates,” he said.
    The ECB has now lowered its key rate three times since June, and by a quarter of a percentage 
    point in each of those steps. But Villeroy indicated that larger moves are possible.
    “The pace must be one of agile pragmatism: in a highly uncertain international environment, 
    we retain full optionality for our upcoming meetings,” he said.
    ""#;

    let japan_union = r#"Japan’s Largest Labor Union Group to Seek Big Pay Hike Next Year
    The move would likely be welcome by the central bank
    TOKYO—Japan’s largest trade union group is going to seek a 5% pay increase for workers 
    at next year’s annual wage negotiations, a move that would likely 
    be welcome by the central bank as it strives to create a so-called virtuous 
    cycle of growth in wages and consumption.
    That would be the second straight year that the Japanese Trade Union Confederation, 
    known as Rengo, has requested a more than 5% wage increase. Companies gave employees 
    an average raise of 5.10% this year, the largest increase in 33 years and higher than 3.58% 
    in 2023.
    “I think this will be the year that we should get the 5% level entrenched in society,” 
    Akira Nidaira, executive director of Rengo’s policy department, said at a news conference 
    on Thursday. “The stage has changed in 2024, but many people are still not on board.”
    As the biggest collection of unions in the country with roughly 7 million members, 
    what Rengo requests typically sets the tone for the annual “Shunto” wage talks in Japan, 
    which see thousands of unions negotiate pay increases with companies.
    In an attempt to help those who work for smaller companies catch up with the pay increases 
    made by large companies, the labor organization set a goal of asking smaller firms 
    to lift wages by 6% or more. Small- and medium-sized businesses employ 
    about 70% of the workers in Japan.
    Companies with fewer than 300 union members gave a 4.45% pay increase to workers this year, 
    according to Rengo data.
    What happens at next year’s Shunto talks will be watched closely as it could affect 
    the Bank of Japan’s decision on when it can raise interest rates, how quickly and by how much.
    Boosting wage growth has also taken on some political urgency ahead of 
    the general election set for Oct. 27. Japan’s newly minted prime minister, Shigeru Ishiba, 
    has pledged to achieve economic growth backed by higher wages.
    A positive cycle of higher wages, more spending and steady inflation has finally set in 
    after years of stagnation on all three fronts. But wage growth has yet to improve to the levels 
    that would let policymakers call victory in the effort to revive demand.
    Some are still worried about Japanese companies’ willingness to continue raising pay next year.
    While Japan’s chronic labor shortages will keep upward pressure on wages, 
    companies may not feel the same pressing need to raise salaries as they did earlier this year 
    because inflation has recently become milder.
    Consumer prices rose 2.5% in September from a year earlier, compared with an average of 3.0% 
    rise in the fiscal year ended in March and a 3.2% increase in the prior year.
    Another headwind could be posed by the yen’s further recovery, which would hurt 
    Japanese exporters’ earnings and reduce their capacity for labor expenses.
    "#;

    // https://www.forbes.com/sites/simonmoore/2024/10/15/where-might-interest-rates-go-in-2025/
    let us_interest_rates = r#""
    Where Interest Rates Might Go In 2025

    Interest rates are expected to move lower in 2025. The question is: How much lower? There are two motivations for the Federal Open Market Committee to cut rates. These tie back to the FOMC’s dual mandate of price stability and full employment.

    Inflation Is Coming Under Control
    The first motivation is that inflation now appears under control. To fight inflation, the FOMC raised interest rates sharply from 0% to 0.25% in January 2022 to 5.25% to 5.5% by July 2023. Now, inflation is a lot lower. The FOMC monitors the Personal Consumption Expenditures Price Index as its preferred inflation metric. That indicator hit a 7.2% annual rate in June 2022 and is 2.2% for August 2024. As a result, the FOMC might be tempted to continue to remove the restrictive interest rates that were intended to fight inflation, a process that may have started with the FOMC’s first rate cut of this cycle in September 2024.

    Of course, there’s always a risk that inflation flares up again, or that inflation doesn’t quite hit the FOMC’s 2% target. Still inflation is now much closer to the FOMC’s target than in recent years. Lower inflation suggests interest rates could move lower.

    The Job Market Is More Ambiguous
    The second motivation for interest rates is a little harder to gauge. It’s the risk that unemployment could move higher than the FOMC would like. Maintaining full employment is the other half of the FOMC’s dual mandate. In a sense, the FOMC wants to avoid a recession because of the job destruction that would likely result. In a recession, the FOMC might want to lower interest rates to stimulate the economy and create jobs.

    Jobs data current appears to be more complex than inflation trends. Unemployment was steadily increasing for most of 2024, up until July when it hit 4.3%. That triggered some fears of a rise in unemployment that might prove sharp enough to cause a recession.

    However, employment reports for August and September then showed unemployment moving lower with greater job creation, alleviating some concerns of an imminent recession. So far, the FOMC has expressed some confidence in the job market, though it continues to monitor data closely.

    So from here, the issue is essentially how much the FOMC might want to lower interest rates given that inflation is now less of concern, and the job market could weaken.

    The 2025 FOMC Meeting Schedule

    Assuming no dramatic economic events, the FOMC will likely adhere to the following eight scheduled dates for determining interest rates in 2025: January 29, March 19, May 7, June 18, July 30, September 17, October 29 and December 10. Each of these decisions will be accompanied by a press conference with Federal Reserve Chair Jerome Powell. The meetings in March, June, September and December will also include an update to the Summary of Economic Projections from FOMC members. All meetings will also see detailed minutes released three weeks after the meeting.

    The Interest Rate Outlook In 2025
    FOMC policymakers’ own estimates of where short-term rates will be in December 2025 is, on average, a little more than 3%, with a median forecast of 3.4%. That forecast was updated at the FOMC’s last meeting on September 18. However, there are a broad range of forecasts across individual policymakers with rate levels from 2.75% to 4.25%. As such, every FOMC member is expecting lower rates, but assessments vary on how much they will decline.

    Fixed income markets also implicitly project where interest rates are expected to trend. This is captured by the CME’s FedWatch Tool. These forecasts are broadly similar to the FOMC’s September projections, seeing rates at 3.25% to 3.5% as most likely for December 2025. Again there are a broad range of outcomes spanning from 2.5% to just above 4%. Therefore, fixed income markets agree that rates will most likely end 2025 just below 3.5%, but they see a range of outcomes that includes slightly lower rates than the FOMC’s own September estimates.

    So, given current economic trends and data, expect short-term rates to end 2025 around 3.5%, representing substantial cuts from the current 4.75% to 5% range. This would largely reflect removing restrictive monetary policy given cooling inflation. However, the main question is the jobs market. If it remains robust then interest rates could perhaps stay closer to 4% and if it were to weaken substantially, rates could fall below 3%.
    ""#;
    
    let analyses_futures = vec![
        analyze_news(european_central_bank, "Eurozone"),
        analyze_news(japan_union, "Japan"),
        analyze_news(us_interest_rates, "US"),
    ];

    let analyses: String = join_all(analyses_futures)
        .await
        .into_iter()
        .collect::<Vec<_>>()
        .join("\n");

    let recommendation = recommend_forex_trades(analyses).await;
    let forex_recommendation: serde_json::Value = serde_json::from_str(recommendation.as_str()).unwrap();
    let pretty_recommendation = pretty_print(&forex_recommendation.to_string()).unwrap();

    println!("{}", pretty_recommendation);

    Ok(())
}
