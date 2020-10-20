use regex::Regex;

pub fn print_usage(color: bool) {
    let usage = "
usage: osoy <operator> [arguments] [flags]

operators:
  c|clone   <query>...            clone packages from using git
  f|fork    <query> <destination> clone a package overwriting remote origin
  y|symlink [query]...            make packages' executables available in PATH
  l|list    [query]...            list (all) packages
  s|status  [query]...            show status of (all) packages
  b|build   [query]...            build (all) packages that have a make or cargo file
  r|remove  <query>...            remove packages
  m|move    <query> <destination> rename package's remote origin and relocate it's folder
  n|new     <query>...            create new empty packages
  u|update  [query]...            update (all) packages
  dir       <query>               print package's directory path
  read      <query>               view package's README file
  license   <query>               view package's LICENSE file

flags:
  -c --color      enable colors
  -d --details    show detailed output
  -h --help       show usage
  -v --version    show version
  -f --force      force prompts
  -y --defaults   continue with prompt defaults
  -n --deny       deny prompts
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
