RustChip16
==========

This is a little Rust implementation of Chip16.

It currently barely boots and shows some images, I am working through my bugs.

The non-Rust dependencies are:
SDL2
freetype-6

You can build it with:

```
cargo build --release
```

The only argument the program needs now is the path to a ROM, which can be a .bin or a .c16

