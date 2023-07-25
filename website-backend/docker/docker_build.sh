#!/bin/bash
set -euxo pipefail

SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
SCRIPT_DIR="$(dirname "$SCRIPT_PATH")"

source "$SCRIPT_DIR/env.sh"

if [[ $(uname -m) == 'arm64' ]]; then
    DOCKER_BUILDKIT=1 docker build -f docker/Dockerfile -t "${AWS_ACCOUNT_ID}".dkr.ecr."${AWS_REGION}".amazonaws.com/"${ORG_NAME}"."${IMAGE_NAME}" --platform=linux/arm64/v8 .
else
    DOCKER_BUILDKIT=1 docker build -f docker/Dockerfile -t "${AWS_ACCOUNT_ID}".dkr.ecr."${AWS_REGION}".amazonaws.com/"${ORG_NAME}"."${IMAGE_NAME}" .
fi
