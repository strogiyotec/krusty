use actix_web::dev::Payload;
use awc::ClientResponse;
use awc::error::SendRequestError;
use log::{error, info};

/// Integration with UniBit API
/// to fetch the data by stock ticker
/// see it's here https://unibit.ai/api/docs/V2.0/

pub struct UniBitApi {
    api_token: String,
}

impl UniBitApi {
    pub fn new(api_token: String) -> Self {
        Self { api_token }
    }

    pub fn get_sector_by_ticker(&self, ticker: String) -> Result<String, Error> {
        let client = awc::Client::default();
        let url = format!("https://api.unibit.ai/v2/company/profile?tickers=AAPL&selectedFields=sector,&accessKey={access_key}", access_key = self.api_token);
        let response = client.get(url)
            .send()
            .await;
        return match response {
            Ok(mut response) => {
                let response_body = response.json::<serde_json::Value>().await?;
                let sector = response_body.get("result_data")
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .get("sector")
                    .unwrap()
                    .as_str();
                Ok(sector.to_string())
            }
            Err(err) => {
                error!("Error fetching sector from UniBit [{:?}]",err);
                Err(Error::HttpError)
            }
        };
    }
}

pub enum Error {
    HttpError,
}

