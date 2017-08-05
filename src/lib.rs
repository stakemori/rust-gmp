#![crate_name = "gmp"]

#![warn(deprecated)]
#![allow(non_camel_case_types)]

extern crate libc;
extern crate num_traits;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod ffi;

#[macro_use]
mod macros;
pub mod mpz;
pub mod mpq;
pub mod mpf;
pub mod rand;
pub mod sign;
