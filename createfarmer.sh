#!/bin/bash

# Define variables
CONTRACT_ID="pointzero.testnet"
FARMER_NEAR_NAME="ChristineK"
GROUP_NAME="Genesis"

# Assign a NEAR name to the farmer
near call $CONTRACT_ID assign_near_name '{"member_initials": "CK", "group_name": "'$GROUP_NAME'"}' --accountId $CONTRACT_ID

# Create a custodial wallet for the farmer
near call $CONTRACT_ID create_custodial_wallet '{"farmer_near_name": "'$FARMER_NEAR_NAME'.'$GROUP_NAME'.antugrow.near"}' --accountId $CONTRACT_ID

# Check if the wallet has been created
# This step depends on how you implement the wallet creation and verification in your contract.
# For demonstration, let's assume you have a method `wallet_exists` that checks if a wallet exists.
near call $CONTRACT_ID wallet_exists '{"wallet_name": "'$FARMER_NEAR_NAME'.custodial"}' --accountId $CONTRACT_ID
