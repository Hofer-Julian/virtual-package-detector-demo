# Virtual package detector demo

A Rust detector reports the username's character count as `__username_length`. The consumer selects `long-username` for counts of at least 10 and `short-username` otherwise.

The detector reads the first nonempty `LOGNAME`, `USER`, `LNAME`, or `USERNAME` environment variable. Platform declarations use `username_length`, while the detector protocol and channel registration use `__username_length`.

The detector uses `pixi-build-rust` and supports Linux, macOS, and Windows. Build it on the machine running the demo.

## Run

From this directory, build Pixi from [PR #7129](https://github.com/prefix-dev/pixi/pull/7129), install the released indexer, and publish the detector to the local channel:

```console
pixi global install --git https://github.com/prefix-dev/pixi.git --rev refs/pull/7129/head --expose pixi-dev=pixi
pixi global install rattler-index
pixi-dev publish --force --path detector --target-channel channel
rattler-index --force --config index-config.toml fs channel
pixi-dev run --manifest-path consumer detect
```

`--force` refreshes the index when rebuilding a package with the same filename.

When prompted, allow the channel's detectors and choose `This repository` to save the decision locally. Choose `All repositories (shared configuration)` to apply it across repositories. Declining skips detector execution.

## Check platform selection

On Linux or macOS, clear the consumer environment before each run:

```console
pixi-dev clean --manifest-path consumer --environment default
CONDA_OVERRIDE_USERNAME_LENGTH=12 pixi-dev run --manifest-path consumer detect
pixi-dev clean --manifest-path consumer --environment default
CONDA_OVERRIDE_USERNAME_LENGTH=5 pixi-dev run --manifest-path consumer detect
```

The runs print `long-username` and `short-username`, respectively. On Windows, set `CONDA_OVERRIDE_USERNAME_LENGTH` using your shell's environment-variable syntax. An empty override declares the capability absent.
