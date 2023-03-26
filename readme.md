# wx-rs

[![Crates.io](https://img.shields.io/crates/v/wx-rs)](https://crates.io/crates/wx-rs)
[![Docs.rs](https://docs.rs/wx-rs/badge.svg)](https://docs.rs/wx-rs)

*This is an alpha release. Don't expect much from me.*

**Only builds on the MSVC toolchain right now**

An extremely partial set of bindings to wxWidgets, for Rust. The intent is that you can import this and all the bits of wxWidgets provided are statically linked in. Implements [`HasRawWindowHandle`](https://crates.io/crates/raw-window-handle). Callbacks are used for rendering and event handling. Features:
- Window initialization
- Keyboard, mouse events
- Clipboard support
- Native menus
- Cursor selection

Some things that'd be great to have:
- The build script is currently highly deficient. It relies on pre-compiled instances of wxWidgets existing in the repo found in `dist/`. So if this does not compile, you need to go in there and compile it. The build script should be better than this.
- Icon support for Windows/Linux


## Building
The approach this crate takes is to download wxWidgets into the `./dist` directory, then build it. This means that you need to have a toolchain capable of compiling wxWidgets.

Why are we doing this instead of using submodules? Because with submodules cargo would naturally check out files, but this isn't desirable since we want cargo to ignore these files when packaging (because wxWidgets is huge: The package, when you trim out all non-source files is still around 100MB).

Why do we save/compile to `dist` rather than cargo's preferred `OUT_DIR`? Because with the latter, we'd have to recompile wxWidgets more frequently. Since it takes quite a while to do so, it's much nicer to only have to compile once and then have the artifact around forever.

If you want to do a fresh install, just `rm -R ./dist`.

# Building wxWidgets

## Windows

### Using the MSVC toolchain

Make sure you're using ""x64 Native Tools Command Prompt for VS 2019"", and that cargo is in your path when compiling for the first time. After it's built, msys works fine to continue building things from the Rust side.
- https://stackoverflow.com/questions/11065421/command-prompt-wont-change-directory-to-another-drive

### Using the GNU toolchain on Windows
This depends on nightly rust to build correctly

#### Getting MSYS2
https://www.msys2.org/

```
$ pacman -Syu
$ pacman -S mingw-w64-x86_64-gcc mingw-w64-x86_64-make mingw-w64-x86_64-libexpat mingw-w64-x86_64-zlib msys2-w32api-runtime pacman -S mingw-w64-x86_64-lld
```

#### Building
This is a note of what needs to be integrated into the build script.

In a MSYS2 MinGW64 terminal:

```
$ cd dist/wxWidgets
$ mkdir msw64-release-build
$ ../configure --with-opengl --disable-shared
$ make -j20
```

### Using the OSX toolchain
... why aren't I documented? :(
