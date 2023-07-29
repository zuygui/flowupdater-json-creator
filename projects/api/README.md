**Flow Updater JSON Creator - Backend**

Welcome to the api page of Flow Updater JSON Creator! This crate is built using Rust and wrap the CurseForge api for the Flow Updater JSON Creator tool.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Flow Updater JSON Creator is a powerful tool that simplifies the process of creating and updating JSON files for your projects using FlowUpdater. With this api, all the Curseforge api are wrapped for the cli tool.

## Features

- **CurseForge API**: This api wrap the CurseForge api for the Flow Updater JSON Creator tool.

## Getting Started

Follow the steps below to get started with Flow Updater JSON Creator:

### Prerequisites

- Rust: Ensure you have Rust installed on your system. If not, you can install it from [here](https://rust-lang.com).

### Installation

1. Add this line to your `Cargo.toml` file:

```toml
[dependencies]
flow_updater_json_creator = "1.0.0"
```

2. It's that simple! You can now use the crate in your project.

## Usage

The Crate is composed of 2 modules:

- `curseapi`: This module wrap the CurseForge api for the Flow Updater JSON Creator tool.
- `minecraft`: This module wrap some structs for the curseforge api.

Just import the module you want to use and you are good to go!

Example:

```rust
use fujc::{/* sub module */};
```

## Contributing

We welcome contributions to make Flow Updater JSON Creator even better! If you find any issues or want to add new features, feel free to open an issue or submit a pull request.

## License

Flow Updater JSON Creator is released under the [MIT License](link-to-license-file). You are free to use, modify, and distribute this software following the terms specified in the license.

Happy JSON handling!
