// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! Verifies bytecode sanity.

#![forbid(unsafe_code)]

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

// Bounds checks are implemented in the `vm` crate.
pub mod check_duplication;
pub mod code_unit_verifier;
pub mod constants;
pub mod control_flow;
pub mod control_flow_graph;
pub mod cyclic_dependencies;
pub mod dependencies;
pub mod instantiation_loops;
pub mod instruction_consistency;
pub mod resources;
pub mod script_signature;
pub mod signature;
pub mod struct_defs;
pub mod verifier;

pub use check_duplication::DuplicationChecker;
pub use code_unit_verifier::CodeUnitVerifier;
pub use instruction_consistency::InstructionConsistency;
pub use resources::ResourceTransitiveChecker;
pub use signature::SignatureChecker;
pub use struct_defs::RecursiveStructDefChecker;
pub use verifier::{verify_module, verify_script};

mod absint;
mod acquires_list_verifier;
mod binary_views;
mod locals_safety;
mod reference_safety;
mod stack_usage_verifier;
mod type_safety;
