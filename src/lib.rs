// Self-sovereign identity (SSID)
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2023-204 by
//     Cypher<cypher@cyphernet.io>
//
// Copyright 2023-2024 Cyphernet DAO, Switzerland
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

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/// # URLs
///
/// - `ssid:<baid58>`, chunked
///
/// # Armors
///
/// Base85 armor in "bindle" format.
///
/// Used for IdCert and SigCert

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding;

mod algo;
mod identity;
mod sigs;
mod bindle;
mod proofs;
mod seal;

pub use crate::algo::{Fingerprint, Pk, RistrettoPk, RistrettoSig, RistrettoSk, Sig, Sk};
pub use crate::bindle::{Bindle, BindleContent, BindleParseError, LoadError};
pub use crate::identity::{IdCert, Identity, Revocation, Ssi};
pub use crate::proofs::{BpProof, Proof};
pub use crate::seal::Seal;
pub use crate::sigs::{SigCert, Signature};

pub const LIB_NAME_SSID: &str = "SSID";

pub type Digest = amplify::Bytes32;
