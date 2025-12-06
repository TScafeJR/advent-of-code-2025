##
## * Rust Specific Targets

## make rust-test; - run rust tests
.PHONY: rust-test
rust-test:
	@cargo test --release

## make rust-debug; - run rust in debug mode
.PHONY: rust-debug
rust-debug:
	@cargo test -- --nocapture

.PHONY: rust-fmt
rust-fmt:
	@cargo fmt
