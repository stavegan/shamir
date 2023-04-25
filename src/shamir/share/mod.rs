pub trait Share<T> {

    fn share(secret: T, n: usize, k: u8) -> Vec<(u8, T)>;
}
