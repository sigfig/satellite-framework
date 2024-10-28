this module contains unsafe rust api wrappers for the rtems classic api

it is currently organized along the same lines as the rtems api documentation

when possible, we try to follow the api exactly, with a few exceptions:
- we do not include any of the datatype utility functions
- we do not replicate the functionality of any macros
