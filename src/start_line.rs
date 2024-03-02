use std::fmt;

use crate::uri::Uri;

use super::uri;

struct RequestLine {
    method: String,
    uri: uri::Uri,
    version: (u8, u8)
}

impl RequestLine {
    fn new(method: &str, uri: Uri) -> Self {
        Self{method: String::from(method), uri: uri, version: (2, 0)}
    }
}

impl fmt::Display for RequestLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} SIP/{}.{}", self.method, self.uri.to_string(), self.version.0, self.version.1)
    }
}

struct StatusLine {
    code: u16,
    reason: String,
    version: (u8, u8),
}

impl StatusLine {
    fn new(code: u16, reason: &str) -> Self {
        Self{code: code, reason: String::from(reason), version: (2, 0)}
    }
}

impl fmt::Display for StatusLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SIP/{}.{} {} {}", self.version.0, self.version.1, self.code, self.reason)
    }
}

pub enum StartLine {
    Request(RequestLine),
    Response(StatusLine),
}


#[cfg(test)]
mod test {
    use crate::uri::Uri;

    use super::{RequestLine, StatusLine};

    #[test]
    fn request_line_to_string() {
        let rl = RequestLine::new("INVITE", Uri::from_str("sip:alice@example.com").unwrap());
        assert_eq!(rl.to_string(), "INVITE sip:alice@example.com SIP/2.0")
    }

    #[test]
    fn status_line_to_string() {
        let sl = StatusLine::new(420, "Bad Extension");
        assert_eq!(sl.to_string(), "SIP/2.0 420 Bad Extension");
    }
}