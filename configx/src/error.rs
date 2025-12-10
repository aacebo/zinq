#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "json")]
    Json(serde_json::Error),

    #[cfg(feature = "yml")]
    Yml(serde_yml::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "json")]
            Self::Json(err) => write!(f, "{}", err),

            #[cfg(feature = "yml")]
            Self::Yml(err) => write!(f, "{}", err),

            #[allow(unused)]
            v => write!(f, "{:#?}", v),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(feature = "json")]
            Self::Json(err) => err.source(),

            #[cfg(feature = "yml")]
            Self::Yml(err) => err.source(),

            #[allow(unused)]
            _ => None,
        }
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        return Self::Json(value);
    }
}

#[cfg(feature = "json")]
impl Into<serde_json::Error> for Error {
    fn into(self) -> serde_json::Error {
        match self {
            Self::Json(err) => err,
            _ => panic!("expected json error"),
        }
    }
}

#[cfg(feature = "yml")]
impl From<serde_yml::Error> for Error {
    fn from(value: serde_yml::Error) -> Self {
        return Self::Yml(value);
    }
}

#[cfg(feature = "yml")]
impl Into<serde_yml::Error> for Error {
    fn into(self) -> serde_yml::Error {
        match self {
            Self::Yml(err) => err,
            _ => panic!("expected yml error"),
        }
    }
}
