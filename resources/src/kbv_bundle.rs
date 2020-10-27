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

use super::{
    misc::PrescriptionId,
    primitives::{Id, Instant},
    Composition, Coverage, Medication, MedicationRequest, Organization, Patient, Practitioner,
    PractitionerRole,
};

#[derive(Clone, PartialEq, Debug)]
pub struct KbvBundle {
    pub id: Id,
    pub meta: Meta,
    pub identifier: PrescriptionId,
    pub timestamp: Instant,
    pub entry: Entry,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Meta {
    pub last_updated: Option<Instant>,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Entry {
    pub composition: Option<(String, Composition)>,
    pub medication_request: Option<(String, MedicationRequest)>,
    pub medication: Option<(String, Medication)>,
    pub patient: Option<(String, Patient)>,
    pub practitioner: Option<(String, Practitioner)>,
    pub organization: Option<(String, Organization)>,
    pub coverage: Option<(String, Coverage)>,
    pub practitioner_role: Option<(String, PractitionerRole)>,
}