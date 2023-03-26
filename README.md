# Carmine API

API with endpoints to be used by the front end.

## Development

For local development, create `.env` file in the root with the following variables:

```
ENVIRONMENT=local
STARKSCAN_API_KEY=your_api_key_goes_here
```

And then run dev mode with Cargo:

```
cargo run -p carmine-api
```

To test `Docker` build, which runs in the production, set `STARKSCAN_API_KEY` in the `docker-compose.yaml` file and then run:

```
docker compose up
```

## Workspace

The workspace consists of four crates:

#### carmine-api

Server using [Actix](https://actix.rs/) with handlers.

#### carmine-api-cache

Struct holding all the data and methods to update them.

#### carmine-api-core

Types used by all crates.

#### carmine-api-starknet

Functions for retrieving data from the [Starknet](https://www.starknet.io/en) blockchain. There is a `Carmine` struct for directly retrieving data from the `carmine-protocol` and functionality for retrieving data from [Starkscan](https://starkscan.co/).
