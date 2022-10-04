# Computer Vision in Rust
This repository has only a few rust scripts, **for rust learning purposes**.
Therefore, you will probably find implementations done in a bad way.


## Env

Creating the env to dev.

```bash
$ pip install rustenv # similar to `virtualenv`

$ rustenv renv # Create the rustenv

$ . renv/bin/activate # Activate the env
```

Using cargo:

```bash
$ cargo init --bin # Create a new Rust project

$ cargo add image # add a dependency

$ cargo run # will run the current package (src/main.rs)

# Working with isolated scripts

# First add into `Cargo.toml` a bin section with path and name for each file
$ cargo build # Build everthing

$ cargo run --bin <name_script> # Run the single script
```

