
all: test build

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all -- -D warnings

build:
	cargo build

test: build
	cargo test --all -- --nocapture 

watch-worker: 
	cargo watch -q -c -w worker/src/ -x 'run -p worker'

run-worker:
	cargo run -r -p worker

run-be:
	cargo run -r -p backend

install-subxt-cli:
	cargo install subxt-cli

generate-rpc:
	subxt metadata -f bytes --url wss://ws-testnet.gafi.network:443/  > worker/metadata.scale

rabbitmq-server:
	docker run -it --rm --name rabbitmq-stream-go-client-test \
		-p 5552:5552 -p 5672:5672 -p 15672:15672 \
		-e RABBITMQ_SERVER_ADDITIONAL_ERL_ARGS="-rabbitmq_stream advertised_host localhost" \
		--pull always \
		pivotalrabbitmq/rabbitmq-stream

mongodb: 
	docker run -it --rm --name mongo-test \
	-p 27107:27107 \
	mongo

help:
	cat Makefile