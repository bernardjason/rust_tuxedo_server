CFLAGS=-I$(TUXDIR)/include -L tuxedorust/target/release

tuxedorust/target/release/libtuxedorust.so: tuxedorust/src/lib.rs
	cd tuxedorust && cargo test && cargo build --release

src/c_upper.o:src/c_upper.c
	cc $(CFLAGS) -o bin/c_upper.o -c src/c_upper.c 

bin/c_server:src/c_upper.o
	CFLAGS="$(CFLAGS)" buildserver -o bin/c_server -s c_upper -f bin/c_upper.o

bin/rust_server:tuxedorust/target/release/libtuxedorust.so
	CFLAGS="$(CFLAGS)" buildserver -o bin/rust_server -s rust_upper -l-ltuxedorust

all:src/c_upper.o tuxedorust/target/release/libtuxedorust.so bin/c_server bin/rust_server

clean:
	cd tuxedorust && cargo clean
	rm -f src/c_upper.o bin/upper_case_c_and_rust bin/rust_server bin/c_server
