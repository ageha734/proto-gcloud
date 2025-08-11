はい、承知いたしました。
以下は、`dotnet` と `.NET` の記述を `gcloud` に修正したものです。

# proto gcloud plugin

[gcloud](https://cloud.google.com/sdk) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

This is a community plugin and is thus not built-in to proto. In order to use it, add the following to `.prototools`:

```toml
[plugins]
gcloud = "source:https://github.com/Phault/proto-gcloud/releases/latest/download/gcloud_plugin.wasm"
```

Or preferably pin a specific version, to avoid nasty surprises if we mess up a release:

```toml
[plugins]
gcloud = "source:https://github.com/Phault/proto-gcloud/releases/download/vX.Y.Z/gcloud_plugin.wasm"
```

This plugin relies on two assumptions:

  - As the SDKs will be installed to the location your `GCLOUD_ROOT` environment variable points to, it **must** be a user-writable location. This is usually `~/.gcloud` which is also the default location if the variable is not defined.
  - The location for your `GCLOUD_ROOT` must be in your `PATH`, as we do not shim or symlink it like other proto plugins.

## Usage

```shell
# install latest SDK
proto install gcloud

# install latest long-term-support release
proto install gcloud lts

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