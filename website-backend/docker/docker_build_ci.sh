#!/bin/bash
set -euxo pipefail

SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
SCRIPT_DIR="$(dirname "$SCRIPT_PATH")"

source "$SCRIPT_DIR/env.sh"

DOCKER_BUILDKIT=1 docker build -f docker/Dockerfile -t "${AWS_ACCOUNT_ID}".dkr.ecr."${AWS_REGION}".amazonaws.com/"${ORG_NAME}"."${IMAGE_NAME}"."${CURRENT_DATE}":git-"${GITHUB_SHA}"
