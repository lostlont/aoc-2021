#!/usr/bin/env sh

for CRATE in *-tests
do
	cargo build --verbose --all
	cargo test --verbose --all
done
