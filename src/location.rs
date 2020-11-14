use regex::Regex;
use std::str::FromStr;
use std::{error, fmt};

#[derive(Debug, PartialEq)]
enum Protocol {
    Git,
    Other(String),
}

#[derive(Debug, PartialEq)]
pub struct Location {
    protocol: Option<Protocol>,
    id: Vec<String>,
}

impl Location {
    pub fn about() -> &'static str {
        "<[[domain/]author/]package> or url"
    }

    pub fn id(&self) -> String {
        self.id.join("/")
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
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
                _ => todo!(),
            }
        )
    }
}

#[derive(Clone, Debug)]
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
        if re_other.is_match(s) {
            Ok(Self {
                protocol: Some(Protocol::Other(re_other.captures(s).unwrap()[1].into())),
                id: re_other
                    .replace(s, "")
                    .split("/")
                    .map(|s| s.to_owned())
                    .collect(),
            })
        } else if re_git.is_match(s) {
            Ok(Self {
                protocol: Some(Protocol::Git),
                id: re_git
                    .replace(s, "$1/")
                    .split("/")
                    .map(|s| s.to_owned())
                    .collect(),
            })
        } else {
            Ok(Self {
                protocol: None,
                id: s.split("/").map(|s| s.to_owned()).collect(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url() {
        let check = |url, id| {
            let location = Location::from_str(url).unwrap();
            assert_eq!(location.to_string(), url);
            assert_eq!(location.id(), id);
        };

        check(
            "http://github.com/rasmusmerzin/hue",
            "github.com/rasmusmerzin/hue",
        );
        check(
            "git@gitlab.com:rasmusmerzin/archer",
            "gitlab.com/rasmusmerzin/archer",
        );
    }

    #[test]
    fn query() {
        let check = |query, url, id| {
            let location = Location::from_str(query).unwrap();
            assert_eq!(location.to_string(), url);
            assert_eq!(location.id(), id);
        };

        check(
            "gitlab.com:rasmusmerzin/xhueloop",
            "git@gitlab.com:rasmusmerzin/xhueloop",
            "gitlab.com/rasmusmerzin/xhueloop",
        );
        check(
            "gitlab.com/rasmusmerzin/gol-java",
            "https://gitlab.com/rasmusmerzin/gol-java",
            "gitlab.com/rasmusmerzin/gol-java",
        );
        check(
            "rasmusmerzin/recl",
            "https://github.com/rasmusmerzin/recl",
            "github.com/rasmusmerzin/recl",
        );
        check(
            "osoy",
            "https://github.com/osoy/osoy",
            "github.com/osoy/osoy",
        );
    }
}
