RFLAGS="-C link-arg=-s"

build: near_zk_demo1

near_zk_demo1: contracts/near_zk_demo1
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p near_zk_demo1 --target wasm32-unknown-unknown --release
	mkdir -p res
	cp target/wasm32-unknown-unknown/release/near_zk_demo1.wasm ./res/near_zk_demo1.wasm

circuit:
	npm install && cd circuits && make

test: near_zk_demo1 circuit
	cargo test -- --nocapture

clean:
	cargo clean
	rm -rf res/
	rm -rf node_modules/
	rm -rf circuits/out
