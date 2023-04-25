pub trait Recover<T> {

    fn recover(secret: &[(u8, T)]) -> T;
}
