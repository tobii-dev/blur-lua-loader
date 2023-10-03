# blur-lua-loader
Blur Plugin to run custom Lua code in the Blur process.

## Build

```bat
LUA_LIB_NAME=lib/lua5.1 LUA_LINK=cdylib cargo build --release --target=i686-pc-windows-msvc --features minhook
COPY /Y "target\i686-pc-windows-msvc\release\lua_hooks.dll" "<BLUR>\amax\dlls\lua_hooks.asi"
```
