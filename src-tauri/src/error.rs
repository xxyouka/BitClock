use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub code: &'static str,
    pub message: String,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}

impl AppError {
    pub fn db(message: impl std::fmt::Display) -> Self {
        AppError { code: "DB_ERROR", message: message.to_string() }
    }

    pub fn lock(message: impl std::fmt::Display) -> Self {
        AppError { code: "LOCK_ERROR", message: message.to_string() }
    }

    pub fn registry(message: impl std::fmt::Display) -> Self {
        AppError { code: "REGISTRY_ERROR", message: message.to_string() }
    }

    pub fn io(message: impl std::fmt::Display) -> Self {
        AppError { code: "IO_ERROR", message: message.to_string() }
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::db(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::io(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_display() {
        let err = AppError::db("connection failed");
        assert_eq!(format!("{}", err), "[DB_ERROR] connection failed");
    }

    #[test]
    fn test_app_error_codes() {
        assert_eq!(AppError::db("x").code, "DB_ERROR");
        assert_eq!(AppError::lock("x").code, "LOCK_ERROR");
        assert_eq!(AppError::registry("x").code, "REGISTRY_ERROR");
        assert_eq!(AppError::io("x").code, "IO_ERROR");
    }

    #[test]
    fn test_from_rusqlite() {
        let sqlite_err = rusqlite::Error::ExecuteReturnedResults;
        let app_err: AppError = sqlite_err.into();
        assert_eq!(app_err.code, "DB_ERROR");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let app_err: AppError = io_err.into();
        assert_eq!(app_err.code, "IO_ERROR");
    }

    #[test]
    fn test_error_trait() {
        let err = AppError::lock("poisoned");
        let dyn_err: &dyn std::error::Error = &err;
        assert!(dyn_err.source().is_none());
    }
}
