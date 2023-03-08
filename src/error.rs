use std::{error, fmt};
use std::fmt::Formatter;

#[derive(Clone)]
pub enum ImageType {
    Png
}

impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PNG"))
    }
}

#[derive(Clone)]
pub enum ErrorKind {
    InvalidValue(ImageType)
}

impl ErrorKind {
    pub fn description(&self) -> String {
        match self {
            ErrorKind::InvalidValue(image) => format!("Invalid value entered. ImageType: {}",image),
        }
    }
}

pub struct ImageError {
    _error: _ImageError,
}

impl ImageError {
    pub(crate) fn new<E>(kind: ErrorKind, error: E) -> Self
        where
            E: Into<Box<dyn error::Error + Send + Sync>>
    {
        ImageError { _error: _ImageError::Custom((kind, error.into())) }
    }

    pub(crate) fn new_simple(kind: ErrorKind) -> Self {
        ImageError { _error: _ImageError::Simple(kind) }
    }

    fn kind(&self) -> ErrorKind {
        match &self._error {
            _ImageError::Simple(s) => s.clone(),
            _ImageError::Custom(c) => c.0.clone(),
        }
    }
}

enum _ImageError {
    Simple( ErrorKind ),
    Custom( (ErrorKind, Box<dyn error::Error + Send + Sync>) ),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.kind().description())
    }
}

impl fmt::Debug for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl error::Error for ImageError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self._error {
            _ImageError::Simple(_) => None,
            _ImageError::Custom(c) => c.1.source()
        }
    }
}