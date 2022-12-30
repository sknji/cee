CFLAGS=-std=c11 -g -fno-common

build: clean fmt 
	cargo build
	cp target/debug/chibicc chibicc

test: build
	cargo test
	./test.sh

clean:
	rm -f chibicc *.o *~ tmp*

fmt:
	cargo fmt 


.PHONY: test clean build fmt 