use regex::Regex;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{error, fmt};

#[derive(Debug, PartialEq, Clone)]
enum Protocol {
    Git,
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    protocol: Option<Protocol>,
    id: Vec<String>,
}

impl Location {
    pub fn about() -> &'static str {
        "<[[domain/]author/]package> or url"
    }

    pub fn id(&self) -> String {
        match &self.protocol {
            Some(_) => self.id.join("/"),
            None => format!(
                "{}{}{}",
                match self.id.len() < 3 {
                    true => "github.com/",
                    false => "",
                },
                match self.id.len() == 1 {
                    true => format!("{}/", self.id[0]),
                    false => "".into(),
                },
                self.id.join("/"),
            ),
        }
    }

    pub fn url(&self) -> String {
        match &self.protocol {
            Some(Protocol::Other(p)) => format!("{}://{}", p, self.id.join("/")),
            Some(Protocol::Git) => format!(
                "{}{}",
                self.id
                    .get(0)
                    .map(|domain| format!("git@{}:", domain))
                    .unwrap_or("".to_string()),
                self.id
                    .get(1..)
                    .map(|route| route.join("/"))
                    .unwrap_or("".to_string())
            ),
            None => format!("https://{}", self.id(),),
        }
    }

    pub fn matches_re(&self, path: &Path) -> bool {
        let mut path = PathBuf::from(path);
        for word in self.id.iter().rev() {
            if Regex::new(&format!("^{}$", word))
                .map(|re| {
                    re.is_match(
                        path.file_name()
                            .map(|osname| osname.to_str())
                            .flatten()
                            .unwrap_or(""),
                    )
                })
                .unwrap_or(false)
            {
                path.pop();
            } else {
                return false;
            }
        }
        true
    }

    pub fn matches(&self, path: &Path) -> bool {
        let mut path = PathBuf::from(path);
        for word in self.id.iter().rev() {
            if path.ends_with(word) {
                path.pop();
            } else {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id.join("/"))
    }
}

#[derive(Clone, Debug, Copy)]
pub struct ParseLocationError {}

impl fmt::Display for ParseLocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "invalid location".fmt(f)
    }
}

impl error::Error for ParseLocationError {}

impl FromStr for Location {
    type Err = ParseLocationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_other = Regex::new("^([^:/]+)://").unwrap();
        let re_git = Regex::new("^git@([^:]+):|^([^:/@]+):").unwrap();

        let protocol;
        let id: Vec<String>;

        if re_other.is_match(s) {
            protocol = Some(Protocol::Other(re_other.captures(s).unwrap()[1].into()));
            id = re_other
                .replace(s, "")
                .split("/")
                .map(|s| s.to_owned())
                .collect();
        } else if re_git.is_match(s) {
            protocol = Some(Protocol::Git);
            id = re_git
                .replace(s, "$1$2/")
                .split("/")
                .map(|s| s.to_owned())
                .collect();
        } else {
            protocol = None;
            id = s.split("/").map(|s| s.to_owned()).collect();
        }

        if id.len() == 0 {
            Err(ParseLocationError {})
        } else {
            Ok(Self { protocol, id })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(query: &str, url: &str, id: &str, display: &str) {
        let location = Location::from_str(query).unwrap();
        assert_eq!(location.url(), url);
        assert_eq!(location.id(), id);
        assert_eq!(location.to_string(), display);
    }

    #[test]
    fn full() {
        check(
            "http://github.com/rasmusmerzin/hue",
            "http://github.com/rasmusmerzin/hue",
            "github.com/rasmusmerzin/hue",
            "github.com/rasmusmerzin/hue",
        );
        check(
            "git@gitlab.com:rasmusmerzin/archer",
            "git@gitlab.com:rasmusmerzin/archer",
            "gitlab.com/rasmusmerzin/archer",
            "gitlab.com/rasmusmerzin/archer",
        );
    }

    #[test]
    fn partial() {
        check(
            "gitlab.com:rasmusmerzin/xhueloop",
            "git@gitlab.com:rasmusmerzin/xhueloop",
            "gitlab.com/rasmusmerzin/xhueloop",
            "gitlab.com/rasmusmerzin/xhueloop",
        );
        check(
            "gitlab.com/rasmusmerzin/gol-java",
            "https://gitlab.com/rasmusmerzin/gol-java",
            "gitlab.com/rasmusmerzin/gol-java",
            "gitlab.com/rasmusmerzin/gol-java",
        );
        check(
            "rasmusmerzin/recl",
            "https://github.com/rasmusmerzin/recl",
            "github.com/rasmusmerzin/recl",
            "rasmusmerzin/recl",
        );
        check(
            "osoy",
            "https://github.com/osoy/osoy",
            "github.com/osoy/osoy",
            "osoy",
        );
    }
}
