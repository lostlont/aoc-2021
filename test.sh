#!/usr/bin/env sh

for CRATE in *-tests
do
	cd $CRATE
	cargo build --verbose --all
	cargo test --verbose --all
	cd ..
done
