GCC_BIN ?= $(shell which g++)
CARGO_BIN ?= $(shell which cargo)

all:
	$(CARGO_BIN) build
	$(GCC_BIN) -o ./target/debug/main ./c_src/main.cpp -Isrc \
		-L ./target/debug -lffi_func_pointer_1



