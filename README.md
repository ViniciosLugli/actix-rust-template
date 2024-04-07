# Actix Rust Template

This is a basic template for a web service built with [Actix Web](https://actix.rs/), a powerful, pragmatic, and extremely fast web framework for Rust.

## Features

-   **Actix Web:** Utilizes Actix Web for handling server operations, known for its speed and reliability.
-   **Serde Serialization/Deserialization:** Implements Serde for efficient and customizable serialization and deserialization of data.
-   **Environment Logging:** Configured with `pretty_env_logger` and `log` for detailed and formatted logging.
-   **Environment Configuration:** Uses `dotenvy` for loading environment variables from a `.env` file, making configuration seamless and flexible.

## Prerequisites

Before you begin, ensure you have met the following requirements:

-   Rust installed on your machine. If you don't have Rust installed, you can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

## Setup

To start using this template, clone the repository and navigate into the project directory:

```bash
git clone git@github.com:ViniciosLugli/actix-rust-template.git
cd actix-rust-template
```

### Configuration

Create a `.env` file in the root directory of your project to store environment variables:

```env
# Example .env content
RUST_LOG=info
```

### Running the Application

To run the application, use the following command:

```bash
cargo run
```

The server will start on `127.0.0.1:3000`. You can access the application by navigating to `http://localhost:3000` in your web browser or using a tool like `curl`:

```bash
curl http://localhost:3000
```

### Testing

This project contains basic tests. Run the tests with:

```bash
cargo test
```
