use std::str::FromStr;
use std::fmt;
use std::ops::Deref;

pub fn strip_after_space(input: &str) -> String {
  if let Some(index) = input.find(' ') {
      let stripped = &input[..index];
      String::from(stripped)
  } else {
      // If there is no space, return the original string
      String::from(input)
  }
}

/// a newtype for db url
pub struct DbUrl(String);

impl DbUrl {
  pub fn new(url: String) -> DbUrl {
      DbUrl(url)
  }

  pub fn as_str(&self) -> &str {
      // We didn't name the inner type, so it follows the same
      // naming convention as tuples. In other words, the inner
      // field is called `0`.
      &self.0
  }
}

// recommended to implement some standard traits when using newtype
impl FromStr for DbUrl {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DbUrl(s.to_string()))
    }
}

impl fmt::Display for DbUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for DbUrl {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}