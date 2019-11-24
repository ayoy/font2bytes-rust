font2bytes
========================
![](https://github.com/ayoy/font2bytes-rust/workflows/Rust/badge.svg)

This is a rewrite of the [font2bytes](https://github.com/ayoy/font2bytes) C++ app in Rust, made purely for learning purposes.
This command-line utility allows for converting a PNG image with a font sample to an array of bytes representing that font, for use in embedded systems' displays.

It has the same set of features as the original app (just no UI). Read the whole story behind the original `font2bytes` [on my blog](https://kapusta.cc/2019/02/10/font2bytes/). 

Requirements
-------------------
The app was developed with Rust 1.39.0 and tested on MacOS and Linux.

Usage
-------------------
* Use `cargo run -- --help` for a full list of configuration options.
* See example font files in `templates` directory. You'll also find there PSD templates for 8x8 and 16x11 fonts.

Author
-------------------
Dominik Kapusta http://kapusta.cc

License
-------------------
MIT, see LICENSE for details.
