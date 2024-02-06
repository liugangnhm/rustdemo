gcc -Wall -g -O0 test.c -I. -Ltarget/debug/ -lrustdemo
RUST_BACKTRACE=1 LD_LIBRARY_PATH=target/debug ./a.out

## with rpath

gcc -Wall -g -O0 test.c -I. -L./target/debug/ -lrustdemo "-Wl,-rpath=./target/debug/"