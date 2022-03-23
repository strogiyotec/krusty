use actix_web::dev::Payload;
use awc::{ClientResponse, Connector};
use awc::error::SendRequestError;
use log::{error, info};
use openssl::ssl::{SslConnector, SslMethod};

use crate::router::stock_router::ErrorMessage;

/// Integration with UniBit API
/// to fetch the data by stock ticker
/// see it's here https://unibit.ai/api/docs/V2.0/
#[derive(Clone)]
pub struct UniBitApi {
    api_token: String,
}

impl UniBitApi {
    pub fn new(api_token: String) -> Self {
        Self { api_token }
    }

    pub async fn get_sector_by_ticker(&self, ticker: &String) -> Result<String, ErrorMessage> {
        let client = awc::Client::new();
        let url = format!("https://api.unibit.ai/v2/company/profile?tickers={ticker}&selectedFields=industry,&accessKey={access_key}", access_key = self.api_token, ticker = ticker);
        let response = client.get(url)
            .send()
            .await;
        return match response {
            Ok(mut response) => {
                let response_body = response.json::<serde_json::Value>().await.expect("Can't get json on sector_by_ticket from UniBit API");
                let sector = response_body.get("result_data")
                    .unwrap()
                    .get(ticker)
                    .unwrap()
                    .get("industry")
                    .unwrap()
                    .as_str()
                    .unwrap();
                Ok(sector.to_string())
            }
            Err(err) => {
                error!("Error fetching sector from UniBit [{:?}]",err);
                return Err(ErrorMessage { message: format!("{:?}", err) });
            }
        };
    }
}


