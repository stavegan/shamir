mod get_updates {
    use super::{Deserialize, Serialize, Update};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Response {
        pub ok: bool,
        pub result: Vec<Update>,
    }
}

use awc::Client as AwcClient;
use serde::{Deserialize, Serialize};
use shamir_model::Update;
use shamir_settings::telegram::Settings;

pub struct Client {
    settings: Settings,
    client: AwcClient,
}

impl From<Settings> for Client {
    fn from(settings: Settings) -> Self {
        let client = AwcClient::builder().disable_timeout().finish();
        Client { settings, client }
    }
}

impl Client {
    pub async fn get_updates(&self, offset: u64) -> Vec<Update> {
        self.client
            .get(format!(
                "{}/bot{}/getUpdates?offset={offset}&timeout={}",
                self.settings.http.uri, self.settings.token, self.settings.http.timeout
            ))
            .send()
            .await
            .unwrap()
            .json::<get_updates::Response>()
            .await
            .map(|response| response.result)
            .unwrap()
    }
}
