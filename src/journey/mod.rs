use crate::Station;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Ok;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use derive_new::new;
use getset::Getters;
use getset::Setters;
use serde::Deserialize;
use serde::Serialize;

static DATE_TIME_PATTERN: &str = "/Date(%s000+0000)/";
pub type RoundTrip = (bool, Option<DateTime<Utc>>, Option<DateTime<Utc>>);

fn extract_utc_time(val: &str) -> anyhow::Result<DateTime<Utc>> {
    DateTime::from_timestamp(
        val.split_once('(')
            .context("Failed to extract dateTime")?
            .1
            .split_once('+')
            .context("Failed to extract dateTime")?
            .0
            .parse::<i64>()
            .expect("Timestamp conversion failed")
            / 1000,
        0,
    )
    .context("invalid timestamp")
}

#[derive(Serialize, Debug, new)]
#[serde(rename_all = "PascalCase")]
pub struct InternalJourneyRequest<'a> {
    signature: &'a str,
    source_system: u8,
    get_available_trains: &'a JourneyRequest,
}

/// Input object for [crate::ItaloApi::find_journeys]
#[derive(Serialize, Debug, Setters)]
#[serde(rename_all = "PascalCase")]
#[set = "pub"]
pub struct JourneyRequest {
    #[getset(skip)]
    departure_station: String,

    #[getset(skip)]
    arrival_station: String,

    #[getset(skip)]
    interval_start_date_time: String,

    #[getset(skip)]
    interval_end_date_time: String,

    /// Set the number of adults
    adult_number: u8,

    /// Set the number of children
    child_number: u8,

    /// Set the number of infants
    infant_number: u8,

    /// Set the number of seniors
    senior_number: u8,

    /// Ignore interval dateTime values
    override_interval_time_restriction: bool,

    /// Currency for the amount. Default is EUR.
    currency_code: String,

    #[getset(skip)]
    is_guest: bool,

    #[getset(skip)]
    round_trip: bool,

    #[getset(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    round_trip_interval_start_date_time: Option<String>,

    #[getset(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    round_trip_interval_end_date_time: Option<String>,
}

impl Default for JourneyRequest {
    fn default() -> Self {
        Self {
            departure_station: Default::default(),
            arrival_station: Default::default(),
            interval_start_date_time: Default::default(),
            interval_end_date_time: Default::default(),
            adult_number: 1,
            child_number: 0,
            infant_number: 0,
            senior_number: 0,
            override_interval_time_restriction: false,
            currency_code: "EUR".to_owned(),
            is_guest: true,
            round_trip: false,
            round_trip_interval_start_date_time: Default::default(),
            round_trip_interval_end_date_time: Default::default(),
        }
    }
}

impl JourneyRequest {
    /// Set the departure station for the journey search
    pub fn set_departure_station(&mut self, val: Station) -> &mut Self {
        self.departure_station = val.code().to_owned();
        self
    }

    /// Set the arrival station for the journey search
    pub fn set_arrival_station(&mut self, val: Station) -> &mut Self {
        self.arrival_station = val.code().to_owned();
        self
    }

    /// Set the DateTime from which to start the search for journeys
    pub fn set_interval_start_date_time(&mut self, val: DateTime<Utc>) -> &mut Self {
        self.interval_start_date_time = val.format(DATE_TIME_PATTERN).to_string();
        self
    }

    /// Set the DateTime limit for the search for journeys
    pub fn set_interval_end_date_time(&mut self, val: DateTime<Utc>) -> &mut Self {
        self.interval_end_date_time = val.format(DATE_TIME_PATTERN).to_string();
        self
    }

    /// Set data to search for round trip solutions
    pub fn set_round_trip(&mut self, val: RoundTrip) -> anyhow::Result<&mut Self> {
        match val {
            (false, _, _) => {
                self.round_trip = false;
                self.round_trip_interval_start_date_time = None;
                self.round_trip_interval_end_date_time = None;
                Ok(self)
            }
            (true, Some(start), Some(end)) => {
                self.round_trip = true;
                self.round_trip_interval_start_date_time =
                    Some(start.format(DATE_TIME_PATTERN).to_string());
                self.round_trip_interval_end_date_time =
                    Some(end.format(DATE_TIME_PATTERN).to_string());
                Ok(self)
            }
            (true, _, _) => Err(anyhow!(
                "Round trip requires both valued date_time, got {:?}",
                val
            )),
        }
    }
}

/// Output object for [crate::ItaloApi::find_journeys]
#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[get = "pub"]
pub struct JourneyResults {
    /// Array of alternative solutions
    #[serde(rename(deserialize = "JourneyDateMarkets"))]
    solutions: Vec<JourneysSolution>,
}

/// Array of journeys for the date specified by [JourneysSolution::departure_date]
#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[get = "pub"]
pub struct JourneysSolution {
    #[getset(skip)]
    departure_date: String,

    /// Array of journeys for the specified date
    journeys: Vec<Journey>,
}

impl JourneysSolution {
    /// Date on which the journeys are valid
    pub fn departure_date(&self) -> anyhow::Result<NaiveDate> {
        //Something is wrong on italo side
        Ok(NaiveDate::from(
            NaiveDateTime::from_timestamp_millis(
                self.departure_date
                    .split_once('(')
                    .context("Failed to extract date")?
                    .1
                    .split_once('+')
                    .context("Failed to extract date")?
                    .0
                    .parse()
                    .context("Failed to parse timestamp")?,
            )
            .context("Failed to parse Date")?,
        ))
    }
}

/// Describes a journey using one or more trains
#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[get = "pub"]
pub struct Journey {
    /// Different parts by which the journey has been divided
    segments: Vec<JourneySegment>,
}

/// Single train journey
#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[get = "pub"]
pub struct JourneySegment {
    #[serde(rename(deserialize = "STD"))]
    #[getset(skip)]
    departure_time: String,

    #[serde(rename(deserialize = "STA"))]
    #[getset(skip)]
    arrival_time: String,

    /// Italo train ID
    train_number: String,

    /// Describes the train as direct
    no_stop_train: bool,

    /// Train Stops
    #[serde(rename(deserialize = "Legs"))]
    stops: Vec<Stop>,
}

impl JourneySegment {
    /// Departure time
    pub fn departure_time(&self) -> anyhow::Result<DateTime<Utc>> {
        extract_utc_time(&self.departure_time)
    }

    /// Arrival time
    pub fn arrival_time(&self) -> anyhow::Result<DateTime<Utc>> {
        extract_utc_time(&self.arrival_time)
    }
}

/// Train stop
#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[get = "pub"]
pub struct Stop {
    #[serde(rename(deserialize = "STD"))]
    #[getset(skip)]
    departure_time: String,

    #[serde(rename(deserialize = "STA"))]
    #[getset(skip)]
    arrival_time: String,

    /// Departure station
    departure_station: String,

    /// Arrival station
    arrival_station: String,
}

impl Stop {
    /// Departure time
    pub fn departure_time(&self) -> anyhow::Result<DateTime<Utc>> {
        extract_utc_time(&self.departure_time)
    }
    /// Arrival time
    pub fn arrival_time(&self) -> anyhow::Result<DateTime<Utc>> {
        extract_utc_time(&self.arrival_time)
    }
}
