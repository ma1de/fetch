all:
	@echo "(1/5) Building the program"
	@cargo build
	@echo "(2/5) Running tests"
	@cargo test
	@echo "(3/3) Setting up man page for fetch"
	@sudo mv target/debug/fetch /usr/bin
	@sudo cp fetch /usr/local/man/man1/fetch.1
	@sudo gzip /usr/local/man/man1/fetch.1
	@echo "Done!"
