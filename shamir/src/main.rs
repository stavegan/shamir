use config::{Config, File};
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
    loop {
        let updates = telegram_client.get_updates(818691024);
        let updates = updates.await;
        println!("{updates:?}");
    }
}
