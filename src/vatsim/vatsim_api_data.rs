use crate::vatsim::vatsim_data_model::{VatsimStatusData, VatsimData};
use serde_json::from_str;
use rand::seq::SliceRandom;
use crate::vatsim::errors::VatsimDataAPIError;

pub struct VatsimApiUrls {
    pub data_urls: VatsimDataApiUrls,
    pub user_url: String,
    pub metar_url: String,
}

impl VatsimApiUrls {
    pub async fn new(status_url: &str) -> Result<Self, VatsimDataAPIError> {
        let vatsim_status_data: VatsimStatusData = from_str(&reqwest::get(status_url).await?.text().await?)?;
        let mut rng = rand::thread_rng();
        let data_urls = VatsimDataApiUrls {
            v3_url: vatsim_status_data.data.v3.choose(&mut rng).expect("Problem getting random data url").to_string(),
            transceivers_url: vatsim_status_data.data.transceivers.choose(&mut rng).expect("Problem getting random transceiver url").to_string(),
            servers_url: vatsim_status_data.data.servers.choose(&mut rng).expect("Problem getting random server url").to_string(),
        };
        let output = Self {
            data_urls,
            user_url: vatsim_status_data.user.choose(&mut rng).expect("Problem getting random user url").to_string(),
            metar_url: vatsim_status_data.metar.choose(&mut rng).expect("Problem getting random metar url").to_string(),
        };
        Ok(output)
    }
}

pub struct VatsimDataApiUrls {
    pub v3_url: String,
    pub transceivers_url: String,
    pub servers_url: String,
}

pub struct VatsimDataHandler {
    pub v3_data: VatsimData,
}

impl VatsimDataHandler {
    pub async fn new(urls: &VatsimDataApiUrls) -> Result<Self, VatsimDataAPIError> {
        let text = reqwest::get(&urls.v3_url).await?.text().await?;
        let v3_data = VatsimData::from_text(text);
        Ok(
            Self{v3_data}
        )
    }
}



