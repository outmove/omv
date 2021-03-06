// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use omv_primitives::gas_schedule::GasAlgebra;
use omv_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeContext, NativeResult},
    values::Value,
};
use smallvec::smallvec;
use omv_core::errors::PartialVMResult;
use alloc::{vec::Vec, collections::VecDeque};

pub fn native_emit_event(
    context: &mut impl NativeContext,
    mut ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.len() == 3);

    let ty = ty_args.pop().unwrap();
    let msg = arguments.pop_back().unwrap();
    let seq_num = pop_arg!(arguments, u64);
    let guid = pop_arg!(arguments, Vec<u8>);

    let cost = native_gas(
        context.cost_table(),
        NativeCostIndex::EMIT_EVENT,
        msg.size().get() as usize,
    );

    if !context.save_event(guid, seq_num, ty, msg)? {
        return Ok(NativeResult::err(cost, 0));
    }

    Ok(NativeResult::ok(cost, smallvec![]))
}
