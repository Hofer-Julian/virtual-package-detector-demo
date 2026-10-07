# Virtual package detector demo

A Rust detector reports the username's character count as `__username_length`. The consumer selects `long-username` for counts of at least 10 and `short-username` otherwise.

Run from this directory using Pixi from [PR #7129](https://github.com/prefix-dev/pixi/pull/7129):

```console
pixi global install --git https://github.com/prefix-dev/pixi.git --rev refs/pull/7129/head --expose pixi-dev=pixi
pixi global install rattler-index
pixi-dev publish --force --path detector --target-channel channel
rattler-index --force --config index-config.toml fs channel
pixi-dev run --manifest-path consumer detect
```

When prompted, trust the detector and choose `This repository`.
