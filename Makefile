.PHONY: release

all: cli gui

help:
	@echo ""
	@echo "make                                 - Build binaries files"
	@echo "make gui                             - Build only GUI binary files"
	@echo "make cli                             - Build only CLI binary files"
	@echo "make release         				- Build release packages"
	@echo "make precommit                       - Execute precommit steps"
	@echo "make clean                           - Clean"
	@echo "make loc                             - Count lines of code in src folder"
	@echo ""

gui:
	cargo build -p coinstr --release

cli:
	cargo build -p coinstr-cli --release

release:
	cd contrib/release && make

dev-gui:
	cargo fmt --all && STDOUT_LOG=true cargo run -p coinstr -- --testnet

precommit:
	@bash .githooks/pre-push

clean:
	cargo clean

loc:
	@echo "--- Counting lines of .rs files (LOC):" && find bindings/ crates/ -type f -name "*.rs" -exec cat {} \; | wc -l
