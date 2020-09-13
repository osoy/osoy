<p align='center'>
  <img alt='logo' src='./logo.svg' height="64" />
</p>
<h1 align='center'>Osoy</h1>

Command-line git repository manager.
Inspired by
<a href='https://github.com/junegunn/vim-plug' />vim-plug</a>,
<a href='https://github.com/yarnpkg/yarn' />yarn</a> and
<a href='https://wiki.archlinux.org/index.php/Pacman' />pacman</a>.

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

    osoy [operator] [flags]

#### Operators

    c|clone   <query>...      clone packages from GitHub, GitLab or Bitbucket
    f|fork    <query> <fork>  clone a package overwriting remote origin to fork
    y|symlink [query]...      make packages' executables available in PATH
    l|list    [query]...      list (all) packages
    s|status  [query]...      show status of (all) packages
    b|build   [query]...      build (all) packages that have a make or cargo file
    r|remove  <query>...      remove packages
    m|move    <query> <dest>  rename package's remote origin and relocate it's folder
    n|new     <query>...      create new empty packages
    u|update  [query]...      update (all) packages

    dir       <query>         print package's directory path
    read      <query>         view package's README file
    license   <query>         view package's LICENSE file

#### Query syntax

`<[[domain/]author/]package>` or `<link>`

Default domain is github.com & default author is `<package>`.

#### Flags

    -c --color      enable colors
    -q --quiet      show less detailed output
    -h --help       show usage
    -v --version    show version

    -f --force      force prompts
    -y --defaults   continue with prompt defaults
    -n --deny       deny prompts

    -o --option <>  specify option to run make with

## Installation

#### Cargo

Install with cargo.

>     cargo install osoy

Default cargo bin directory is `~/.cargo/bin`.
More at [rust-lang.org](https://doc.rust-lang.org/cargo/guide/cargo-home.html#directories).

#### Manual

Clone osoy git repository.

>     mkdir -p ~/.osoy/packages/github.com/osoy &&
>       cd ~/.osoy/packages/github.com/osoy &&
>       git clone https://github.com/osoy/osoy

Change directory to `~/.osoy/packages/github.com/osoy/osoy`.

>     cd ~/.osoy/packages/github.com/osoy/osoy

Create osoy release build.

>     cargo build --release

Create a symbolic links with osoy.

>     ./target/release/osoy y

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

- [x] option flag for make
- [x] fork operator
- [x] list current branches
- [x] deny flag to deny all prompts
- [x] chain make prompt to update, clone & fork
- [x] quiet flag for less output
- [x] add explicit prompt when running `remove` without a query
- [ ] copy operator
- [ ] visual progress for update, clone & fork
- [ ] asynchronous updating & cloning
- [ ] docs website
