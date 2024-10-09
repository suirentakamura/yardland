# Yardland, the official 65x64 emulator

Yardland is a Rust-based project aimed at emulating the Nozotech 65x64 experimental cpu.

## Features

- **Simple**: Yardland is designed to be easy to use and understand.
- **Accurate**: Yardland aims to be as accurate as possible in emulating the 65x64.
- **Fast**: Yardland is built with performance in mind.
- **Cross-platform**: Yardland is compatible with Windows, macOS, and Linux.
- **Open-source**: Yardland is free and open-source software.
- **Extensible**: Yardland is designed to be easily extensible with plugins.

## Installation

To install Yardland, ensure you have Rust installed, then run:

```sh
cargo install yardland
```

## Usage

After installation, you can start using Yardland with:

```sh
yardland --help
```

## Development Setup

To set up Yardland for development, clone the repository and navigate to the project directory:

```sh
git clone https://github.com/kyokotoreno/yardland.git
cd yardland
```

Then, install the needed cargo components to build bevy:

```sh
# LLD installation for linux
sudo pacman -S lld clang # Arch
sudo apt-get install lld clang # Ubuntu
sudo dnf install lld clang # Fedora

# LLD installation for windows
cargo install -f cargo-binutils
rustup component add llvm-tools-preview

# Cranelift codegen installation
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```

## Contributing

We welcome contributions! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Support

If you encounter any issues or have questions, feel free to open an issue on our [GitHub Issues](https://github.com/kyokotoreno/yardland/issues) page.

## Acknowledgements

We would like to thank all the contributors and the open-source community for their invaluable support.

## Contact

For further inquiries, you can reach us at [kyokotoreno@proton.me](mailto:kyokotoreno@proton.me).
