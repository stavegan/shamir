use crate::shamir::recover::Recover;
use crate::shamir::share::Share;
use rand::{thread_rng, Rng};

pub mod recover;
pub mod share;

pub struct Shamir {
    poly: Vec<u8>,
}

impl Shamir {
    fn from(secret: &u8, k: u8) -> Result<Self, String> {
        if k > 0 {
            let mut poly = vec![0; k as usize];
            let mut rng = thread_rng();
            poly[0] = secret.clone();
            for i in 1..k as usize {
                poly[i] = rng.gen();
            }
            Ok(Self { poly })
        } else {
            Err("k must be a positive number".to_owned())
        }
    }

    fn evaluate(&self, x: u8) -> u8 {
        let mut b = 0;
        let range = 0..self.poly.len();
        for i in range.rev() {
            b = b * x + self.poly[i];
        }
        b
    }
}

impl Recover<u8> for Shamir {
    fn validate_recover(shares: &[(u8, u8)]) -> Result<(), String> {
        if shares.len() == 0 {
            return Err("shares must be non-empty".to_owned());
        }
        Ok(())
    }

    fn recover(shares: &[(u8, u8)]) -> Result<u8, String> {
        Self::validate_recover(shares)?;
        let mut recovered = 0;
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
        Ok(recovered)
    }
}

impl Share<u8> for Shamir {
    fn validate_share(_: &u8, n: u8, k: u8) -> Result<(), String> {
        if k == 0 {
            return Err("k must be a positive number".to_owned());
        }
        if k > n {
            return Err("k must be less than or equal to n".to_owned());
        }
        Ok(())
    }

    fn share(secret: &u8, n: u8, k: u8) -> Result<Vec<(u8, u8)>, String> {
        Self::validate_share(secret, n, k)?;
        let shamir = Self::from(secret, k)?;
        let mut shares = vec![(0, 0); n as usize];
        for i in 0..n as usize {
            let x = (i + 1) as u8;
            let y = shamir.evaluate(x);
            shares[i].0 = x;
            shares[i].1 = y;
        }
        Ok(shares)
    }
}

impl Recover<Vec<u8>> for Shamir {
    fn validate_recover(shares: &[(u8, Vec<u8>)]) -> Result<(), String> {
        if shares.len() == 0 {
            return Err("shares must be non-empty".to_owned());
        }
        let len = shares[0].1.len();
        for (_, evaluated) in shares {
            if evaluated.len() != len {
                return Err("all shares must be the same len".to_owned());
            }
        }
        Ok(())
    }

    fn recover(shares: &[(u8, Vec<u8>)]) -> Result<Vec<u8>, String> {
        Self::validate_recover(shares)?;
        let len = shares[0].1.len();
        let mut recovered = vec![0; len];
        for i in 0..len {
            let mut temp = vec![(0, 0); shares.len()];
            for j in 0..shares.len() {
                temp[j].0 = shares[j].0;
                temp[j].1 = shares[j].1[i];
            }
            recovered[i] = Self::recover(&temp)?;
        }
        Ok(recovered)
    }
}

impl Share<Vec<u8>> for Shamir {
    fn validate_share(secret: &Vec<u8>, n: u8, k: u8) -> Result<(), String> {
        if secret.len() == 0 {
            return Err("secret must be non-empty".to_owned());
        }
        if k == 0 {
            return Err("k must be a positive number".to_owned());
        }
        if k > n {
            return Err("k must be less than or equal to n".to_owned());
        }
        Ok(())
    }

    fn share(secret: &Vec<u8>, n: u8, k: u8) -> Result<Vec<(u8, Vec<u8>)>, String> {
        Self::validate_share(secret, n, k)?;
        let mut shares = vec![(0, vec![0; secret.len()]); n as usize];
        for i in 0..secret.len() {
            let temp = Self::share(&secret[i], n, k)?;
            for j in 0..temp.len() {
                shares[j].0 = temp[j].0;
                shares[j].1[i] = temp[j].1;
            }
        }
        Ok(shares)
    }
}
