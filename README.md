<p align='center'>
  <img alt='logo' src='./logo.svg' height="64" />
</p>
<h1 align='center'>Osoy</h1>

Lightweight git repository manager written in Rust.
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
- [Configuration](#Configuration)
- [File structure](#File-structure)
- [Contributions](#Contributions)

## Compatibility

This software relies on extended file metadata to determine whether a file is executable.

## Dependencies

`git` & `make` are required for the application to function.

`make` & `cargo` are required to build the executable.

## Usage

    osoy [operator] [flags]

#### Operators

    c|clone    <query*>  clone packages from GitHub, GitLab or Bitbucket
    r|remove   <query*>  remove packages
    l|list     [query*]  list (all) packages
    m|make     [query*]  make (all) packages
    s|symlink  [query*]  make packages' executables available in PATH
    u|update   [query*]  update (all) packages

    dir        <query>   print package's directory path
    read       <query>   view package's README file
    license    <query>   view package's LICENSE file

#### Query syntax

`<[[domain/]author/]package>` or `<link>`

Default domain is github.com & default author is `<package>`.

#### Flags

    -c --color     enable colors
    -f --force     force prompts
    -d --defaults  continue with prompt defaults
    -h --help      show usage
    -v --version   show version

## Installation

Clone osoy git repository.

>     mkdir -p ~/.osoy/packages/github.com/osoy &&
>       cd ~/.osoy/packages/github.com/osoy &&
>       git clone https://github.com/osoy/osoy

Create a symbolic link for osoy executable.

>     mkdir -p ~/.osoy/bin &&
>       ln -s ~/.osoy/packages/github.com/osoy/osoy/osoy ~/.osoy/bin/osoy

Build the executable.

>     cd ~/.osoy/packages/github.com/osoy/osoy && make

Add osoy bin directory ~/.osoy/bin to your system path.

>     PATH="$PATH:$HOME/.osoy/bin"

To make it permanent add the previous line to your shell profile — ~/.bash_profile, ~/.zprofile, ~/.profile, etc.
More at
<a href='https://www.computerhope.com/issues/ch001647.htm'>computerhope</a>,
<a href='https://askubuntu.com/questions/60218/how-to-add-a-directory-to-the-path'>askubuntu</a> or
<a href='https://www.google.com/?q=add+directory+to+path'>google</a>.

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

- [ ] fork operator
