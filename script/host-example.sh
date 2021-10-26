#!/bin/bash

set -euo pipefail

(
	SCRIPT_RELATIVE_DIR=$(dirname "${BASH_SOURCE[0]}") 
	cd $SCRIPT_RELATIVE_DIR/../example-frontend

	python -m http.server
)
