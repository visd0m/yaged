### yaged (yet another gif encoder decoder)

Gif encoder/decoder based on [GIF89a specification](https://www.w3.org/Graphics/GIF/spec-gif89a.txt).

#### Examples

Decode a gif file using `ColorMap` color output mode.
```rust
let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
let gif = decode(file, ColorOutput::ColorMap).unwrap();
```

Decode a gif file using `RGBA` color output mode.
```rust
let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
let gif = decode(file, ColorOutput::RGBA).unwrap();
```

#### Todos

- [ ] handle interlaced flag
- [ ] handle disposal method
- [ ] handle user input
- [ ] decoding optimization
- [ ] implements gif encoding