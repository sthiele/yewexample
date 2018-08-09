# web frontend with yew

    $ rustup target add wasm32-unknown-unknown

To build this project you nedd to have [cargo-web] installed:
    $ cargo install cargo-web
> Add '--force' option to ensure you install the latest version.

#### Build
    $ cargo +nightly web build --target=wasm32-unknown-unknown

#### Run
    $ cargo +nightly web start --target=wasm32-unknown-unknown


[cargo-web]: https://github.com/koute/cargo-web
