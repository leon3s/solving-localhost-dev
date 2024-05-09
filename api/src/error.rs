use ntex::web;
use ntex::http;
use ntex::web::error::BlockingError;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpError {
  #[serde(skip)]
  pub status: http::StatusCode,
  pub msg: String,
}

/// Helper function to display an HttpError
impl std::fmt::Display for HttpError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}] {}", self.status, self.msg)
  }
}

/// Implement standard error for HttpError
impl std::error::Error for HttpError {}

/// Helper function to convert an HttpError into a ntex::web::HttpResponse
impl web::WebResponseError for HttpError {
  fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
    web::HttpResponse::build(self.status).json(&self)
  }
}

pub struct NError {
  pub context: Option<String>,
  pub kind: std::io::ErrorKind,
  pub inner: Box<dyn std::error::Error + Send + Sync + 'static>,
}

impl NError {
  pub fn other<T>(error: T) -> Self
  where
    T: ToString,
  {
    NError {
      context: None,
      kind: std::io::ErrorKind::Other,
      inner: Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        error.to_string(),
      )),
    }
  }

  pub fn invalid_data<T>(error: T) -> Self
  where
    T: ToString,
  {
    NError {
      context: None,
      kind: std::io::ErrorKind::InvalidData,
      inner: Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        error.to_string(),
      )),
    }
  }
}

impl std::fmt::Debug for NError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(context) = &self.context {
      write!(f, "{}: {}", context, self.inner)
    } else {
      write!(f, "{}", self.inner)
    }
  }
}

impl std::fmt::Display for NError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(context) = &self.context {
      write!(f, "{}: {}", context, self.inner)
    } else {
      write!(f, "{}", self.inner)
    }
  }
}

pub trait NErrorCtx {
  fn context<T>(self, context: T) -> NError
  where
    T: ToString;
}

pub trait IntoNError {
  fn into_nerror(self) -> NError;
}

impl NErrorCtx for NError {
  fn context<T>(self, context: T) -> NError
  where
    T: ToString,
  {
    NError {
      context: Some(context.to_string()),
      ..self
    }
  }
}

impl<E> From<E> for NError
where
  E: std::error::Error + Send + Sync + 'static,
{
  #[cold]
  fn from(error: E) -> Self {
    NError {
      context: None,
      kind: std::io::ErrorKind::Other,
      inner: Box::new(error),
    }
  }
}

impl From<NError> for Box<dyn std::error::Error + Send + Sync + 'static> {
  #[cold]
  fn from(error: NError) -> Self {
    error.inner
  }
}

impl From<NError> for Box<dyn std::error::Error> {
  #[cold]
  fn from(error: NError) -> Self {
    error.inner
  }
}

pub trait NResultCtx<O, E> {
  fn set_err_ctx<F>(self, f: F) -> Result<O, NError>
  where
    F: ToString;
}

impl<O, E> NResultCtx<O, E> for Result<O, E>
where
  E: std::error::Error + Send + Sync + 'static,
{
  fn set_err_ctx<F>(self, ctx: F) -> Result<O, NError>
  where
    F: ToString,
  {
    self.map_err(|err| NError {
      context: Some(ctx.to_string()),
      kind: std::io::ErrorKind::Other,
      inner: Box::new(err),
    })
  }
}

impl<O> NResultCtx<O, NError> for Result<O, NError> {
  fn set_err_ctx<F>(self, ctx: F) -> Result<O, NError>
  where
    F: ToString,
  {
    self.map_err(|err| NError {
      context: Some(ctx.to_string()),
      ..err
    })
  }
}

pub type Result<T, E = NError> = std::result::Result<T, E>;

impl From<NError> for HttpError {
  fn from(err: NError) -> Self {
    let msg = err.to_string();
    let source = err.inner;
    if let Ok(source) = source.downcast::<std::io::Error>() {
      (*source).into()
    } else {
      HttpError {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        msg,
      }
    }
  }
}

impl From<std::io::Error> for HttpError {
  fn from(err: std::io::Error) -> Self {
    println!("processing error: {}", err);
    match err.kind() {
      std::io::ErrorKind::NotFound => HttpError {
        status: http::StatusCode::NOT_FOUND,
        msg: err.to_string(),
      },
      std::io::ErrorKind::PermissionDenied => HttpError {
        status: http::StatusCode::FORBIDDEN,
        msg: err.to_string(),
      },
      std::io::ErrorKind::ConnectionRefused => HttpError {
        status: http::StatusCode::FORBIDDEN,
        msg: err.to_string(),
      },
      std::io::ErrorKind::AlreadyExists => HttpError {
        status: http::StatusCode::CONFLICT,
        msg: err.to_string(),
      },
      std::io::ErrorKind::InvalidInput => HttpError {
        status: http::StatusCode::BAD_REQUEST,
        msg: err.to_string(),
      },
      std::io::ErrorKind::InvalidData => HttpError {
        status: http::StatusCode::BAD_REQUEST,
        msg: err.to_string(),
      },
      std::io::ErrorKind::TimedOut => HttpError {
        status: http::StatusCode::GATEWAY_TIMEOUT,
        msg: err.to_string(),
      },
      _ => HttpError {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        msg: err.to_string(),
      },
    }
  }
}

impl From<diesel::result::Error> for HttpError {
  fn from(err: diesel::result::Error) -> Self {
    println!("processing error: {}", err);
    match &err {
      diesel::result::Error::NotFound => HttpError {
        status: http::StatusCode::NOT_FOUND,
        msg: err.to_string(),
      },
      diesel::result::Error::DatabaseError(kind, info) => match kind {
        diesel::result::DatabaseErrorKind::UniqueViolation => HttpError {
          status: http::StatusCode::CONFLICT,
          msg: "already exists".into(),
        },
        diesel::result::DatabaseErrorKind::ForeignKeyViolation => HttpError {
          status: http::StatusCode::BAD_REQUEST,
          msg: info.hint().unwrap_or("unknown error").to_string(),
        },
        _ => HttpError {
          status: http::StatusCode::INTERNAL_SERVER_ERROR,
          msg: err.to_string(),
        },
      },
      _ => HttpError {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        msg: err.to_string(),
      },
    }
  }
}

impl From<BlockingError<HttpError>> for HttpError {
  fn from(err: BlockingError<HttpError>) -> Self {
    match err {
      BlockingError::Error(err) => err,
      BlockingError::Canceled => HttpError {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        msg: "thread pool is gone".into(),
      },
    }
  }
}
