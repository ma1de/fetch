all:
	@echo "(1/2) Building the program"
	@cargo build --release
	@echo "(2/2) Running tests"
	@cargo test
	@echo "Done!"
