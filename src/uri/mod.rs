mod userinfo;
use std::fmt;

pub struct Uri {
    schema: String,
    userinfo: String,
    hostport: String
}

impl Uri {
    pub fn from_str(s: &str) -> Result<Uri, &'static str> {
        let mut splitter = s.splitn(2, ":");
        let schema = splitter.next().unwrap();
        let userinfo_and_hostport = splitter.next().unwrap();
        let mut splitter = userinfo_and_hostport.splitn(2, "@");
        let (userinfo, hostport) = (splitter.next().unwrap(), splitter.next().unwrap());
        Ok(Uri{schema: schema.to_string(), userinfo: userinfo.to_string(), hostport: hostport.to_string()})
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}@{}", self.schema, self.userinfo, self.hostport)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn uri_to_string() {
        assert_eq!(Uri{schema: String::from("sip"), userinfo: String::from("alice"), hostport: String::from("example.com")}.to_string(), "sip:alice@example.com")
    }

    #[test]
    fn uri_from_string() {
        let uri = Uri::from_str("sip:alice@example.com").unwrap();
        assert_eq!(uri.schema, "sip");
        assert_eq!(uri.userinfo, "alice");
        assert_eq!(uri.hostport, "example.com");
    }

}