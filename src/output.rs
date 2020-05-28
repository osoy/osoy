pub fn print_usage() {
    println!(
        "
  usage: osoy <operator> [arguments] [flags]

  operators:
    c|clone    <query*>   clone packages from GitHub, GitLab or Bitbucket
    r|remove   <query*>   remove packages
    l|list     [query*]   list (all) packages
    m|make     [query*]   make (all) packages
    s|symlink  [query*]   link packages' executables to PATH
    u|update   [query*]   update (all) packages
    dir        <query>    print package's directory path
    readme     <query>    view package's README file
    license    <query>    view package's LICENSE file

  query syntax: <[[domain/]author/]package> || <link>
"
    );
}
