# Website

This repository consists of `rust-at-night` website frontend and backend crates.

# Project Structure

To be written..

# Prerequisites

This is a rust code base, it is assumed that you have `rust` and `cargo` installed on your system. If not, please follow the instructions [here](https://www.rust-lang.org/tools/install).

- `cargo install --locked wasm-bindgen-cli trunk just`

# Running the backend

In the `website-backend` root directory, run the following command:

```sh
just run_dev
```

If you'd like to use the local tracing collector, run the following command:

```sh
just run_with_tracing
```

for the local tracing collector to work, you need to have `jaeger` server running.

```sh
just run_tracing_server
```

this sets up a container running `jaeger` server.

You may check `website-backend/src/options.rs` for environment variables that can be used to configure the backend.

# Running the frontend

Currently the frontend depends on the backend to work. In later PRs a better dev setup will be provided.

In the `website-frontend` root directory, run the following command:

```sh
just watch
```

this populates the `dist` directory with the compiled frontend code.

The running backend can serve this code.
