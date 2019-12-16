build:
	cargo build
	ln -sf ./target/debug/puzzle ./puzzle

clean:
	cargo clean
	rm ./puzzle
