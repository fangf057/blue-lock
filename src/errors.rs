
use snafu::Snafu;

pub type AppResult <T> = Result<T, AppError>;


#[derive(Snafu, Debug)]
pub enum AppError {
    #[snafu(display("Error sending sample"))]
    SampleSendError { },
    #[snafu(display("Error processing sample: {}",r))]
    ProcessingError{r:String},
    #[snafu(display("Insufficient data for detection"))]
    InsufficientData,
    #[snafu(display("db error: {}",source))]
    DbError{source:sea_orm::DbErr},
    #[snafu(display("Error serializing data: {}",source))]
    InvalidData { source: serde_json::Error },
    #[snafu(display(" not found"))]
    NotFound ,
}