##
## * Git Specific Targets

## make git-add-patch; - stage git changes via patch add command
.PHONY: git-add-patch
git-add-patch:
	@git add -p

