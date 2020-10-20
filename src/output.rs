const USAGE: &str = "Usage: osoy <operator> [arguments] [flags]

Operators:
  n,  new      <query>...             create new empty git repositories
  cl, clone    <query>...             clone git repositories
      fork     <query> <destination>  clone git repository to a different
      pull     [query]...             update repositories
  ln, link     [query]...             link executables to PATH
  ls, list     [query]...             list repositories
  rm, remove   <query>...             remove repositories
  mv, move     <query> <destination>  rename repository remote origin
  st, status   [query]...             show repository statuses
  mk, make     [query]...             make/build repositories
      dir      <query>                print repository directory path
      readme   <query>                print repository README file
      license  <query>                print repository LICENSE file

Flags:
  -c, --color      enable colors
  -d, --details    show detailed output
  -h, --help       show usage
  -v, --version    show version
  -f, --force      force prompts
  -y, --defaults   continue with prompt defaults
  -n, --deny       deny prompts
  -o, --option <>  specify option to run make with

Query syntax: <[[domain/]author/]package> or <link>
  Default domain is github.com & default author is <package>";

pub fn print_usage() {
    println!("{}", USAGE);
}
