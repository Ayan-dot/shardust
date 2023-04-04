program := shardust

build:
	cargo build 

test:
	cargo test

help:
	@echo "usage: make [build|test]"