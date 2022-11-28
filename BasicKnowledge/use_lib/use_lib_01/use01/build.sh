#!/usr/bin/env bash
rustc src/main.rs --extern addr=../lib01/target/debug/libaddr.rlib
