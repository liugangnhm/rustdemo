gcc -Wall -g -O0 test.c -I. -Ltarget/debug/ -lrustdemo -o target/debug/test.exe

if [ $? -eq 0 ]; then
    echo "Build successful"
else
    echo "Build failed"
    exit 1
fi

export RUST_BACKTRACE=1

./target/debug/test.exe