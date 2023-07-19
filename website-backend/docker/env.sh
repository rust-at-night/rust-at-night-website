#!/bin/bash

# Please only source this file, don't execute it directly.

AWS_ACCOUNT_ID="012345678901"
AWS_REGION="eu-central-1"
ORG_NAME="rust-at-night"
IMAGE_NAME="website-backend"
CURRENT_DATE=$(date +%Y%m%d)

export AWS_ACCOUNT_ID
export AWS_REGION
export ORG_NAME
export IMAGE_NAME
export CURRENT_DATE
