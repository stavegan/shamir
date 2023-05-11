pub mod model;

use awc::{Client as AwcClient, SendClientRequest};
use serde::de::DeserializeOwned;
use shamir_model::telegram::Message;
use shamir_model::telegram::MessageId;
use shamir_model::telegram::Update;
use shamir_model::telegram::User;
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
    async fn parse<T: DeserializeOwned>(request: SendClientRequest) -> T {
        request
            .await
            .unwrap()
            .json::<model::Response<T>>()
            .await
            .map(|response| response.result)
            .unwrap()
    }

    pub async fn get_updates(&self, offset: u64) -> Vec<Update> {
        Self::parse(
            self.client
                .get(format!(
                    "{}/bot{}/getUpdates?offset={offset}&timeout={}",
                    self.settings.http.uri, self.settings.token, self.settings.http.timeout
                ))
                .send(),
        )
        .await
    }

    pub async fn get_me(&self) -> User {
        Self::parse(
            self.client
                .get(format!(
                    "{}/bot{}/getMe",
                    self.settings.http.uri, self.settings.token
                ))
                .send(),
        )
        .await
    }

    pub async fn log_out(&self) -> bool {
        Self::parse(
            self.client
                .get(format!(
                    "{}/bot{}/logOut",
                    self.settings.http.uri, self.settings.token
                ))
                .send(),
        )
        .await
    }

    pub async fn close(&self) -> bool {
        Self::parse(
            self.client
                .get(format!(
                    "{}/bot{}/close",
                    self.settings.http.uri, self.settings.token
                ))
                .send(),
        )
        .await
    }

    pub async fn send_message(&self, request: &model::SendMessageRequest) -> Message {
        Self::parse(
            self.client
                .post(format!(
                    "{}/bot{}/sendMessage",
                    self.settings.http.uri, self.settings.token
                ))
                .send_json(request),
        )
        .await
    }

    pub async fn forward_message(&self, request: &model::ForwardMessageRequest) -> Message {
        Self::parse(
            self.client
                .post(format!(
                    "{}/bot{}/forwardMessage",
                    self.settings.http.uri, self.settings.token
                ))
                .send_json(request),
        )
        .await
    }

    pub async fn copy_message(&self, request: &model::CopyMessageRequest) -> MessageId {
        Self::parse(
            self.client
                .post(format!(
                    "{}/bot{}/copyMessage",
                    self.settings.http.uri, self.settings.token
                ))
                .send_json(request),
        )
        .await
    }
}
