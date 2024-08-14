# Building from source.

Prerequisites: You will need a rust compiler, together with its default dependency manager, "cargo".

 1. Download the sources
 ```
 git clone https://github.com/matrix-org/vodozemac-bindings.git
 ```

 2. Change to the directory with the c++ bindings.
 ```
 cd vodozemac-bindings/cpp
 ```

 3. Compile
 ```
 cargo build --release
 ```

This will produce:

 1. a c++ header at
`../target/cxxbridge/vodozemac/src/lib.rs.h`

 2. a c++ source file that represents the C++ side of the bindings at
`../target/cxxbridge/vodozemac/src/lib.rs.cpp`

 3. a static library that represents the Rust side of the bridge at
 ../target/release/libvodozemac.a

# Usage.

Please, take a look at the `.cpp` files inside the `tests` directory for the examples.
