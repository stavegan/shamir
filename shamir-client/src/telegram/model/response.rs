use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub ok: bool,
    pub result: T,
}
