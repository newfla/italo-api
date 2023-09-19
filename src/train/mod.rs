use getset::Getters;
use serde::Deserialize;

/// Realtime data for a train
#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[get = "pub"]
pub struct TrainRealtime {
    last_update: String,
    train_schedule: TrainSchedule,
}

/// Train trip
#[derive(Deserialize, Debug, Getters)]
#[get = "pub"]
#[serde(rename_all = "PascalCase")]
pub struct TrainSchedule {
    /// Italo ID
    train_number: String,

    /// Rete Ferroviaria Italian ID
    rfi_train_number: String,

    /// Scheduled departure time
    #[serde(rename(deserialize = "DepartureDate"))]
    departure_time: String,

    ///First trip station name
    #[serde(rename(deserialize = "DepartureStationDescription"))]
    departure_station_name: String,

    /// Scheduled arrival time
    #[serde(rename(deserialize = "ArrivalDate"))]
    arrival_time: String,

    /// Terminus station
    #[serde(rename(deserialize = "ArrivalStationDescription"))]
    arrival_station_name: String,

    /// Service distruption data
    distruption: Distruption,

    /// Additional information on the first station
    #[serde(rename(deserialize = "StazionePartenza"))]
    departure_station: TrainStation,

    /// Stations where the train has already stopped
    #[serde(rename(deserialize = "StazioniFerme"))]
    stations_with_stop: Vec<TrainStation>,

    /// Stations where it will stop
    #[serde(rename(deserialize = "StazioniNonFerme"))]
    stations_with_transit: Vec<TrainStation>,
}

/// Distruption data
#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[get = "pub"]
pub struct Distruption {
    /// Delay (in minutes)
    delay_amount: i32,

    /// Unknown
    location_code: String,

    /// Unknown
    warning: bool,

    /// Unknown
    running_state: u16,
}

/// Station data enriched with train informations
#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
#[get = "pub"]
pub struct TrainStation {
    /// Italo station ID
    location_code: String,

    /// Human firendly name
    location_description: String,

    /// Rete Ferroviaria Italiana ID
    rfi_location_code: String,

    /// Estimated time by which the train will leave the station
    estimated_departure_time: String,

    /// Real time by which the train will leave the station
    actual_departure_time: String,

    /// Estimated time by which the train will arrive to the station
    estimated_arrival_time: String,

    /// Real time by which the train will arrive to the station
    actual_arrival_time: String,

    /// Platform
    #[serde(rename(deserialize = "ActualArrivalPlatform"))]
    platform: Option<String>,

    /// Station index in the trip plan
    #[serde(rename(deserialize = "StationNumber"))]
    sequence: u8,
}
