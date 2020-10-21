<div align='center'>
  <img alt='logo' src='./logo.svg' height="64" />
  <h1>Osoy</h1>
</div>

Command-line git repository manager.
Inspired by
[vim-plug](https://github.com/junegunn/vim-plug)
[yarn](https://github.com/yarnpkg/yarn) and
[pacman](https://wiki.archlinux.org/index.php/Pacman).

## Table of contents

- [Compatibility](#Compatibility)
- [Dependencies](#Dependencies)
- [Usage](#Usage)
  - [Operators](#Operators)
  - [Query syntax](#Query-syntax)
  - [Flags](#Flags)
- [Installation](#Installation)
  - [Cargo](#Cargo)
  - [Manual](#Manual)
  - [PATH](#PATH)
- [Configuration](#Configuration)
- [File structure](#File-structure)
- [Todo](#Todo)

## Compatibility

This software relies on extended file metadata to determine whether a file is executable.

## Dependencies

`git`, `cargo` & `make` are required for the application to fully function.

`cargo` is required to build the executable.

## Usage

    osoy [operator] [flags] [params]

#### Operators

| Short | Long    | Parameters                | Description                         |
| ----- | ------- | ------------------------- | ----------------------------------- |
| n     | new     | \<query\>...              | create new empty git repositories   |
| cl    | clone   | \<query\>...              | clone git repositories              |
|       | fork    | \<query\> \<destination\> | clone git repository to a different |
|       | pull    | [query]...                | update repositories                 |
| ln    | link    | [query]...                | link executables to PATH            |
| ls    | list    | [query]...                | list repositories                   |
| rm    | remove  | \<query\>...              | remove repositories                 |
| mv    | move    | \<query\> \<destination\> | rename repository remote origin     |
| st    | status  | [query]...                | show repository statuses            |
| mk    | make    | [query]...                | make/build repositories             |
|       | dir     | \<query\>                 | print repository directory path     |
|       | readme  | \<query\>                 | print repository README file        |
|       | license | \<query\>                 | print repository LICENSE file       |

#### Query syntax

`<[[domain/]author/]package>` or `<link>`

Default domain is github.com & default author is `<package>`.

#### Flags

| Short | Long       | Description                   |
| ----- | ---------- | ----------------------------- |
| -c    | --color    | enable colors                 |
| -d    | --details  | show detailed output          |
| -h    | --help     | show usage                    |
| -v    | --version  | show version                  |
| -f    | --force    | force prompts                 |
| -y    | --defaults | continue with prompt defaults |
| -n    | --deny     | deny prompts                  |

#### Options

| Short | Long     | Description                                     |
| ----- | -------- | ----------------------------------------------- |
| -o    | --option | specify options/features to run make/build with |

Options can have one value or multiple comma-seperated values.

> Ex: osoy make -o clean,all

## Installation

#### Cargo

Install with cargo.

>     cargo install osoy

Default cargo bin directory is `~/.cargo/bin`.
More at [rust-lang.org](https://doc.rust-lang.org/cargo/guide/cargo-home.html#directories).

#### Manual

Clone osoy git repository.

>     mkdir -p ~/.osoy/packages/github.com/osoy &&
>       git clone https://github.com/osoy/osoy ~/.osoy/packages/github.com/osoy/osoy

Change directory to `~/.osoy/packages/github.com/osoy/osoy`.

>     cd ~/.osoy/packages/github.com/osoy/osoy

Create osoy release build.

>     cargo build --release

Create a symbolic links with osoy.

>     ./target/release/osoy symlink

#### PATH

Add osoy bin directory ~/.osoy/bin to your system path.

>     PATH="$PATH:$HOME/.osoy/bin"

To make it permanent add the previous line to your shell profile — ~/.bash_profile, ~/.zprofile, ~/.profile, etc.  
More at
[askubuntu.com](https://askubuntu.com/questions/60218/how-to-add-a-directory-to-the-path) or
[google.com](https://www.google.com/?q=add+directory+to+path)

## Configuration

You can configure osoy by making an alias.
For example, next line will enable colors by default

>     alias osoy='osoy -c'

Following line will make it easier to navigate to package's directory

>     oycd() { cd "$(osoy dir "$*")"; }

## File structure

    ~/.osoy/
    ├── bin/
    │   ├── <symlink>  ->  <executable>
    │   :   ...
    │
    └── packages/
        ├── <domain>/
        │   :   ...
        │
        ├── <domain>/
        :   ├── <package>/
            │   :   ...
            │
            ├── <author>/
            │   :   ...
            │
            ├── <author>/
            :   ├── <package>/
                │   :   ...
                │
                ├── <package>/
                :   ├── <modules>/
                    ├── <module>
                    ├── <executable>
                    :   ...

## Todo

- [x] default list and status with less details
- [x] two-letter shorthands
- [x] disable prompt linking
- [x] fix move (when target & destination match)
- [ ] operator errors should be returned
- [ ] enable linking with flags
- [ ] tidy output messages
- [ ] unlink operator
- [ ] parallel list and status
- [ ] copy operator
- [ ] visual progress for update, clone & fork
- [ ] parallel updating & cloning
- [ ] improve build support (wrappers: maven&gradle, npm&yarn, deno)
- [ ] windows support (symlink & is_exe)
- [ ] docs website
