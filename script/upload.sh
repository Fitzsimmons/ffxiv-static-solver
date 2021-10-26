#!/bin/bash

set -euo pipefail

(
	SCRIPT_RELATIVE_DIR=$(dirname "${BASH_SOURCE[0]}") 
	cd $SCRIPT_RELATIVE_DIR/..

	script/build.sh

	cd example-frontend

	aws --profile fitzsimmons --region us-east-1 s3 sync --delete . s3://ffxiv-static-solver-production
)
