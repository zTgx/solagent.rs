use crate::{actions::fetch_price_by_pyth, parameters_json_schema};
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct FetchPricePyThArgs {
    price_feed_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchPricePyThOutput {
    pub price: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("FetchPricePyTh error")]
pub struct FetchPricePyThError;

pub struct FetchPricePyTh;
impl FetchPricePyTh {
    pub fn new() -> Self {
        FetchPricePyTh {}
    }
}

impl Tool for FetchPricePyTh {
    const NAME: &'static str = "fetch_price_by_pyth";

    type Error = FetchPricePyThError;
    type Args = FetchPricePyThArgs;
    type Output = FetchPricePyThOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "fetch_price_by_pyth".to_string(),
            description: r#"Fetch the current price from a Pyth oracle price feed
                input: {
                    price_feed_id: "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
                },
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                price_feed_id: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let price_feed_id = args.price_feed_id;
        let price = fetch_price_by_pyth(&price_feed_id)
            .await
            .expect("fetch_price_by_pyth");

        Ok(FetchPricePyThOutput { price })
    }
}
