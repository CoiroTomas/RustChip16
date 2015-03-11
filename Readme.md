RustChip16
==========

This is a Rust implementation of Chip16.

It currently mostly works with some glitches and no sound, and control support is really slow.

The non-Rust dependencies are:
SDL2
freetype-6

You can build it with:

```
cargo build --release
```

You can run it with:

```
cargo run PATH SCREEN_MULTIPLIER --release
```

The path is a path to the program you want to run, either a .bin or .c16 file.
The multiplier is the number of pixels you want each emulator pixel to take.

The controls for the first controller are:

Arrows to move, Right shift as select, Return as pause, Numpad7 as A and Numpad9 as B

The controls for the second controller are:

WASD to move, Left control as select, Space as pause, H as A and J as B