/*
 * Copyright (c) 2020 gematik GmbH
 * 
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * 
 *    http://www.apache.org/licenses/LICENSE-2.0
 * 
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ContactPointDef {
    #[serde(default)]
    #[serde(alias = "system")]
    #[serde(rename = "value-tag=system")]
    pub system: Option<String>,

    #[serde(default)]
    #[serde(alias = "value")]
    #[serde(rename = "value-tag=value")]
    pub value: Option<String>,

    #[serde(default)]
    #[serde(alias = "use")]
    #[serde(rename = "value-tag=use")]
    pub use_: Option<String>,

    #[serde(default)]
    #[serde(alias = "rank")]
    #[serde(rename = "value-tag=rank")]
    pub rank: Option<usize>,
}