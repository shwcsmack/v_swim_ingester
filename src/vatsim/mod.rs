use crate::vatsim::errors::VatsimDataAPIError;
use crate::vatsim::vatsim_api_data::{VatsimApiUrls, VatsimDataHandler};

mod errors;
pub mod vatsim_api_data;
pub(crate) mod vatsim_data_model;

pub struct Vatsim {
    pub data_handler: VatsimDataHandler,
}

impl Vatsim {
    pub fn new() -> VatsimBuilder {
        VatsimBuilder {
            api_status_url: "https://status.vatsim.net/status.json".to_string(),
        }
    }
}

pub struct VatsimBuilder {
    api_status_url: String,
}

impl VatsimBuilder {
    pub fn with_status_url(mut self, url: &str) -> Self {
        self.api_status_url = url.to_string();
        self
    }

    pub async fn build(self) -> Result<Vatsim, VatsimDataAPIError> {
        let api_urls_handler = VatsimApiUrls::new(&self.api_status_url).await?;
        let data_handler = VatsimDataHandler::new(&api_urls_handler.data_urls).await?;
        let vatsim = Vatsim{
            data_handler,
        };
        Ok(vatsim)
    }
}
