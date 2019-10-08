#![warn(absolute_paths_not_starting_with_crate)]
#![warn(deprecated_in_future)]
#![warn(macro_use_extern_crate)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_labels)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::float_arithmetic)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::mem_forget)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::option_unwrap_used)]
#![warn(clippy::print_stdout)]
#![warn(clippy::result_unwrap_used)]
#![warn(clippy::string_add)]
#![warn(clippy::unimplemented)]
#![warn(clippy::use_debug)]
#![warn(clippy::wrong_pub_self_convention)]
#![allow(clippy::empty_enum)]
#![allow(clippy::large_enum_variant)]

pub mod beacon_state;
pub mod config;
pub mod consts;
pub mod primitives;
pub mod types;
