// Copyright (c) 2020 Wei Tang.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use sp_core::{blake2_256, H256};
use crate::is_label;

/// Get the label hash of a string. Returns None if the label is not valid.
pub fn labelhash(s: &str) -> Option<H256> {
	if !is_label(s) {
		return None
	}

	Some(H256(blake2_256(s.as_bytes())))
}

/// Get the root namehash.
pub fn root_namehash() -> H256 {
	H256::default()
}

/// Get a name hash given a parent namehash. Returns None if the label is not valid.
pub fn namehash(name: &str, parent: &H256) -> Option<H256> {
	if let Some(labelhash) = labelhash(name) {
		let mut input = [0u8; 64];

		input[0..32].copy_from_slice(&parent[..]);
		input[32..64].copy_from_slice(&labelhash[..]);

		Some(H256(blake2_256(&input)))
	} else {
		None
	}
}
