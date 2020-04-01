.PHONY: release

src/games/wheel/payouts.rs: ./wheel-payouts-to-rust-arr.js
	node ./wheel-payouts-to-rust-arr.js > src/games/wheel/payouts.rs

release:
	# we have to build from scratch to ensure version in binary will be
	# updated
	rm -rf target/;
	cargo release $@;
