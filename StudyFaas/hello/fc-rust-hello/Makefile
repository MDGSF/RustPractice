build-img:
	docker build -t fc-rust-env -f ./fc_rust_env/Dockerfile .

build: build-img
	docker run --rm -it -v ${PWD}:/opt fc-rust-env bash -c "cd /opt/code/hello && /root/.cargo/bin/cargo build"

deploy: build
	s deploy -y