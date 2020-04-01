# eth-stats

A basic Ethereum statistical analysis primarily made for me to learn Rust.

## Prerequisites

- Rust
- An accessible Ethereum client with JSON-RPC enabled

## Usage

1. Start a geth instance in docker. This step is not needed if you already have one running or using Infura.

    ```shell
    # Default to starting for Ropsten
    $ make start

    # Start for Ropsten
    $ make start-ropsten

    # Start for Rinkeby
    $ make start-rinkeby

    # Start for Mainnet
    $ make start-mainnet
    ```

2. Run the app.

    ```shell
    $ ETH_CLIENT_URL=http://localhost:8545 \
      NUM_BLOCKS=1000 \
      make run
    ```
