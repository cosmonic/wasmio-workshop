# wasmCloud Crash Course with Cosmonic

This repo contains some useful getting started information, scripts, and templates for the Crash
Course workshop. Below is an overview of what is in the repo:

- `todo/`: Contains fully functional code of the application we'll be writing for you to use as a
  reference
- `provider/`: A scaffold for creating your own custom provider as part of the stretch goals
- `kubernetes.sh`: An optional script for connecting your Kubernetes cluster

## Getting started

For this workshop you'll need to have the following tools installed:

- Rust
- `cosmo` cli
- `wash` cli (somewhat optional, but recommended)

If for some reason you are having issues installing or getting started, we also created a
devcontainer you can use!

### Installing Rust

For most people, you can just run the following command to install rust

```terminal
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For a list of different methods of installation, please see the [Rust
docs](https://www.rust-lang.org/tools/install)

To follow this example and build own actors or providers, you will need to add the  `wasm32-unkown-unknown` compile target:

```bash
rustup target add wasm32-unknown-unknown
```

### Installing Cosmo

Installing cosmo is as easy as running the command below:

```terminal
bash -c "$(curl -fsSL https://cosmonic.sh/install.sh)"
```

The install script is very simple. If piping into bash is worrysome for you, please feel free to
take a look at the script before running

### Installing Wash

This is optional, but highly recommended. There are some tools we are still porting over to `cosmo`
that might be useful for debugging, learning, and advanced usage.

If you don't want to use a package manager, the easiest way to do it is:

```terminal
cargo install wash-cli
```

For full package manager instructions, see the [wasmCloud
docs](https://wasmcloud.dev/overview/installation/)
