#!/bin/bash

(
	SCRIPT_RELATIVE_DIR=$(dirname "${BASH_SOURCE[0]}") 
	cd $SCRIPT_RELATIVE_DIR/..

	version=$(toml get Cargo.toml package.version | sed s/\"//g)

	script/build.sh

	cd example-frontend

	zip -r ../pkg/ffxiv-static-solver-$version.zip ffxiv-static-solver -x ffxiv-static-solver/.gitignore
)
