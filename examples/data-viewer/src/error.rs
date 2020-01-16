use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    RestqError(#[from] restq::Error),
    #[error("error parsing header {0}")]
    HeaderParseError(restq::pom::Error),
    #[error("io error {0}")]
    HeaderIoError(io::Error),
    #[error("error parsing csv body")]
    CsvParseError,
}
