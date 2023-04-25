use rand::{Rng, thread_rng};
use crate::shamir::recover::Recover;
use crate::shamir::share::Share;

pub mod recover;
pub mod share;

pub struct Shamir {
    poly: Vec<u8>,
}

impl Shamir {

    fn from(secret: &u8, k: u8) -> Self {
        let mut poly = vec![secret.clone()];
        let mut rng = thread_rng();
        for _ in 1..k {
            let coefficient: u8 = rng.gen();
            poly.push(coefficient)
        }
        Self { poly }
    }

    fn evaluate(&self, x: u8) -> u8 {
        let mut b = 0u8;
        let range = 0..self.poly.len();
        for i in range.rev() {
            b = b * x + self.poly[i];
        }
        b
    }

}

impl Share<u8> for Shamir {

    fn share(secret: u8, n: usize, k: u8) -> Vec<(u8, u8)> {
        let mut shares = vec![(0u8, 0u8); n];
        let shamir = Self::from(&secret, k);
        for i in 0..n {
            let x = (i + 1) as u8;
            let y = shamir.evaluate(x);
            shares[i].0 = x;
            shares[i].1 = y;
        }
        shares
    }

}

impl Recover<u8> for Shamir {

    fn recover(shares: &[(u8, u8)]) -> u8 {
        let mut recovered = 0u8;
        for i in 0..shares.len() {
            let mut numerator = 1i64;
            let mut denominator = 1i64;
            let x_i = shares[i].0 as i64;
            for j in 0..shares.len() {
                if i != j {
                    let x = shares[j].0 as i64;
                    numerator *= x;
                    denominator *= x - x_i;
                }
            }
            recovered += shares[i].1 * (numerator / denominator) as u8;
        }
        recovered
    }

}

impl Share<Vec<u8>> for Shamir {

    fn share(secret: Vec<u8>, n: usize, k: u8) -> Vec<(u8, Vec<u8>)> {
        let mut shares = vec![(0u8, Vec::new()); n];
        for i in 0..secret.len() {
            let temp = Shamir::share(secret[i], n, k);
            for j in 0..temp.len() {
                shares[j].0 = temp[j].0;
                shares[j].1.push(temp[j].1)
            }
        }
        shares
    }

}

impl Recover<Vec<u8>> for Shamir {

    fn recover(secret: &[(u8, Vec<u8>)]) -> Vec<u8> {
        let mut recovered = Vec::new();
        let mut i = 0usize;
        'outer: loop {
            let mut temp = Vec::new();
            for j in 0..secret.len() {
                if i == secret[j].1.len() {
                    break 'outer;
                }
                temp.push((secret[j].0, secret[j].1[i]));
            }
            recovered.push(Shamir::recover(&temp));
            i += 1;
        }
        recovered
    }

}
