use std::fmt;

struct Userinfo {
    user: String,
    password: Option<String>
}

impl Userinfo {
    pub fn new(user: &str, password: Option<&str>) -> Self {
        if let Some(pass) = password {
            Self{user: String::from(user), password: Some(String::from(pass))}
        } else {
            Self{user: String::from(user), password: None}
        }
    }
    pub fn from_string(s: &str) -> Result<Userinfo, &'static str> {
        Ok(Userinfo{user: String::from("alice"), password: Some(String::from("test"))})
    }
}

impl fmt::Display for Userinfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(i) = &self.password {
            write!(f, "{}:{}", self.user, i)

        } else {
            write!(f, "{}", self.user)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Userinfo;

    #[test]
    fn new_userinfo_with_password() {
        let userinfo = Userinfo::new("alice", Some("password"));
        assert_eq!(userinfo.to_string(), "alice:password")
    }

    #[test]
    fn new_userinfo_without_password() {
        let userinfo = Userinfo::new("alice", None);
        assert_eq!(userinfo.to_string(), "alice");
    }
}