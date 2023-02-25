rust:
	@cargo build --release

uniffi-build:
	@cd uniffi && cargo build --release

python: uniffi-build
	@cd uniffi && cargo run --release --bin uniffi-bindgen generate src/base.udl --out-dir generated --language $@

python-shell: python
	@cp uniffi/target/release/libbase_uniffi.dylib uniffi/generated/libuniffi_base.dylib
	@cd uniffi/generated/ && python3

clean:
	@cargo clean
	@cd uniffi && cargo clean