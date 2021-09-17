use chrono::{DateTime, Utc};
use thiserror::Error;

/// The error type for this crate.
#[derive(Error, Debug)]
pub enum Error {
    #[error("start_time `{0}` is ahead of current time")]
    StartTimeAheadOfCurrentTime(DateTime<Utc>),
    #[error("over the time limit")]
    OverTimeLimit,
    #[error("mutex is poisoned (i.e. a panic happened while it was locked)")]
    MutexPoisoned,
}