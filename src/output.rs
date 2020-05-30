use regex::Regex;

pub fn print_usage(color: bool) {
    let usage = "
usage: osoy <operator> [arguments] [flags]

operators:
  c|clone    <query*>  clone packages from GitHub, GitLab or Bitbucket
  r|remove   <query*>  remove packages
  l|list     [query*]  list (all) packages
  m|make     [query*]  make (all) packages
  s|symlink  [query*]  link packages' executables to PATH
  u|update   [query*]  update (all) packages

  dir        <query>   print package's directory path
  readme     <query>   view package's README file
  license    <query>   view package's LICENSE file

flags:
  -c --color      enable colors
  -f --force      force prompts
  -d --defaults   continue with prompt defaults
  -h --help       show usage
  -v --version    show version
  -o --option <>  specify option to run make with

query syntax: <[[domain/]author/]package> or <link>
  default domain is github.com & default author is <package>
";
    if color {
        println!(
            "{}",
            Regex::new(r"([*|\[\]<>])")
                .unwrap()
                .replace_all(usage, "\u{1b}[2m$1\u{1b}[0m")
        );
    } else {
        println!("{}", usage);
    }
}
