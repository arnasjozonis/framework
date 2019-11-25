#[macro_use]
extern crate error_chain;

#[macro_use]
mod macros;

pub mod attestation_producer;
pub mod service;
pub mod duties_manager;
pub mod beacon_node;
pub mod rest_client;
pub mod errors;