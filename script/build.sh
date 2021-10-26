#!/bin/bash

set -euo pipefail

(
	SCRIPT_RELATIVE_DIR=$(dirname "${BASH_SOURCE[0]}") 
	cd $SCRIPT_RELATIVE_DIR/..

	wasm-pack build --target no-modules --out-dir example-frontend/ffxiv-static-solver
	cp -rv js/*.js example-frontend/ffxiv-static-solver

	cd example-frontend/ffxiv-static-solver
	rm package.json README.md
)
