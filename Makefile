all:
	@echo "(1/3) Building the program"
	@cargo build --release
	@echo "(2/3) Running tests"
	@cargo test
	@echo "(3/3) Copying fetch to /usr/bin"
	@cp target/release/fetch /usr/bin/
	@echo "Done!"
