// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use omv_types::{
    loaded_data::runtime_types::Type,
    natives::function::{NativeContext, NativeResult},
    values::Value,
};
use std::collections::VecDeque;
use omv_core::errors::PartialVMResult;

pub fn native_sha2_256(
    _context: &impl NativeContext,
    _ty_args: Vec<Type>,
    mut _arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    unimplemented!()
}

pub fn native_sha3_256(
    _context: &impl NativeContext,
    _ty_args: Vec<Type>,
    mut _arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    unimplemented!()
}
