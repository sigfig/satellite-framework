# untitled satellite framework

builds on top of rtems. this project is very much a work in progress.

## overview

```
- armv7-rtems-eabihf.json         | hardware target specification
- crates                          |
  - rtems                         | rtems classic api bindings
    - ffi                         | unsafe bindings
- src                             |
  - io                            | convenience abstraction for kernel io
  - config.c                      | rtems kernel configuration
  - init.rs                       | application entry point
  - panic.rs                      | crashpad handler
```

## prerequisites

this framework builds on top of [rtems](https://www.rtems.org/) and requires an
appropriate cross-compiler toolchain and board support package available. please
consult the [manual](https://docs.rtems.org/branches/master/user/index.html) for
instructions on how to do this.
