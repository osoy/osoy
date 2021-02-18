<h1>
  <img src="./logo.svg" />
  <br />
  Osoy
  <br />
  <a href="https://gitlab.com/osoy/osoy/-/commits/master">
    <img alt="build" src="https://img.shields.io/gitlab/pipeline/osoy/osoy/master" />
  </a>
  <a href="https://crates.io/crates/osoy">
    <img alt="license" src="https://img.shields.io/crates/l/osoy" />
  </a>
  <a href="https://crates.io/crates/osoy">
    <img alt="version" src="https://img.shields.io/crates/v/osoy" />
  </a>
  <a href="https://docs.rs/osoy">
      <img alt="documentation" src="https://img.shields.io/badge/docs.rs-osoy-blue"/>
  </a>
</h1>

Osoy is a command-line git repository manager which's features include:

- Clone and pull repositories in bulk.
- Bulk execute commands in repositories.
- Filter repositories using regex.
- Create symbolic links to repositories' executables.
- See the status of all repositories with one command.

## Osoy Home

Osoy home is where dowloaded git repositories and created symbolic links will be stored.
You can alter the location of Osoy home by setting the `OSOY_HOME` environment variable
which by default is `$HOME/.osoy` (`%USERPROFILE%\.osoy` on Windows).

### Directories

- `src` Downloaded repositories will be stored here.
- `bin` Symolic links to executables will be stored here.
  To make these accessible, add the path of the directory to your `PATH` environment variable.

## Installation

### Cargo

```bash
cargo install osoy
```

Make sure that [cargo bin](https://doc.rust-lang.org/stable/cargo/guide/cargo-home.html#directories) is in your `PATH` environment variable.

### Compiling from Source

[Cargo](https://doc.rust-lang.org/cargo) (+rustc) and [Git](https://git-scm.com) are required to compile.

```bash
git clone https://gitlab.com/osoy/osoy ~/.osoy/src/gitlab.com/osoy/osoy
cd ~/.osoy/src/gitlab.com/osoy/osoy
cargo build --release --bin osoy
./osoy link osoy -fv
```
