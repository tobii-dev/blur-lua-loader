# blur-lua-loader
DLL to run custom Lua code inside the Blur.exe process

## Build
```
cargo +nightly build --release --target=i686-pc-windows-msvc
```
COPY: "target\i686-pc-windows-msvc\release\lua_hooks.dll" "<Path to Blur>\Blur\amax\dlls\lua_hooks.asi"
