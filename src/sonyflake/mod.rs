pub mod error;

use std::{collections::HashMap, sync::{Arc, Mutex}, thread, time::Duration};
use chrono::{DateTime, TimeZone, Utc};

use self::{error::Error};

/// bit length of time
const BIT_LEN_TIME: u64 = 39;
/// bit length of sequence number
const BIT_LEN_SEQUENCE: u64 = 8;
/// bit length of machine id
const BIT_LEN_MACHINE_ID: u64 = 63 - BIT_LEN_TIME - BIT_LEN_SEQUENCE;

#[derive(Debug)]
pub struct Internals {
    pub elapsed_time: i64,
    pub sequence: u16,
}

pub struct SharedSonyflake {
    pub machine_id: u16,
    pub start_time: i64,
    pub internals: Mutex<Internals>,
}

/// Sonyflake is a distributed unique ID generator.
pub struct Sonyflake(pub Arc<SharedSonyflake>);

impl Sonyflake {
    /// Create a new [`Sonyflake`]
    ///
    /// # Arguments
    ///
    /// * `machine_id` - A unique identifier of the machine ID.
    /// if sonyflake is being used in a distributed setting no two machines must use the same ID
    ///
    /// * `start_time` - Optional start time for the mutex. If no time provided this will be set to 2014-09-01 00:00:00 +0000 UTC
    pub fn new(machine_id: u16, start_time: Option<DateTime<Utc>>) -> Result<Self, Error> {
        Ok(Self(Arc::new(SharedSonyflake {
            machine_id,
            start_time: match start_time {
                Some(time) => {
                    if time > Utc::now() {
                        return Err(Error::StartTimeAheadOfCurrentTime(time));
                    }

                    to_sonyflake_time(time)
                },
                None => to_sonyflake_time(Utc.ymd(2014, 9, 1).and_hms(0, 0, 0))
            },
            internals: Mutex::new(Internals {
                elapsed_time: 0,
                sequence: 1 << (BIT_LEN_SEQUENCE - 1),
            }),
        })))
    }

    // pub(crate) fn new_inner(shared: Arc<SharedSonyflake>) -> Self {
    //     Self(shared)
    // }

    /// Generate the next unique id.
    /// After the Sonyflake time overflows, next_id returns an error.
    pub fn next_id(&mut self) -> Result<u64, Error> {
        let mut internals = self.0.internals.lock().map_err(|_| Error::MutexPoisoned)?;

        let current = current_elapsed_time(self.0.start_time);
        if internals.elapsed_time < current {
            internals.elapsed_time = current;
            internals.sequence = 0;
        } else {
            // self.elapsed_time >= current
            internals.sequence = (internals.sequence + 1) & (1 << BIT_LEN_SEQUENCE) - 1;
            if internals.sequence == 0 {
                internals.elapsed_time += 1;
                let overtime = internals.elapsed_time - current;
                thread::sleep(sleep_time(overtime));
            }
        }

        if internals.elapsed_time >= 1 << BIT_LEN_TIME {
            return Err(Error::OverTimeLimit);
        }

        Ok(
            (internals.elapsed_time as u64) << (BIT_LEN_SEQUENCE + BIT_LEN_MACHINE_ID)
                | (internals.sequence as u64) << BIT_LEN_MACHINE_ID
                | (self.0.machine_id as u64),
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

/// Break a Sonyflake ID up into its parts.
pub fn decompose(id: u64) -> HashMap<String, u64> {
    let mut map = HashMap::new();

    let mask_sequence = ((1 << BIT_LEN_SEQUENCE) - 1) << BIT_LEN_MACHINE_ID;
    let mask_machine_id = (1 << BIT_LEN_MACHINE_ID) - 1;

    map.insert("id".into(), id);
    map.insert("msb".into(), id >> 63);
    map.insert("time".into(), id >> (BIT_LEN_SEQUENCE + BIT_LEN_MACHINE_ID));
    map.insert(
        "sequence".into(),
        (id & mask_sequence) >> BIT_LEN_MACHINE_ID,
    );
    map.insert("machine-id".into(), id & mask_machine_id);

    map
}