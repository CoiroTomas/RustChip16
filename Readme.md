RustChip16
==========

This is a Rust implementation of Chip16.

It currently doesn't have any sound code.

You can build it with:

```
cargo build --release
```

You can run it with:

```
cargo run PATH SCREEN_MULTIPLIER(optional) --release
```

The path is a path to the program you want to run, either a .bin or .c16 file.
The multiplier is the number of pixels you want each emulator pixel to take.

The controls for the first controller are:

Arrows to move, Right shift as select, Return as pause, Numpad7 as A and Numpad9 as B

The controls for the second controller are:

WASD to move, Left control as select, Space as pause, H as A and J as B


You can read more about Chip16 in here:

https://github.com/chip16/chip16
