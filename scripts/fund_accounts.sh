#!/bin/bash

# Load variables from .env file
set -o allexport
source scripts/.env
set +o allexport

# -------------- #
# Initial checks #
# -------------- #
if [ -z "$PRIVATE_KEY" ] || [ -z "$ADDRESS" ]
then
    echo "You need to provide the PRIVATE_KEY and the ADDRESS of the deployer"
    exit 0
fi

(
    cd ~/arbitrum/nitro-testnode
    (. ./test-node.bash script send-l2 --to address_$ADDRESS --ethamount 5)
    (. ./test-node.bash script send-l2 --to address_$RECEIVER_ADDRESS --ethamount 3.14)
)

echo "Contracts funded"