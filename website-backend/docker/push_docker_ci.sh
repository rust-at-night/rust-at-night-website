#!/bin/bash

SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
SCRIPT_DIR="$(dirname "$SCRIPT_PATH")"

source "$SCRIPT_DIR/env.sh"

docker push "${AWS_ACCOUNT_ID}".dkr.ecr."${AWS_REGION}".amazonaws.com/"${ORG_NAME}"."${IMAGE_NAME}"."${CURRENT_DATE}":git-"${GITHUB_SHA}"
