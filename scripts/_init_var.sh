#!/bin/bash

if [ -z "$PARACHAIN_BUILD_FOLDER" ]; then
  PARACHAIN_BUILD_FOLDER="build/alphanet"
fi

mkdir -p $PARACHAIN_BUILD_FOLDER


if [ -z "$STANDALONE_BINARY" ]; then
  STANDALONE_BINARY="node/standalone/target/release/moonbase-standalone"
fi

if [ -z "$PARACHAIN_BINARY" ]; then
  PARACHAIN_BINARY="target/release/moonbase-alphanet"
fi

if [ -z "$STANDALONE_SPEC_TMP" ]; then
  STANDALONE_SPEC_TMP="/tmp/moonbase-standalone-specs-tmp.json"
fi

if [ -z "$ALPHANET_SPEC_TEMPLATE" ]; then
  ALPHANET_SPEC_TEMPLATE="specs/moonbase-alphanet-specs-template.json"
fi

if [ -z "$STAGENET_SPEC_TEMPLATE" ]; then
  STAGENET_SPEC_TEMPLATE="specs/moonbase-stagenet-specs-template.json"
fi

if [ -z "$ALPHANET_SPEC_PLAIN" ]; then
  ALPHANET_SPEC_PLAIN="$PARACHAIN_BUILD_FOLDER/moonbase-alphanet-specs-plain.json"
fi

if [ -z "$STAGENET_SPEC_PLAIN" ]; then
  STAGENET_SPEC_PLAIN="$PARACHAIN_BUILD_FOLDER/moonbase-stagenet-specs-plain.json"
fi

if [ -z "$ALPHANET_SPEC_RAW" ]; then
  ALPHANET_SPEC_RAW="$PARACHAIN_BUILD_FOLDER/moonbase-alphanet-specs-raw.json"
fi

if [ -z "$STAGENET_SPEC_RAW" ]; then
  STAGENET_SPEC_RAW="$PARACHAIN_BUILD_FOLDER/moonbase-stagenet-specs-raw.json"
fi

if [ -z "$ALPHANET_SPEC_TMP" ]; then
  ALPHANET_SPEC_TMP="/tmp/moonbase-alphanet-specs-tmp.json"
fi

if [ -z "$STAGENET_SPEC_TMP" ]; then
  STAGENET_SPEC_TMP="/tmp/moonbase-stagenet-specs-tmp.json"
fi

if [ -z "$PARACHAIN_WASM" ]; then
  PARACHAIN_WASM="$PARACHAIN_BUILD_FOLDER/moonbase-alphanet-runtime.wasm"
fi

if [ -z "$PARACHAIN_GENESIS" ]; then
  PARACHAIN_GENESIS="$PARACHAIN_BUILD_FOLDER/moonbase-alphanet-genesis.txt"
fi

if [ -z "$PARACHAIN_ID" ]; then
  PARACHAIN_ID=1000
fi

if [ -z "$POLKADOT_SPEC_TEMPLATE" ]; then
  POLKADOT_SPEC_TEMPLATE="specs/rococo-alphanet-specs-template.json"
fi

if [ -z "$POLKADOT_SPEC_PLAIN" ]; then
  POLKADOT_SPEC_PLAIN="$PARACHAIN_BUILD_FOLDER/rococo-alphanet-specs-plain.json"
fi

if [ -z "$POLKADOT_SPEC_RAW" ]; then
  POLKADOT_SPEC_RAW="$PARACHAIN_BUILD_FOLDER/rococo-alphanet-specs-raw.json"
fi

if [ -z "$POLKADOT_SPEC_TMP" ]; then
  POLKADOT_SPEC_TMP="/tmp/rococo-alphanet-specs-tpm.json"
fi


if [ -z "$USER_PORT" ]; then
  USER_PORT=30000
fi

if [ -z "$RELAY_IP" ]; then
  RELAY_IP="127.0.0.1"
fi

if [ -z "$PARACHAIN_IP" ]; then
  PARACHAIN_IP="127.0.0.1"
fi

# Alice private key
if [ -z "$SUDO_SEED" ]; then
  SUDO_SEED="0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a"
fi

NODE_KEYS=(
  0000000000000000000000000000000000000000000000000000000000000000
  1111111111111111111111111111111111111111111111111111111111111111
  2222222222222222222222222222222222222222222222222222222222222222
)

RELAY_SEEDS=(
  "group affair stuff market convince bundle merge share vague token focus total"
  "concert one novel maze joke fossil absent universe dignity near tank layer"
  "six stumble adjust guide delay rail waste ivory hood peasant seminar fluid"
)

WELL_KNOWN_USERS=(
  "alice"
  "bob"
  "charlie"
  "dave"
  "eve"
  "ferdie"
)

RELAY_LOCAL_IDS=(
  12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN
  12D3KooWPqT2nMDSiXUSx5D7fasaxhxKigVhcqfkKqrLghCq9jxz
  12D3KooWLdJAwPtyQ5RFnr9wGXsQzpf3P2SeqFbYkqbfVehLu4Ns
)

PARACHAIN_KEYS=(
  0000000000000000000000000000100000000000000000000000000000000000
  0000000000000000000000000000111111111111111111111111111111111111
  0000000000000000000000000000122222222222222222222222222222222222
  0000000000000000000000000000133333333333333333333333333333333333
  0000000000000000000000000000144444444444444444444444444444444444
  0000000000000000000000000000155555555555555555555555555555555555
  0000000000000000000000000000166666666666666666666666666666666666
  0000000000000000000000000000177777777777777777777777777777777777
  0000000000000000000000000000188888888888888888888888888888888888
  0000000000000000000000000000199999999999999999999999999999999999
)

PARACHAIN_LOCAL_IDS=(
  12D3KooWJ8BACzWs2x7yt1UozRrZrtcgUhpJrBbYrYqAQBa1ZXFN
  12D3KooWAZsNtba8pPJMYdbUUr9KFLQbFioPZKbpdNnbouSk6tto
  12D3KooWK5yfHLPADzZEKKzyqHjf3y5bi6jP6egNZL7aYp4us52E
  12D3KooWHtc11BNuNo9CbHb7MQS7NDue1qRjgnHbyyY8bo3Fu2L3
  12D3KooWDoaPdMd86ZPRTR9tmdw1qcdkdzLLWAXZSg5r4qWcyvZF
  12D3KooWHXyX7DHVr8nRHfbu7ct6Zyy4NEhciAzG6XqHRFC3zHEs
  12D3KooWAUHzje7vqkwBUTkiK8LMrMi52qgfyb8ouVpefo3tSXnk
  12D3KooWRjUyXkjqbmZ5XUiwrSz4fPrP3Q4wkhq7b2FZD8cRmDkm
  12D3KooWJ2sbvqxSA8D3VYa33XhhkmuZt58nxdWMKo2z3Mf3mafa
  12D3KooWNdXzpEDHHt6gnWx6RXc8J8YFZMhjQ154GDGHQBDNZZ9V
)
