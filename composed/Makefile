uniffi-build:
	@cd uniffi && cargo build --release -q

python: uniffi-build
	@cd uniffi && cargo run --release --bin uniffi-bindgen generate src/composed.udl --out-dir generated --language $@

python-shell: python
	@cp ../base/uniffi/generated/*.dylib uniffi/generated
	@cp ../base/uniffi/generated/*.py uniffi/generated
	@cp uniffi/target/release/libcomposed.dylib uniffi/generated/libuniffi_composed.dylib
	@cd uniffi/generated/ && python3

clean:
	@cargo clean
	@cd uniffi && cargo clean
	@rm -rf uniffi/generated
