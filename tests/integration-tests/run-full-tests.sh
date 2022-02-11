#!/usr/bin/env bash
set -e

SCRIPT_PATH="$(dirname "${BASH_SOURCE[0]}")"
cd $SCRIPT_PATH

set -a
. ../../.env
set +a

CONTAINER_ID=$(./run-test-node-docker.sh)


function cleanup() {
    docker logs ${CONTAINER_ID} --tail 15
    docker-compose -f ../../docker-compose.yml down -v
}

trap cleanup EXIT

sleep 3

# Display runtime version
yarn workspace api-scripts tsnode-strict src/status.ts | grep Runtime

# Start a query-node
../../query-node/start.sh

# Setup storage & distribution
HOST_IP=$(./get-host-ip.sh)
export COLOSSUS_1_URL="http://${HOST_IP}:3333"
export COLOSSUS_1_TRANSACTOR_KEY=$(docker run --rm --pull=always docker.io/parity/subkey:2.0.1 inspect ${COLOSSUS_1_TRANSACTOR_URI} --output-type json | jq .ss58Address -r)
export DISTRIBUTOR_1_URL="http://${HOST_IP}:3334"
./run-test-scenario.sh initStorageAndDistribution

# Start colossus & argus
docker-compose -f ../../docker-compose.yml up -d colossus-1
docker-compose -f ../../docker-compose.yml up -d distributor-1

# Run full tests reusing the existing keys
REUSE_KEYS=true IGNORE_HIRED_LEADS=true ./run-test-scenario.sh full
