.PHONY: fmt
fmt:
	cargo +nightly fmt

.PHONY: clippy
clippy:
	cargo +nightly clippy -- -D warnings
