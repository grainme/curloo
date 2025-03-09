# Curloo

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/grainme/curloo/.github/workflows/ci.yml?label=build)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

Curloo is a lightweight HTTP client that combines command-line efficiency with interactive functionality. It supports both interactive (`auto`) and direct (`manual`) modes, making it great for API testing and debugging.

## Features

* **Interactive Mode**: Use `auto` mode for a guided, step-by-step interface
* **Direct Mode**: Use `manual` mode for quick, one-off HTTP requests - WIP
* **Multiple HTTP Methods**: Supports `GET`, `POST`, `PUT`, and `DELETE`
* **Single Binary**: No dependencies—just download and run
* **Cross-Platform**: Works on Linux, macOS, and Windows

## Installation

### From Source

1. Ensure you have Rust installed
2. Run the following command:

```bash
cargo install --git https://github.com/grainme/curloo
```

### Docker

Pull the image directly from Docker Hub:

```bash
docker pull grainme47/curloo
```

### Pre-built Binaries

Download the latest release for your platform from the [Releases](https://github.com/grainme/curloo/releases) page.


## Usage

### Interactive Mode

Run Curloo in interactive mode for a guided experience:

```bash
curloo --mode auto
```


## Options

```
Usage: curloo [OPTIONS]

Options:
      --mode <MODE>
          Operation mode: 'auto' for interactive prompts, 'manual' for direct command input.

          [default: manual]
          [possible values: auto, manual]

  -m, --method <METHOD>
          HTTP request method. Defaults to GET if not specified.

          [default: get]
          [possible values: get, post, delete, put]

  -u, --url <URL>
          Target URL for the HTTP request.

  -b, --body <BODY>
          Request body, typically used with POST and PUT requests.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Contributing

Curloo is open source under the MIT License. Contributions are welcome! Here's how you can help:

* **Report Issues**: Found a bug? Open an issue on GitHub
* **Suggest Features**: Have an idea for a new feature? Let us know!
* **Submit Pull Requests**: Contributions are always welcome. Follow these steps:
   1. Fork the repository
   2. Create a new branch for your feature or bugfix
   3. Submit a pull request with a detailed description of your changes

## License

Curloo is licensed under the MIT License. Feel free to use, modify, and distribute it as you see fit.

## Acknowledgments

* Built with ❤️ and Rust
* Designed for modern API workflows
* Inspired by tools like curl, Postman and HTTPie.

## Performance

Curloo is designed to be lightweight and fast. It tries to leverage Rust's performance and safety features to provide a seamless experience for API testing.

## What mode should I use?

* **Interactive Mode**: Perfect for beginners or when you need a guided experience
* **Direct Mode**: Ideal for scripting or quick debugging

## Contribution Opportunities

Feel free to help us! I (grainme) am still new to Rust by the way :)