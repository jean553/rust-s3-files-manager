[![Build Status](https://travis-ci.org/jean553/rust-s3-files-manager.svg?branch=master)](https://travis-ci.org/jean553/rust-s3-files-manager)

# rust-s3-files-manager

Clone of aiohttp-s3-files-manager but using Rust.

## Start the service

Start and connect to the container:

```
vagrant up
vagrant ssh
```

Run the service:

```
cargo run
```

(the initial directory is the Rust project directory)

## Tests

```
cargo test
```
