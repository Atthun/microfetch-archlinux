<div align="center">
    <img src="https://deps.rs/repo/github/notashelf/microfetch/status.svg" alt="https://deps.rs/repo/github/notashelf/microfetch">
    <img src="https://img.shields.io/github/stars/atthun/microfetch-archlinux?label=stars&color=DEA584">
</div>

<h1 align="center">Microfetch but Arch Linux</h1>

Stupidly simple, laughably fast fetch tool. But the Arch Linux version...

<p align="center">
  <img
    alt="Demo"
    src="./.github/assets/Demo.png"
    width="850px"
  >
</p>

## Installation

Manual installation:-

```sh
git clone https://github.com/Atthun/microfetch-archlinux`
cd microfetch-archlinux
cargo build --release
sudo mv target/release/microfetch /usr/bin/microfetch
```

1. `git clone https://github.com/Atthun/microfetch-archlinux` Clone the repository
2. `cd microfetch-archlinux` Change directory into the repository
3. `cargo build --release` Build it using cargo to obtain the binary. The `--release` is used to specify the build profile, release is recommended for normal users.
4. You can now either move the binary using `sudo mv target/release/microfetch /usr/bin/microfetch` or write a PKGBUILD to install it using `pacman`.

> [!NOTE]
> You will need a Nerdfonts patched font installed, and for your terminal
> emulator to support said font. Microfetch uses nerdfonts glyphs by default.

Thanks to [@NotAShelf](https://github.com/NotAShelf) - For the original project

## License

Microfetch is licensed under [GPL3](LICENSE). See the license file for details.
