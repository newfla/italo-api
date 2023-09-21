use derive_new::new;
use getset::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Getters)]
#[serde(rename_all = "camelCase")]
#[get = "pub"]
pub struct StationCode {
    code: String,
    url_coding: String,
}

#[derive(Deserialize, Debug, Getters)]
#[get = "pub"]
pub struct StationLabel {
    label: String,
    value: String,
}

/// Station metadata
#[derive(Debug, Getters, new)]
#[get = "pub"]
pub struct Station {
    /// Internal italotreno ID
    code: String,

    /// Partial URL to access <https://italoinviaggio.italotreno.it/it/stazione>
    url_coding: String,

    /// Human friendly station name
    name: String,
}

/// Abstraction over departure and departure board for a station
#[derive(Deserialize, Debug, Getters)]
#[get = "pub"]
pub struct StationRealtime {
    #[serde(rename(deserialize = "ListaTreniArrivo"))]
    arrival_board: Vec<StationTrainRealtime>,

    #[serde(rename(deserialize = "ListaTreniPartenza"))]
    departure_board: Vec<StationTrainRealtime>,
}

/// Train data during its stay at the station
#[derive(Deserialize, Debug, Getters)]
#[get = "pub"]
pub struct StationTrainRealtime {
    /// Train number
    #[serde(rename(deserialize = "Numero"))]
    number: String,

    /// Train end point
    #[serde(rename(deserialize = "DescrizioneLocalita"))]
    destination: String,

    /// Scheduled departure time
    #[serde(rename(deserialize = "OraPassaggio"))]
    scheduled_time: String,

    /// Real departure time
    #[serde(rename(deserialize = "NuovoOrario"))]
    forecast_time: String,

    /// Train platform
    #[serde(rename(deserialize = "Binario"))]
    platform: String,

    /// Generic trip description
    #[serde(rename(deserialize = "Descrizione"))]
    description: String,
}
