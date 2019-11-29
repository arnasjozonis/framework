# Honest validator client by [ Arnas, Aurintas, Rasa ]

Implementation of Honest validator client for 2019 VU MIF Blockchain course. Validator client written to comply [the eth2 spec](https://github.com/ethereum/eth2.0-specs/blob/dev/specs/validator/0_beacon-chain-validator.md).

## Validator role in eth2 blockchain

The Honest validator client is responsible for the following tasks:

- Requesting validator duties from the BN.
- Prompting the BN to produce a new block, when a validators block production duties require.
- Completing all the fields on a new block (e.g., RANDAO reveal, signature) and publishing the block to a BN.
- Prompting the BN to produce a new shard attestation as per a validators duties.
- Ensuring that no slashable messages are signed by a validator private key.
- Keeping track of the system clock and how it relates to slots/epochs.

## Setting up test environment

- Validator client can be tested with lighthouse test configuration. To set it up, please follow these instructions:

1. [Download](https://github.com/sigp/lighthouse) lighthouse eth2 implementation.

2. Follow lighthouse [instructions](http://lighthouse-book.sigmaprime.io/setup.html) for development setup.

3. After 'make' command successfully builds lighthouse application, add lighthouse to path with command: cargo install --path lighthouse

4. run this line in terminal (8 - validators count, 157444926 - genesis start time, can be changed to whatever [UNIX time](https://duckduckgo.com/?q=unix+time&t=ffab&ia=answer)): lighthouse beacon_node testnet -f quick 8 157444926

5. Now, you have beacon node running and listening for HTTP API requests on localhost:5052. ([API Swagger documention](https://app.swaggerhub.com/apis-docs/spble/lighthouse_rest_api/0.2.0), note, that not all endpoints are implemented).

6. Validator keys used for testing, can be found [here](https://github.com/ethereum/eth2.0-pm/blob/9a9dbcd95e2b8e10287797bd768014ab3d842e99/interop/mocked_start/keygen_10_validators.yaml?fbclid=IwAR3jngEcUE0nmI0oWV0PMd-cdmUuiuwe-jnSsV3fuKijFej3Vz5f3OZkSyM).

7. _Run Honest Validator client, with args..._
