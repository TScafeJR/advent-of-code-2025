.PHONY: fmt
fmt: sh-fmt rust-fmt

.PHONY: fix
fix: fmt

.PHONY: gen-day
gen-day:
	@./scripts/gen-day

.PHONY: test
test: rust-test

include makefiles/*.mk
