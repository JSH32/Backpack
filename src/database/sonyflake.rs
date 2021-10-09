//! # Sonyflake implementation
//! Worker/machine IDs must be manually assigned, IPv4 may not be used
//!
//! ## Structure
//! 39 bits for time in units of 10 msec\
//!  8 bits for a sequence number\
//! 16 bits for a machine id

use thiserror::Error;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use chrono::{DateTime, TimeZone, Utc};

/// bit length of time
const BIT_LEN_TIME: u64 = 39;
/// bit length of sequence number
const BIT_LEN_SEQUENCE: u64 = 8;
/// bit length of machine id
const BIT_LEN_MACHINE_ID: u64 = 63 - BIT_LEN_TIME - BIT_LEN_SEQUENCE;

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

#[derive(Debug)]
struct WorkerState {
    pub elapsed_time: i64,
    pub sequence: u16,
}

pub struct SharedSonyflake {
    worker_id: u16,
    start_time: i64,
    worker_state: Mutex<WorkerState>,
}

/// Distributed unique ID generator.
/// This is thread safe
pub struct Sonyflake(pub Arc<SharedSonyflake>);

impl Sonyflake {
    /// Create a new [`Sonyflake`]
    ///
    /// # Arguments
    ///
    /// * `worker_id` - A unique identifier of the machine ID.
    /// if sonyflake is being used in a distributed setting no two machines must use the same ID
    ///
    /// * `start_time` - Optional start time for the mutex. If no time provided this will be set to 2014-09-01 00:00:00 +0000 UTC
    pub fn new(worker_id: u16, start_time: Option<DateTime<Utc>>) -> Result<Self, Error> {
        Ok(Self(Arc::new(SharedSonyflake {
            worker_id,
            start_time: match start_time {
                Some(time) => {
                    if time > Utc::now() {
                        return Err(Error::StartTimeAheadOfCurrentTime(time));
                    }

                    to_sonyflake_time(time)
                }
                None => to_sonyflake_time(Utc.ymd(2014, 9, 1).and_hms(0, 0, 0)),
            },
            worker_state: Mutex::new(WorkerState {
                elapsed_time: 0,
                sequence: 1 << (BIT_LEN_SEQUENCE - 1),
            }),
        })))
    }

    /// Generate the next unique id.
    /// After the Sonyflake time overflows, next_id returns an error.
    pub fn next_id(&self) -> Result<u64, Error> {
        let mut worker_state = self
            .0
            .worker_state
            .lock()
            .map_err(|_| Error::MutexPoisoned)?;

        let current = current_elapsed_time(self.0.start_time);
        if worker_state.elapsed_time < current {
            worker_state.elapsed_time = current;
            worker_state.sequence = 0;
        } else {
            worker_state.sequence = (worker_state.sequence + 1) & (1 << BIT_LEN_SEQUENCE) - 1;
            if worker_state.sequence == 0 {
                worker_state.elapsed_time += 1;
                let overtime = worker_state.elapsed_time - current;
                thread::sleep(sleep_time(overtime));
            }
        }

        if worker_state.elapsed_time >= 1 << BIT_LEN_TIME {
            return Err(Error::OverTimeLimit);
        }

        Ok(
            (worker_state.elapsed_time as u64) << (BIT_LEN_SEQUENCE + BIT_LEN_MACHINE_ID)
                | (worker_state.sequence as u64) << BIT_LEN_MACHINE_ID
                | (self.0.worker_id as u64),
        )
    }
}

/// Returns a new `Sonyflake` referencing the same state as `self`.
impl Clone for Sonyflake {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

const SONYFLAKE_TIME_UNIT: i64 = 10_000_000; // nanoseconds, i.e. 10msec

fn to_sonyflake_time(time: DateTime<Utc>) -> i64 {
    time.timestamp_nanos() / SONYFLAKE_TIME_UNIT
}

fn current_elapsed_time(start_time: i64) -> i64 {
    to_sonyflake_time(Utc::now()) - start_time
}

fn sleep_time(overtime: i64) -> Duration {
    Duration::from_millis(overtime as u64 * 10)
        - Duration::from_nanos((Utc::now().timestamp_nanos() % SONYFLAKE_TIME_UNIT) as u64)
}
