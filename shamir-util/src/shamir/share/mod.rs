pub trait Share<T> {
    fn validate_share(secret: &T, n: u8, k: u8) -> Result<(), String>;

    fn share(secret: &T, n: u8, k: u8) -> Result<Vec<(u8, T)>, String>;
}
