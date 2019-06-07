// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An implementation of union-find and (optionally)
//! congruence-closure. See the `unify` and `cc` modules for more
//! details. Note that congruence-closure requires you to opt-in to
//! the feature "congruence-closure".

// The CC code uses `impl Trait`
#![cfg_attr(feature = "bench", feature(test))]

#[macro_use]
extern crate log;

#[cfg(feature = "persistent")]
extern crate dogged;

#[cfg(feature = "congruence-closure")]
extern crate petgraph;

pub mod cc;
pub mod snapshot_vec;
pub mod unify;
