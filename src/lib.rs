// Copyright 2016 Jeffrey Burdges

//! Zeroing drop wrapper types.

#![feature(core_intrinsics)]

extern crate consistenttime;

mod zd;
mod zdd;
// mod cow;

pub use zd::ZeroDrop;
pub use zdd::ZeroDropDrop;
// pub use cow::ZeroDropCow;

