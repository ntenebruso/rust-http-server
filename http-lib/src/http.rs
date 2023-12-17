use std::fmt;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum HttpMethod {
    #[default]
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
    Other(String),
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Head => write!(f, "HEAD"),
            Self::Post => write!(f, "Post"),
            Self::Put => write!(f, "Put"),
            Self::Delete => write!(f, "Delete"),
            Self::Connect => write!(f, "Connect"),
            Self::Options => write!(f, "Options"),
            Self::Trace => write!(f, "Trace"),
            Self::Patch => write!(f, "Patch"),
            Self::Other(method) => write!(f, "{}", method),
        }
    }
}

impl FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            "PATCH" => Ok(Self::Patch),
            _ => Ok(Self::Other(s.to_owned())),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum HttpStatusCode {
    #[default]
    Success = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Conflict = 409,
    InternalError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}

impl HttpStatusCode {
    pub fn default_reason_phrase(&self) -> String {
        match self {
            Self::Success => "OK".to_owned(),
            Self::BadRequest => "Bad Request".to_owned(),
            Self::Unauthorized => "Unauthorized".to_owned(),
            Self::Forbidden => "Forbidden".to_owned(),
            Self::NotFound => "Not Found".to_owned(),
            Self::Conflict => "Conflict".to_owned(),
            Self::InternalError => "Internal Error".to_owned(),
            Self::NotImplemented => "Not Implemented".to_owned(),
            Self::BadGateway => "Bad Gateway".to_owned(),
            Self::ServiceUnavailable => "Service Unavailable".to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct HttpError(pub HttpStatusCode, pub String);

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}\n{}", self.0 as u16, self.1)
    }
}
