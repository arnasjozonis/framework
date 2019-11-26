#[macro_use]
extern crate error_chain;

#[macro_use]
mod macros;

pub mod attestation_producer;
pub mod beacon_node;
pub mod duties_manager;
pub mod errors;
pub mod rest_client;
pub mod service;
