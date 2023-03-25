# Carmine API

API with endpoints to be used by the front end.

## Development

The workspace consists of three crates:

#### carmine-api

Server using [Actix](https://actix.rs/) with handlers.

#### carmine-api-cache

Struct holding all the data and methods to update them.

#### carmine-api-core

Types used by all crates.

#### carmine-api-starknet

Functions for retrieving data from the [Starknet](https://www.starknet.io/en) blockchain. There is a `Carmine` struct for directly retrieving data from the `carmine-protocol` and functionality for retrieving data from [Starkscan](https://starkscan.co/).
