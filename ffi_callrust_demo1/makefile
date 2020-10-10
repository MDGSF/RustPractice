GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

run: clean build
	./target/debug/main

clean:
	$(CARGO_BIN) clean

build:
	$(CARGO_BIN) build
	$(GCC_BIN) -o ./target/debug/main ./c_src/main.cpp -Isrc -L ./target/debug -lcallrust

buildarm:
	$(CARGO_BIN) build --target=armv7-unknown-linux-gnueabihf
	arm-linux-gnueabihf-g++-7 -o ./target/armv7-unknown-linux-gnueabihf/debug/main ./c_src/main.cpp -Isrc -L ./target/armv7-unknown-linux-gnueabihf/debug -lcallrust
	arm-linux-gnueabihf-strip ./target/armv7-unknown-linux-gnueabihf/debug/main

