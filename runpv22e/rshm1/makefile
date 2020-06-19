GCC_BIN ?= $(shell which g++)
CARGO_BIN ?= $(shell which cargo)

runserver:
	export LD_LIBRARY_PATH=$(PWD)/../target/debug && ../target/debug/server

runclient:
	export LD_LIBRARY_PATH=$(PWD)/../target/debug && ../target/debug/client

clean:
	$(CARGO_BIN) clean

build:
	$(CARGO_BIN) build
	$(GCC_BIN) -g -o ../target/debug/server ./c_src/server.cpp \
		-Isrc \
		-L ../target/debug -lrshm1
	$(GCC_BIN) -g -o ../target/debug/client ./c_src/client.cpp \
		-Isrc \
		-L ../target/debug -lrshm1
