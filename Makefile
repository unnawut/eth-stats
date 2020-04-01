ETH_CLIENT_URL ?= http://localhost:8545

GETH_SHARED_ARGS := --syncmode light \
		--rpc \
		--rpcaddr "0.0.0.0" \
		--rpcport 8545 \
		--rpcapi "eth,net,web3"

#
# Run the entire process
#

start-run: start run

.PHONY: start-run

#
# Service provisioning
#

start: start-ropsten

start-ropsten:
	docker run \
	-v "$(pwd)":/root \
	-p 8545:8545 \
	-p 30303:30303 \
	ethereum/client-go \
		$(GETH_SHARED_ARGS) \
		--testnet

start-rinkeby:
	docker run \
	-v "$(pwd)":/root \
	-p 8545:8545 \
	-p 30303:30303 \
	ethereum/client-go \
		$(GETH_SHARED_ARGS) \
		--rinkeby

start-mainnet:
	docker run \
	-v "$(pwd)":/root \
	-p 8545:8545 \
	-p 30303:30303 \
	ethereum/client-go \
		$(GETH_SHARED_ARGS)

clean:
	rm -rf .ethereum/

clean-start: clean start

.PHONY: start start-ropsten start-rinkeby clean clean-start

#
# Building the tool
#

build:
	cargo build

.PHONY: build

#
# Running the tool
#

run:
	cargo run

.PHONY: run

#
# Direct JSON-RPC requests. Useful for quick debugging.
#

latest-block:
	curl -X POST ${ETH_CLIENT_URL} \
	-H "Content-Type: application/json" \
	--data '{ \
		"jsonrpc":"2.0", \
		"method":"eth_blockNumber", \
		"params":[], \
		"id":1 \
	}' | jq

syncing:
	curl -X POST ${ETH_CLIENT_URL} \
	-H "Content-Type: application/json" \
	--data '{ \
		"jsonrpc":"2.0", \
		"method":"eth_syncing", \
		"params":[], \
		"id":1 \
	}' | jq

.PHONY: latest-block syncing
