<header>
<h1 align="center">jittermouse</h1>
<p align="center">a simple rust program to move the mouse around every few seconds</p>
</header>
<br />
Jittermouse is a work-in-progress rust program that moves the mouse around every 9-10 seconds using the <a href="https://crates.io/crates/enigo">Enigo</a> library. It is still very incomplete, and has many features and customizations missing.

Because this program uses Enigo, Linux users may have to install libxdo/xdotool. See <a href="https://github.com/enigo-rs/enigo#runtime-dependencies">here</a> for how to install libxdo/xdotool.

<h2>Building</h2>
To build jittermouse, assuming you already have <a href="https://www.rust-lang.org/tools/install">The Rust toolchain</a> and git, run:

```fish
git clone --depth=1 https://github.com/ArrowAced/jittermouse
cd jittermouse
cargo build
```

You can also use `cargo run` to build and run jittermouse.

