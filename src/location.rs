use regex::Regex;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{error, fmt};

#[derive(Debug, PartialEq, Clone)]
pub enum Protocol {
    Ssh(String),
    Other(String),
}

type LocationRegex = Vec<Option<Result<Regex, regex::Error>>>;

#[derive(Debug, Clone)]
pub struct Location {
    protocol: Option<Protocol>,
    id: Vec<String>,
    regex: Option<LocationRegex>,
}

impl Location {
    pub fn new(protocol: Option<Protocol>, id: Vec<String>) -> Self {
        Self {
            protocol,
            id,
            regex: None,
        }
    }

    pub fn about() -> &'static str {
        "<[[domain/]author/]package> or url"
    }

    pub fn id(&self) -> String {
        match &self.protocol {
            Some(_) => self.id.join("/"),
            None => format!(
                "{}{}{}",
                match self.id.len() {
                    1 | 2 => "github.com/",
                    _ => "",
                },
                match self.id.len() {
                    1 => format!("{}/", self.id[0]),
                    _ => "".into(),
                },
                self.id.join("/"),
            ),
        }
    }

    pub fn url(&self) -> String {
        match &self.protocol {
            Some(Protocol::Other(p)) => format!("{}://{}", p, self.id.join("/")),
            Some(Protocol::Ssh(user)) => format!(
                "{}{}",
                self.id
                    .get(0)
                    .map(|domain| format!("{}@{}:", user, domain))
                    .unwrap_or("".to_string()),
                self.id
                    .get(1..)
                    .map(|route| route.join("/"))
                    .unwrap_or("".to_string())
            ),
            None => format!("https://{}", self.id()),
        }
    }

    fn get_regex(&mut self) -> &LocationRegex {
        if self.regex.is_none() {
            self.regex = Some(
                self.id
                    .iter()
                    .map(|word| {
                        (!word.is_empty()).then(|| {
                            Regex::new(&format!(
                                "^({}{})$",
                                match word.starts_with("*") {
                                    true => ".",
                                    false => "",
                                },
                                word
                            ))
                        })
                    })
                    .collect::<LocationRegex>(),
            );
        }
        self.regex.as_ref().unwrap()
    }

    pub fn matches_re(&mut self, path: &Path) -> bool {
        let mut path = PathBuf::from(path);
        for word_re in self.get_regex().iter().rev() {
            if word_re.as_ref().map_or(true, |re_res| {
                re_res.as_ref().map_or(false, |re| {
                    re.is_match(
                        path.file_name()
                            .map(|osname| osname.to_str())
                            .flatten()
                            .unwrap_or(""),
                    )
                })
            }) {
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
        if s.is_empty() {
            Err(ParseLocationError {})
        } else {
            lazy_static! {
                static ref RE_OTHER: Regex = Regex::new("^([^:/]+)://").unwrap();
                static ref RE_SSH: Regex = Regex::new("^([^@]+)@([^:]+):|^([^:/@]+):").unwrap();
            }

            let protocol;
            let id: Vec<String>;

            if let Some(caps) = RE_OTHER.captures(s) {
                protocol = Some(Protocol::Other(caps[1].into()));
                id = RE_OTHER
                    .replace(s, "")
                    .split("/")
                    .map(|s| s.to_owned())
                    .collect();
            } else if let Some(caps) = RE_SSH.captures(s) {
                protocol = Some(Protocol::Ssh(
                    caps.get(1).map(|user| user.into()).unwrap_or("git").into(),
                ));
                id = RE_SSH
                    .replace(s, "$2$3/")
                    .split("/")
                    .map(|s| s.to_owned())
                    .collect();
            } else {
                protocol = None;
                id = s.split("/").map(|s| s.to_owned()).collect();
            }

            Ok(Self::new(protocol, id))
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
        check(
            "gituser@gitlab.com:rasmusmerzin/fr3",
            "gituser@gitlab.com:rasmusmerzin/fr3",
            "gitlab.com/rasmusmerzin/fr3",
            "gitlab.com/rasmusmerzin/fr3",
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

    #[test]
    fn error() {
        assert!(Location::from_str("").is_err());
    }
}
