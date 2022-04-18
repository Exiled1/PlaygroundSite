// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum AppError {
//     // Failed request.
//     #[error("Invalid/Failed request")]
//     BadRequest,
//     // 404 Not Found.
//     #[error("404 Not Found. Attempted to access: {path}. If this is an error, please contact the developer.")]
//     NotFound{
//         path: String
//     },
//     // 500 Internal Server Error.
//     #[error("500 Internal Server Error. Something occured server side that caused unintended behavior.")]
//     InternalServerError,
    
//     #[error(transparent)]
//     IoError(#[from] std::io::Error),
// }