#!/usr/bin/env sh

for CRATE in *-tests
do
	cd $CRATE
	cargo build --verbose --all || exit 1
	cargo test --verbose --all || exit 1
	cd ..
done
