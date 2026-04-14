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
	./target/release/vm_translator ./vm/FunctionCalls/FibonacciElement/
	./target/release/vm_translator ./vm/FunctionCalls/NestedCall/
	./target/release/vm_translator ./vm/FunctionCalls/SimpleFunction/
	./target/release/vm_translator ./vm/FunctionCalls/StaticsTest/
	./target/release/vm_translator ./vm/ProgramFlow/BasicLoop/
	./target/release/vm_translator ./vm/ProgramFlow/FibonacciSeries/
