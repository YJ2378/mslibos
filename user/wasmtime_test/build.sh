$CPP test.cpp -o test.wasm -fno-exceptions -fno-rtti -ffast-math -funroll-loops -fomit-frame-pointer -Ofast

wasmtime compile --target x86_64-unknown-none -W threads=n,tail-call=n test.wasm

cargo build --target x86_64-unknown-none --release && cc \
  -Wl,--gc-sections -nostdlib \
  -Wl,--whole-archive \
  target/x86_64-unknown-none/release/libwasmtime_test.a \
  -Wl,--no-whole-archive \
  -shared \
  -o target/x86_64-unknown-none/release/libwasmtime_test.so

ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_test/target/x86_64-unknown-none/release/libwasmtime_test.so /home/wyj/alloy_stack/mslibos/target/release/libwasmtime_test.so
