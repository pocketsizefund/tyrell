use anyhow::Result;
use jsonxf::pretty_print;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tyrell::{ClaudeRequest, ContentType, Model, Role, Tool, ToolBuilder, ToolChoice};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Organization {
    /// name of the company
    name: String,
    /// is this company listed on the nasdaq?
    listed_on_nasdaq: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Polarity {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct SentimentAnalysis {
    /// what organization is this article about?
    organization: String,
    bullishness: f32,
    key_topics: Vec<String>,
    /// overall sentiment
    polarity: Polarity,
}

impl ToolBuilder for SentimentAnalysis {
    fn name() -> &'static str {
        "analyze_sentiment"
    }

    fn description() -> Option<&'static str> {
        Some("Analyze the sentiment of a news article")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tool = Tool::new::<SentimentAnalysis>();

    // https://techcrunch.com/2024/10/11/anthropic-ceo-goes-full-techno-optimist-in-15000-word-paean-to-ai/
    let news_article = r#"
    Anthropic CEO Dario Amodei wants you to know he’s not an AI “doomer.”

    At least, that’s my read of the “mic drop” of a ~15,000 word essay Amodei published to his blog late Friday. (I tried asking Anthropic’s Claude chatbot whether it concurred, but alas, the post exceeded the free plan’s length limit.)

    In broad strokes, Amodei paints a picture of a world in which all AI risks are mitigated, and the tech delivers heretofore unrealized prosperity, social uplift, and abundance. He asserts this isn’t to minimize AI’s downsides — at the start, Amodei takes aim, without naming names, at AI companies overselling and generally propagandizing their tech’s capabilities. But one might argue that the essay leans too far in the techno-utopianist direction, making claims simply unsupported by fact.

    Amodei believes that “powerful AI” will arrive as soon as 2026. By powerful AI, he means AI that’s “smarter than a Nobel Prize winner” in fields like biology and engineering and that can perform tasks like proving unsolved mathematical theorems and writing “extremely good novels.” This AI, Amodei says, will be able to control any software or hardware imaginable, including industrial machinery, and essentially do most jobs humans do today — but better.

    “[This AI] can engage in any actions, communications, or remote operations … including taking actions on the internet, taking or giving directions to humans, ordering materials, directing experiments, watching videos, making videos, and so on,” Amodei writes. “It does not have a physical embodiment (other than living on a computer screen), but it can control existing physical tools, robots, or laboratory equipment through a computer; in theory it could even design robots or equipment for itself to use.”

    Lots would have to happen to reach that point.

    Even the best AI today can’t “think” in the way we understand it. Models don’t so much reason as replicate patterns they’ve observed in their training data.

    Assuming for the purpose of Amodei’s argument that the AI industry does soon “solve” human-like thought, would robotics catch up to allow future AI to perform lab experiments, manufacture its own tools, and so on? The brittleness of today’s robots imply it’s a long shot.

    Yet Amodei is optimistic — very optimistic.

    He believes AI could, in the next 7 to 12 years, help treat nearly all infectious diseases, eliminate most cancers, cure genetic disorders, and halt Alzheimer’s at the earliest stages. In the next 5 to 10 years, Amodei thinks that conditions like PTSD, depression, schizophrenia, and addiction will be cured with AI-concocted drugs, or genetically prevented via embryo screening (a controversial opinion) — and that AI-developed drugs will also exist that “tune cognitive function and emotional state” to “get [our brains] to behave a bit better and have a more fulfilling day-to-day experience.”

    Should this come to pass, Amodei expects the average human lifespan to double to 150.

    “My basic prediction is that AI-enabled biology and medicine will allow us to compress the progress that human biologists would have achieved over the next 50-100 years into 5-10 years,” he writes. “I’ll refer to this as the ‘compressed 21st century’: the idea that after powerful AI is developed, we will in a few years make all the progress in biology and medicine that we would have made in the whole 21st century.”

    These seem like stretches, too, considering that AI hasn’t radically transformed medicine yet — and may not for quite some time, or ever. Even if AI does reduce the labor and cost involved in getting a drug into pre-clinical testing, it may fail at a later stage, just like human-designed drugs. Consider that the AI deployed in healthcare today has been shown to be biased and risky in a number of ways, or otherwise incredibly difficult to implement in existing clinical and lab settings. Suggesting all these issues and more will be solved roughly within the decade seems, well, aspirational.

    But Amodei doesn’t stop there.

    AI could solve world hunger, he claims. It could turn the tide on climate change. And it could transform the economies in most developing countries; Amodei believes AI can bring the per-capita GDP of sub-Saharan Africa ($1,701 as of 2022) to the per-capita GDP of China ($12,720 in 2022) in 5 to 10 years.

    These are bold pronouncements, although likely familiar to anyone who’s listened to disciples of the “Singularity” movement, which expects similar results. To Amodei’s credit, he acknowledges that such developments would require “a huge effort in global health, philanthropy, [and] political advocacy,” which he posits will occur because it’s in the world’s best economic interest.

    That would be a dramatic change in human behavior if so, given people have shown time and again that their primary interest is in what benefits them in the shorter term. (Deforestation is but one example among thousands.) It’s also worth noting that many of the workers responsible for labeling the datasets used to train AI are paid far below minimum wage while their employers reap tens of millions — or hundreds of millions — in capital from the results.

    Amodei touches, briefly, on the dangers of AI to civil society, proposing that a coalition of democracies secure AI’s supply chain and block adversaries who intend to use AI toward harmful ends from the means of powerful AI production (semiconductors, etc.). In the same breath, he suggests that AI, in the right hands, could be used to “undermine repressive governments” and even reduce bias in the legal system. (AI has historically exacerbated biases in the legal system.)

    “A truly mature and successful implementation of AI has the potential to reduce bias and be fairer for everyone,” Amodei writes.

    So, if AI takes over every conceivable job and does it better and faster, won’t that leave humans in a lurch economically speaking? Amodei admits that, yes, it would, and that at that point, society would have to have conversations about “how the economy should be organized.”

    But he offers no solution.

    “People do want a sense of accomplishment, even a sense of competition, and in a post-AI world it will be perfectly possible to spend years attempting some very difficult task with a complex strategy, similar to what people do today when they embark on research projects, try to become Hollywood actors, or found companies,” he writes. “The facts that (a) an AI somewhere could in principle do this task better, and (b) this task is no longer an economically rewarded element of a global economy, don’t seem to me to matter very much.”

    Amodei advances the notion, in wrapping up, that AI is simply a technological accelerator — that humans naturally trend toward “rule of law, democracy, and Enlightenment values.” But in doing so, he ignores AI’s many costs. AI is projected to have — is already having — an enormous environmental impact. And it’s creating inequality. Nobel Prize-winning economist Joseph Stiglitz and others have noted the labor disruptions caused by AI could further concentrate wealth in the hands of companies and leave workers more powerless than ever.

    These companies include Anthropic, as loath as Amodei is to admit it. Anthropic is a business, after all — one reportedly worth close to $40 billion. And those benefiting from its AI tech are, by and large, corporations whose only responsibility is to boost returns to shareholders, not better humanity.

    A cynic might question the essay’s timing, in fact, given that Anthropic is said to be in the process of raising billions of dollars in venture funds. OpenAI CEO Sam Altman published a similarly techno-optimist manifesto shortly before OpenAI closed a $6.5 billion funding round. Perhaps it’s a coincidence.

    Then again, Amodei isn’t a philanthropist. Like any CEO, he has a product to pitch. It just so happens that his product is going to “save the world” — and those who think otherwise risk being left behind. Or so he’d have you believe.
    "#;

    let chat = ClaudeRequest::builder()
        .model(Model::Sonnet35)
        .add_message(
            Role::Assistant,
            vec![ContentType::Text {
                text: "You are an expert financial analyst specializing in tech industry sentiment analysis.".to_string(),

            }],
        )
        .add_message(
            Role::User,
            vec![ContentType::Text {
                text: format!("Analyze the sentiment of this news article:\n\n{}", news_article),
            }],
        )
        .max_tokens(200)
        .tools(vec![tool])
        .tool_choice(ToolChoice::Specific {
            // TODO: should name be checked that it matches
            // the tool?
            name: "analyze_sentiment".to_string(),
            disable_parallel_tool_use: Some(false),
        })
        .build()
        .unwrap();

    let response = chat.call().await.unwrap();
    let response = pretty_print(&response).unwrap();

    println!("{}", response);

    Ok(())
}
