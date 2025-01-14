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

use async_trait::async_trait;
use resources::misc::PrescriptionId;

use crate::fhir::{
    decode::{DataStream, Decode, DecodeError, DecodeStream, Search},
    definitions::primitives::IdentifierEx,
    encode::{DataStorage, Encode, EncodeError, EncodeStream},
};

#[async_trait(?Send)]
impl Decode for PrescriptionId {
    async fn decode<S>(stream: &mut DecodeStream<S>) -> Result<Self, DecodeError<S::Error>>
    where
        S: DataStream,
    {
        let value = stream.value(Search::Any).await?.unwrap();
        let value = match value.parse() {
            Ok(value) => value,
            Err(_) => {
                return Err(DecodeError::InvalidValue {
                    value,
                    path: stream.path().into(),
                })
            }
        };

        Ok(value)
    }
}

impl Encode for &PrescriptionId {
    fn encode<S>(self, stream: &mut EncodeStream<S>) -> Result<(), EncodeError<S::Error>>
    where
        S: DataStorage,
    {
        stream.value(self.to_string())?;

        Ok(())
    }
}

impl IdentifierEx for PrescriptionId {
    fn from_parts(value: String) -> Result<Self, String> {
        match value.as_str().parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(value),
        }
    }

    fn value(&self) -> String {
        self.to_string()
    }

    fn system() -> Option<&'static str> {
        Some("https://gematik.de/fhir/NamingSystem/PrescriptionID")
    }
}
