pub trait Recover<T> {
    fn validate_recover(shares: &[(u8, T)]) -> Result<(), String>;

    fn recover(shares: &[(u8, T)]) -> Result<T, String>;
}
