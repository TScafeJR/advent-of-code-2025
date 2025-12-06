##
## * Shell Specific Targets

## make sh-fmt; - format shell scripts
.PHONY: sh-fmt
sh-fmt:
	@shfmt -w ./scripts/*

