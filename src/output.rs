pub fn print_usage() {
    println!(
        "
  Usage: osoy <operator> [arguments] [flags]

  Operators:
    c|clone    <query*>   clone packages from GitHub, GitLab or Bitbucket
    r|remove   <query*>   remove packages
    s|symlink  [query*]   link packages' executables to PATH
    u|update   [query*]   update (all) packages
    l|list     [query*]   list (all) packages
    m|make     [query*]   make (all) packages
    dir        <query>    print package's directory path
    read       <query>    view package's README file
    license    <query>    view package's LICENSE file
    uninstall  -          uninstall osoy and all packages

  Flags:
    -d         <domain>   enforce a specific domain to clone from
    -a         <author>   specify packages' author
    -p         <protocol> specify a protocol other than HTTPS
    -b         <branch>   specify a single branch as the HEAD
    -B         -          clone all branches
    -y         -          proceed with defaults
    -f         -          force overwriting and/or removing
    -v         -          show version
    -h         -          show help menu

  Query syntax: <[[domain/]author/]package>
"
    );
}

pub fn msg(txt: &str) {
    println!("osoy: {}", txt)
}

pub fn error(txt: &str) {
    msg(&format!("error: {}", txt))
}
