/*
 * Copyright (c) 2021 gematik GmbH
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

use super::{misc::TelematikId, primitives::DateTime};

#[derive(Clone, PartialEq, Debug)]
pub struct ErxComposition {
    pub beneficiary: TelematikId,
    pub date: DateTime,
    pub author: String,
    pub event_start: DateTime,
    pub event_end: DateTime,
}