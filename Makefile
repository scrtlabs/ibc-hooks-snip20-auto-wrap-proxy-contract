all: src/contract.rs src/lib.rs src/msg.rs Cargo.toml Cargo.lock
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
	cat ./target/wasm32-unknown-unknown/release/ibc_hooks_snip20_auto_wrap_proxy_contract.wasm | gzip -9 > ./ibc-hooks-snip20-auto-wrap-proxy-contract.wasm.gz

clean:
	cargo clean
	-rm -f ./ibc-hooks-snip20-auto-wrap-proxy-contract.wasm.gz