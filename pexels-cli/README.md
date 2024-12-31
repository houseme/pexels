# Pexels CLI

Pexels CLI is a command-line interface for interacting with the Pexels API. It allows you to search for photos, videos, and collections, as well as retrieve individual media items by their ID.

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/houseme/pexels.git
    ```

2. Navigate to the project directory:
    ```sh
    cd pexels/pexels-cli
    ```

3. Build the project:
    ```sh
    cargo build
    ```

## Usage

Before using the CLI, make sure to set your Pexels API key in a `.env` file:

```sh
PEXELS_API_KEY=your_api_key_here
```

### Commands

- **Search Photos**:
    ```sh
    cargo run -- search-photos --query "nature" --per-page 10 --page 1
    ```

- **Search Videos**:
    ```sh
    cargo run -- search-videos --query "nature" --per-page 10 --page 1
    ```

- **Get Photo by ID**:
    ```sh
    cargo run -- get-photo --id 12345
    ```

- **Get Video by ID**:
    ```sh
    cargo run -- get-video --id 12345
    ```

- **Search Collections**:
    ```sh
    cargo run -- search-collections --per-page 10 --page 1
    ```

- **Search Media**:
    ```sh
    cargo run -- search-media --query "nature" --per-page 10 --page 1 --type "photo" --sort "latest"
    ```

## Documentation

For detailed documentation, please refer to [Documentation](https://docs.rs/pexels-cli).

## License

Licensed under either of

* Apache License, Version 2.0, [LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 or MIT license, shall be dual licensed as above, without any additional terms or conditions.