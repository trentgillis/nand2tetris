.DEFAULT_GOAL := run

.PHONY:fmt vet build

fmt:
	go fmt ./...

vet: fmt
	go vet ./...

build:
	cargo build

build-release:
	cargo build --release

test:
	cargo test

run: build-release
	./target/release/vm_translator ./vm/StackArithmetic/SimpleAdd/
	./target/release/vm_translator ./vm/StackArithmetic/StackTest/
	./target/release/vm_translator ./vm/MemoryAccess/BasicTest/
	./target/release/vm_translator ./vm/MemoryAccess/PointerTest/
	./target/release/vm_translator ./vm/MemoryAccess/StaticTest/
