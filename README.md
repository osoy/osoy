<h1>
  <img src="./logo.svg" />
  <br />
  Osoy
  <br />
  <a href="https://gitlab.com/osoy/osoy/-/commits/main">
    <img alt="build" src="https://img.shields.io/gitlab/pipeline/osoy/osoy/main" />
  </a>
  <a href="https://docs.rs/osoy">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/osoy"/>
  </a>
  <a href="https://crates.io/crates/osoy">
    <img alt="crates.io" src="https://img.shields.io/crates/v/osoy" />
  </a>
  <a href="https://aur.archlinux.org/packages/osoy-bin">
    <img alt="aur.archlinux.org" src="https://img.shields.io/aur/version/osoy-bin"/>
  </a>
  <a href="https://sourceforge.net/p/osoy">
    <img alt="sourceforge.net" src="https://img.shields.io/sourceforge/dt/osoy"/>
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

### AUR

```
yay -Sy osoy-bin
```

### Compiling from Source

[Cargo](https://doc.rust-lang.org/cargo) (+rustc) and [Git](https://git-scm.com) are required to compile.

```bash
git clone https://gitlab.com/osoy/osoy ~/.osoy/src/gitlab.com/osoy/osoy
cd ~/.osoy/src/gitlab.com/osoy/osoy
cargo build --release
./osoy link osoy -fv
```

## Updating

Pull updates with `osoy pull osoy`.

Recompile with `osoy execute osoy make` or `osoy execute osoy -- cargo build --release`.
