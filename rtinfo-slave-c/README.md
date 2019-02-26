# Build
The Makefile will compiles the project, using `jansson`, `hiredis` and `curl` as dependencies.
The `hiredis` and `jansson` will be compiled statically on the binary, all the rest will be dynamically linked.
