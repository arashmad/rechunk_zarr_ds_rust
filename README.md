# ReChunk-Zarr-DS using Rust

A small python module that provides functionalities to generate and re-chunk input zarr files.

## Resources

[![Poetry](https://img.shields.io/endpoint?url=https://python-poetry.org/badge/v0.json)](https://python-poetry.org/)
[![Python](https://img.shields.io/badge/Python-3.12-blue)](https://www.python.org/)
[![codecov](https://codecov.io/gh/arashmad/rechunk-zarr-ds/graph/badge.svg?token=Z8Aabt3Yr0)](https://codecov.io/gh/arashmad/rechunk-zarr-ds)

Poetry helps you declare, manage and install dependencies of Python projects,
ensuring you have the right stack everywhere.

## Installation

Cargo is used as a dependency manager for Rust project. For installation instructions, please [check this page](https://www.rust-lang.org/tools/install).

Once you installed the Rust Cargo package manager, follow the steps below:

```bash
# Download the source code
git clone https://github.com/arashmad/rechunk_zarr_ds_rust.git
# Install dependencies
cd rechunk-zarr-ds
poetry install
# Activate virtual environment
poetry shell
# Test the code and installation
make lint && make test
```

## Run

```bash
cd /path/to/rechunk_zarr_ds_rust
cargo run # check /test/results directory
```
