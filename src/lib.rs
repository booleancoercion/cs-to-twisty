use std::fmt::Debug;

use chrono::{DateTime, Duration, TimeZone, Utc};
use json::JsonValue;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Penalty {
    Dnf,
    PlusTwo,
    None,
}

/// Represents a complete solve
pub struct Solve {
    pub duration: Duration,
    pub penalty: Penalty,
    pub scramble: String,
    pub datetime: DateTime<Utc>,
}

impl Solve {
    pub fn parse(solve: &JsonValue) -> Option<Self> {
        let (duration, penalty) = {
            let times = &solve[0];

            let offset = times[0].as_i64()?;
            let duration = times[1].as_i64()?;

            if offset == -1 {
                (duration, Penalty::Dnf)
            } else if offset == 2000 {
                (duration + offset, Penalty::PlusTwo)
            } else {
                (duration + offset, Penalty::None)
            }
        };

        let duration = Duration::milliseconds(duration);

        let scramble = solve[1].as_str()?.to_string();
        let datetime = Utc.timestamp(solve[3].as_i64()?, 0);

        Some(Self {
            duration,
            penalty,
            scramble,
            datetime,
        })
    }

    pub fn to_twisty_string(&self) -> String {
        let duration = {
            let minutes = self.duration.num_minutes();
            let millis = self.duration.num_milliseconds() - minutes * 60 * 1000;

            let sm = (millis as f64) / 1000.0;

            if minutes > 0 {
                format!("{}:{:05.2}", minutes, sm)
            } else {
                format!("{:.2}", sm)
            }
        };

        let dnf = if self.penalty == Penalty::Dnf {
            r#";"DNF""#
        } else {
            ""
        };

        format!(
            r#""{}";"{}";"{}"{}"#,
            duration,
            self.scramble,
            self.datetime.to_rfc3339(),
            dnf
        )
    }
}
