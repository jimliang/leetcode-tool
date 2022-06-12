
// #[derive(Debug, thiserror::Error)]
// pub enum Error {
//   // #[error("`{0:?}`")]
//   // RequestError(#[from]  http_types::Error),
//   #[error("Empty error")]
//   Empty,
// }

pub type Result<T> = std::result::Result<T, anyhow::Error>;