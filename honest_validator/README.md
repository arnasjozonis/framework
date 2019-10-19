# Honest validator client (HVC) by [ Arnas, Aurintas, Rasa ]

[comment]: # (TODO: add short description)
_Should be short description of the project._

## Roles

[comment]: # (TODO: As this is copied from lighthouse docs, use this as guide at first, but ultimately needs to be replaced by our original content)

The HVC is responsible for the following tasks:

- Requesting validator duties (a.k.a. shuffling) from the BN.
- Prompting the BN to produce a new block, when a validators block production
	duties require.
- Completing all the fields on a new block (e.g., RANDAO reveal, signature) and
	publishing the block to a BN.
- Prompting the BN to produce a new shard attestation as per a validators
	duties.
- Ensuring that no slashable messages are signed by a validator private key.
- Keeping track of the system clock and how it relates to slots/epochs.

The HVC is capable of managing multiple validators in the same process tree.

## Implementation

[comment]: # (TODO: add short description of implementation)
_This section describes the present implementation of this HVC binary._

### Services

[comment]: # (TODO: add short description of services)
_Description._

#### `DutiesManagerService`

[comment]: # (TODO: add short description of services)
_Documentation_

#### `BlockProducerService`

[comment]: # (TODO: add short description of services)
_Documentation_

#### `AttestationProducerService`

[comment]: # (TODO: add short description of services)
_Documentation_

### Configuration

[comment]: # (TODO: short documentation of configuration for HVC)
_Documentation_

## BN Communication

[comment]: # (TODO: short documentation of communication with BN)
_Documentation_
