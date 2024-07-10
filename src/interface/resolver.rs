// RGB standard library for working with smart contracts on Bitcoin & Lightning
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2024 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2019-2024 LNP/BP Standards Association. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rgb::validation::{ResolveWitness, WitnessResolverError};
use rgb::XWitnessTx;
use strict_encoding::StrictDumb;

use crate::resolvers::ResolveHeight;
use crate::{WitnessAnchor, XWitnessId};

pub(crate) struct DumbResolver;

impl ResolveWitness for DumbResolver {
    fn resolve_pub_witness(&self, _: XWitnessId) -> Result<XWitnessTx, WitnessResolverError> {
        Ok(XWitnessTx::strict_dumb())
    }
}

impl ResolveHeight for DumbResolver {
    fn resolve_height(&mut self, _: XWitnessId) -> Result<WitnessAnchor, String> {
        Ok(WitnessAnchor::strict_dumb())
    }
}
