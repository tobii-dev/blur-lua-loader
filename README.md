# blur-lua-loader
Blur Plugin to run custom Lua code in the Blur process.

## Build
```bat
cargo +nightly build --release --target=i686-pc-windows-msvc
COPY /Y "target\i686-pc-windows-msvc\release\lua_hooks.dll" "<BLUR>\amax\dlls\lua_hooks.asi"
```
