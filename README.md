# sixel
The pure sixel image encoder

### Note
The binary application currently supports only RGB (without a transparent) png images.
The library takes a simple RGB image (from buffer) as input.

### Build
Just type a:
```
cargo r
```
..to see an image. Your terminal has to be supported [sixel](https://en.wikipedia.org/wiki/Sixel) format.

Try to display a custom image:
```
cargo r -- image.png
```
