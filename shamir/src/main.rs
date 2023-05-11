use config::{Config, File};
use shamir_client::telegram::model::CopyMessageRequest;
use shamir_client::telegram::Client as TelegramClient;
use shamir_settings::Settings;

const SETTINGS_FILE_NAME: &'static str = "shamir/Settings.toml";

#[actix_rt::main]
async fn main() {
    let settings = Config::builder()
        .add_source(File::with_name(SETTINGS_FILE_NAME))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap();
    println!("{settings:?}");
    let telegram_client = TelegramClient::from(settings.telegram);
    let copy_message_request = CopyMessageRequest::from(2090208212, 2090208212, 13);
    let message_id = telegram_client.copy_message(&copy_message_request);
    let message_id = message_id.await;
    println!("{message_id:?}");
    // loop {
    //     let updates = telegram_client.get_updates(818691025);
    //     let updates = updates.await;
    //     println!("{updates:?}");
    // }
}
