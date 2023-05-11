use config::{Config, File};
use shamir_client::telegram::model::SendPhotoRequest;
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
    let send_photo_request = SendPhotoRequest::from(2090208212, String::from("https://sun9-72.userapi.com/impg/LtMt96zzsoH4vuALy2Q8i6GfGuPfI6hDve5jvw/erZBvoFdRtg.jpg?size=640x640&quality=96&sign=ea7d014a57747d9fc9c6c1cb2b97794f&type=album"));
    let send_photo_request = SendPhotoRequest {
        disable_notification: Some(true),
        ..send_photo_request
    };
    let message = telegram_client.send_photo(send_photo_request);
    let message = message.await;
    println!("{message:?}");
    // loop {
    //     let updates = telegram_client.get_updates(818691025);
    //     let updates = updates.await;
    //     println!("{updates:?}");
    // }
}
