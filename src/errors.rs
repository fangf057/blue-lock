
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
}