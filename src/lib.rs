use std::collections::HashMap;

use anyhow::Context;
use login::{LoginRequestBody, LoginResponse};
use reqwest::Client;
pub use station::{Station, StationRealtime, StationTrainRealtime};
use station::{StationCode, StationLabel};
pub use train::{Distruption, TrainRealtime, TrainSchedule, TrainStation};

static LOGIN_ENDPOINT: &str = "https://big.ntvspa.it/BIG/v7/Rest/SessionManager.svc/Login";
static STATION_LIST_ENDPOINT: &str = "https://italoinviaggio.italotreno.it/it/stazione";
static STATION_REALTIME_ENDPOINT: &str =
    "https://italoinviaggio.italotreno.it/api/RicercaStazioneService?&CodiceStazione=";
static TRAIN_REALTIME_ENDPOINT: &str =
    "https://italoinviaggio.italotreno.it/api/RicercaTrenoService?&TrainNumber=";

mod login;
mod station;
mod train;

/// Use this struct to access italotreno API.
///
/// Use [`Self::default()`] to instantiate the interface.
///
#[derive(Default)]
pub struct ItaloApi {
    signature: Option<LoginResponse>,
    client: Client,
}

impl ItaloApi {
    fn is_initialized(&self) -> bool {
        self.signature.is_some()
    }

    async fn init(&mut self) -> anyhow::Result<()> {
        self.signature = Some(
            self.client
                .post(LOGIN_ENDPOINT)
                .json(&LoginRequestBody::default())
                .send()
                .await?
                .json()
                .await?,
        );
        Ok(())
    }

    /// Retrieves stations recognized by the italotreno information system.
    ///
    /// The struct contains internal Ids used by [`Self::station_realtime()`]
    pub async fn station_list(&self) -> anyhow::Result<Vec<Station>> {
        let res = self
            .client
            .get(STATION_LIST_ENDPOINT)
            .send()
            .await?
            .text()
            .await?;

        let raw_lists = res
            .split_once("ItaloInViaggio.Resources.stationList = ")
            .context("stationList not found")?
            .1
            .split_once("ItaloInViaggio.Resources.stationCoding = ")
            .context("stationCoding not found")?;

        let label_list: Vec<StationLabel> =
            serde_json::from_str(raw_lists.0.trim_end().trim_end_matches(';'))?;

        let code_list: Vec<StationCode> = serde_json::from_str(
            raw_lists
                .1
                .split_once("ItaloInViaggio.Resources.localizzation")
                .context("localizzation not found")?
                .0
                .trim_end()
                .trim_end_matches(';'),
        )?;

        let label_map = label_list
            .iter()
            .map(|elem| (elem.value(), elem.label()))
            .collect::<HashMap<_, _>>();

        Ok(code_list
            .iter()
            .map(|elem| {
                Station::new(
                    elem.code().to_owned(),
                    elem.url_coding().to_owned(),
                    label_map
                        .get(elem.code())
                        .unwrap_or(&&"".to_string())
                        .to_string(),
                )
            })
            .filter(|elem| !elem.name().is_empty())
            .collect())
    }

    /// Retrieve the departure and arivval boards for a station using [`Self::station_realtime()`]
    pub async fn station_realtime(&self, station: Station) -> anyhow::Result<StationRealtime> {
        Ok(self
            .client
            .get(STATION_REALTIME_ENDPOINT.to_string() + station.code())
            .send()
            .await?
            .json()
            .await?)
    }

    /// Retrieve realtime data on a moving train
    pub async fn train_realtime(&self, train_code: &str) -> anyhow::Result<TrainRealtime> {
        Ok(self
            .client
            .get(TRAIN_REALTIME_ENDPOINT.to_string() + train_code)
            .send()
            .await?
            .json()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let mut api = ItaloApi::default();
        assert!(!api.is_initialized());

        assert!(api.init().await.is_ok());
        assert!(api.is_initialized());

        let stations = api.station_list().await;
        println!("{:?}", stations);
        println!();
        assert!(stations.is_ok_and(|f| f.len() > 0));

        let station_realtime = api
            .station_realtime(Station::new(
                "MC_".to_string(),
                "milano-centrale".to_string(),
                "Milano Centrale".to_string(),
            ))
            .await;
        println!("{:?}", station_realtime);
        println!();
        assert!(station_realtime
            .is_ok_and(|f| f.arrival_board().len() > 0 && f.departure_board().len() > 0));

        let train_realtime = api.train_realtime("8158").await;
        println!("{:?}", train_realtime);
        println!();
    }
}
