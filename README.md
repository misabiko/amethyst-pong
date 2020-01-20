# Pong

First project using Rust, and consequently Amethyst, following directly on the [instruction book](https://book.amethyst.rs/stable/pong-tutorial.html).

## What I learned 
Things I've found outside of the tutorial.
- The tutorial tells you to add the line `let mut world: World`, but the world is actually created in `Application::new()`.

Things left to figure out.
- Some files import `Transform` from `amethyst::core::transform::Transform`, and some simply with `amethyst::core::Transform`, I haven't found how/why `Transform` is made available directly through `core`.

## How to run

To run the game, use

```
cargo run --features "vulkan"
```

on Windows and Linux, and

```
cargo run --features "metal"
```

on macOS.
