# Back to Front compiler tools

Experimenting with tools for creating programming languages.

> [!WARNING]
> The tools could change at any moment for now, and could contain many bugs

## Syntem requirements

The language is designed to only **"work on my machine"** for now, meaning it runs on Windows 11
with [WSL](https://learn.microsoft.com/en-us/windows/wsl/install), and so it compiles to an `ELF`
executable file, thus it has been tested only on:

- Ubuntu 22.04.5 LTS
- CPU: Intel i7-8565u
- GPU: Nvidia MX250
- RAM: 16 GB

Therefore it will only compile code down to `x86-64` assembly, and it may work on other similar
machines and combination of components.

## Getting started

The language is written in [Rust](https://www.rust-lang.org/) version
[1.89.0](https://releases.rs/docs/1.89.0/) using [cargo](https://doc.rust-lang.org/cargo/) as the
build system.

1. Checking if Rust and cargo are installed and up to the previously specified version:

    ```shell
    cargo --version
    ```

    If an error stating that the `cargo` command could not be found occurs, try following the
    [installation guide](https://www.rust-lang.org/tools/install) to install it.

2. Cloning the repo:

    ```shell
    git clone https://github.com/StefanoIncardone/kaylang
    cd kaylang
    ```
