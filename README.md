# Carmine API

API with endpoints to be used by the front end.

## Development

For local development, create `.env` file in the root with the following variables:

```
NETWORK=testnet
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

## API Endpoints

Path uses two variables: `network` and `pool`.

Allowed values for `network`:

- `mainnet`
- `testnet`

Allowed values for `pool`:

- `eth-usdc-call`
- `eth-usdc-put`

###### /api/v1/{network}/live-options

Options that can be currently traded with premia for size 1.

###### /api/v1/{network}/all-transactions

All events that are currently stored in the database.

###### /api/v1/{network}/transactions?address={user_address}

All events triggered by the `user_address`.

###### /api/v1/{network}/airdrop?address={user_address}

If the `user_address` is eligible for an airdrop, this endpoint returns address, amount of tokens and hashes to produce Merkel tree root.

###### /api/v1/{network}/option-volatility

All options with volatility historic data.

###### /api/v1/mainnet/{pool}

Historic data of pool state for the given pool - mainnet only.

###### /api/v1/mainnet/{pool}/state

Last pool state for the given pool - mainnet only.

###### /api/v1/mainnet/{pool}/apy

APY of the given pool - mainnet only.

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
