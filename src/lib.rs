// Copyright 2016 Jeffrey Burdges

//! Zeroing drop wrapper types.

#![feature(core_intrinsics)]

extern crate consistenttime;

mod zd;
mod zdd;

pub use zd::ZeroDrop;
pub use zdd::ZeroDropDrop;

