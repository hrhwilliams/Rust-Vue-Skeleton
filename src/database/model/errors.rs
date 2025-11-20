pub enum DatabaseError {
    SqlxError(sqlx::Error),
    SerdeError(serde_json::Error),
    RngError,
}

impl From<sqlx::Error> for DatabaseError {
    fn from(value: sqlx::Error) -> Self {
        Self::SqlxError(value)
    }
}
