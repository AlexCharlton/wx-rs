# wx-rs

*This is an alpha release. Don't expect much from me.*

An extremely partial set of bindings to wxWidgets, for Rust. The intent is that you can import this and all the bits of wxWidgets provided are statically linked in. Implements [`HasRawWindowHandle`](https://crates.io/crates/raw-window-handle). Callbacks are used for rendering and event handling. Features:
- Window initialization
- Keyboard, mouse events
- Clipboard support
- Native menus
- Cursor selection

Some things that'd be great to have:
- The build script is currently highly deficient. It relies on pre-compiled instances of wxWidgets existing in the repo found in `dist/`. So if this does not compile, you need to go in there and compile it. The build script should be better than this.
- Icon support for Windows/Linux


## Building from source
```
git submodule init && git submodule update
```

Before using cargo.

# Building wxWidgets

## Windows

### Using the MSVC toolchain

Using ""x64 Native Tools Command Prompt for VS 2019""
- https://stackoverflow.com/questions/11065421/command-prompt-wont-change-directory-to-another-drive

```
$ cd dist/wxWidgets
$ git submodule update --init
$ cd build/msw
$ nmake /f makefile.vc BUILD=release TARGET_CPU=X64
```

For the sake of Github's file size limits:
```
$ cd dist/wxWidgets/vc_x64_lib
$ gzip wxmsw31u_core.lib
```

### Using the GNU toolchain
This depends on nightly rust to build correctly

#### Getting MSYS2
https://www.msys2.org/

```
$ pacman -Syu
$ pacman -S mingw-w64-x86_64-gcc mingw-w64-x86_64-make mingw-w64-x86_64-libexpat mingw-w64-x86_64-zlib msys2-w32api-runtime pacman -S mingw-w64-x86_64-lld
```

#### Building

In a MSYS2 MinGW64 terminal:

```
$ cd dist/wxWidgets
$ mkdir msw64-release-build
$ ../configure --with-opengl --disable-shared
$ make -j20
```

Changes can then be committed and pushed to the upstream:
```
git push origin HEAD:release-build
```

### Using the OSX toolchain
... why aren't I documented? :(
