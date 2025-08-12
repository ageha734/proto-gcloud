# gcloud plugin

[gcloud](https://cloud.google.com/sdk) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

This is a community plugin and is thus not built-in to proto. In order to use it, add the following to `.prototools`:

```toml
[plugins]
gcloud = "github://ageha734/proto-gcloud"
```

Or preferably pin a specific version, to avoid nasty surprises if we mess up a release:

```toml
proto plugin add gcloud github://ageha734/proto-gcloud
```

## Usage

```shell
# install latest SDK
proto install gcloud

# install a specific version
proto install gcloud 519.0.0
```

## Configuration

gcloud plugin does not support configuration.

## Hooks

gcloud plugin does not support hooks.

## Contributing

Build the plugin:

```shell
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1
```

Test the plugin by running `proto` commands.

```shell
proto install gcloud-test
proto list-remote gcloud-test
```
