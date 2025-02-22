// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};
use solagent_core::rig::{completion::ToolDefinition, tool::Tool};
use solagent_parameters::parameters;
use solagent_plugin_goplus::get_token_phishing_site_info;

#[derive(Debug, Deserialize)]
pub struct PhishingSiteInfoArgs {
    url: String,
}

#[derive(Deserialize, Serialize)]
pub struct PhishingSiteInfoOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("PhishingSiteInfo error")]
pub struct PhishingSiteInfoError;

#[derive(Default)]
pub struct PhishingSiteInfo;
impl PhishingSiteInfo {
    pub fn new() -> Self {
        PhishingSiteInfo {}
    }
}

impl Tool for PhishingSiteInfo {
    const NAME: &'static str = "get_token_phishing_site_info";

    type Error = PhishingSiteInfoError;
    type Args = PhishingSiteInfoArgs;
    type Output = PhishingSiteInfoOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_token_phishing_site_info".to_string(),
            description: r#"
            Check if the URL is a phishing site.

            examples: [
                [
                    {
                        input: {
                            url: "http://a.b",
                        },
                    },
                ],
            ]
              "#
            .to_string(),
            parameters: parameters!(
                chain_id: String,
                contract_address: String
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_token_phishing_site_info(&args.url).await.expect("get_token_phishing_site_info");

        Ok(PhishingSiteInfoOutput { data })
    }
}
