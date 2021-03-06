// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use omv_types::{
    loaded_data::runtime_types::Type,
    natives::function::{NativeContext, NativeResult},
    values::Value,
};
use omv_core::errors::PartialVMResult;
use alloc::{vec::Vec, collections::VecDeque};

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
