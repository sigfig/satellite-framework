## build automation

currently, cargo/rustc does not properly support building executables using a
gcc toolchain. i noticed that many of the arguments in the target specification
are outright ignored when attempting to link an executable binary. it behaves
correctly when linking static libraries, so for now we do that instead, and link
executables manually

## config templating

currently, rtems is configured using a single `config.c` file that imports the
api and specifies the kernel parameters. in the future, we may want to generate
this file from some custom configuration format instead. that can be done fairly
easily by extending `build.rs`

## config build setup

to find the platform configuration options in the build script, i use the
pkgconf files provided by rtems for its targets. for that to work properly you
need to set `PKG_CONFIG_PATH` to wherever `6/lib/pkgconfig` is in your rtems
distribution
