all: clean release

clean:
	rm -rf target/release

release:
	cargo build --release

debug:
	cargo build
