# wxRust

master: [![master build status](https://travis-ci.org/kenz-gelsoft/wxRust.png?branch=master)](https://travis-ci.org/kenz-gelsoft/wxRust)
/ rust-0.9: [![rust 0.9 build status](https://travis-ci.org/kenz-gelsoft/wxRust.png?branch=rust-0.9)](https://travis-ci.org/kenz-gelsoft/wxRust)
/ rust-0.8: [![rust 0.8 build status](https://travis-ci.org/kenz-gelsoft/wxRust.png?branch=rust-0.8)](https://travis-ci.org/kenz-gelsoft/wxRust)

This is a [Rust](http://www.rust-lang.org/) binding for the [wxWidgets cross platform toolkit](http://www.wxwidgets.org/).

## API

[wxRust API documentation](http://kenz-gelsoft.github.io/wxRust/)

## How it works

The wxRust library is heavily based on the [wxHaskell](http://www.haskell.org/haskellwiki/WxHaskell)'s wxc library.

The [wxc](https://github.com/wxHaskell/wxHaskell/tree/master/wxc) is a C language binding for the C++ wxWidgets toolkit.

We utilize the [rust-bindgen](https://github.com/crabtw/rust-bindgen) [![rust-bindgen build status](https://api.travis-ci.org/crabtw/rust-bindgen.png?branch=master)](https://travis-ci.org/crabtw/rust-bindgen) automatic rust binding generator for its [_unsafe](http://kenz-gelsoft.github.io/wxRust/src/wx/src/_unsafe.rs.html) low-level binding.

And we generate an OOP-style high-level binding (other modules than _unsafe) by [codegen.py code generator](https://github.com/kenz-gelsoft/wxRust/blob/rust-servo/src/codegen.py).

## Build

We use [CMake](http://www.cmake.org/) for cross platform build, but Windows platform is not yet tested.

For Linux build instructions, see [INSTALL.linux.md](INSTALL.linux.md)

### Build Prerequisite

Use following Rust compiler version for your wxRust branche. We're using Servo master's one for main development.

<table>
<thead>
<tr><td>wxRust branch</td><td>Supported Rust compiler version</td></tr>
</thead>
<tbody>
<tr><td>master</td><td>[master](https://github.com/mozilla/rust)</td></tr>
<tr><td>rust-0.9</td><td>[0.9](https://github.com/mozilla/rust/releases/tag/0.9)</td></tr>
<tr><td>rust-servo</td><td>[Servo master](https://github.com/mozilla/servo/) bundled version</td></tr>
<tr><td>rust-0.8</td><td>[0.8](https://github.com/mozilla/rust/releases/tag/0.8)</td></tr>
</tbody>
</table>

Install the wxWidgets 2.9.5 (or later) and CMake as below:

    brew install wxmac --devel
    brew install cmake

With some tweak you may be able to compile wxRust with a bit older versions (2.9.0 < x < 2.9.4) of wxWidgets. See [Issue 21](https://github.com/kenz-gelsoft/wxRust/issues/21#issuecomment-31661394).

### Build the library

Checkout git submodules:

    git submodule init # for the first time.
    git submodule update

At the project root directory:

    mkdir build
    cd build
    cmake ..
    make

### Compile and Run the Test program

At the CMake binary directory:

    make test && ./test

On Mac, Run as below:

    make Test.app
    open ./Test.app # or open in Finder

### Generate Documentation

At the CMake binary directory:

    make docs

Generates [a rustdoc documentation](http://kenz-gelsoft.github.io/wxRust/) under docs directory.
