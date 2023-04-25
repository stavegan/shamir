use bip39::Mnemonic;
use rand::{thread_rng, Rng};
use shamir_util::shamir::{recover::Recover, share::Share, Shamir};

fn entropy() -> [u8; 32] {
    let mut entropy = [0u8; 32];
    let mut rng = thread_rng();
    rng.fill(&mut entropy);
    entropy
}

fn main() {
    let entropy = entropy();
    println!("Entropy: {:?}", entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
    println!("Seed phrase: {:?}", mnemonic.to_string());

    let shares = Shamir::share(&Vec::from(entropy), 5, 3).unwrap();
    println!("Shares:");
    for i in 0..shares.len() {
        let mnemonic = Mnemonic::from_entropy(&shares[i].1).unwrap();
        println!("{:?}: {:?}", shares[i].0, mnemonic.to_string());
    }

    let recovered_entropy = Shamir::recover(&shares).unwrap();
    println!("Recovered entropy: {:?}", recovered_entropy);

    let recovered_mnemonic = Mnemonic::from_entropy(&recovered_entropy).unwrap();
    println!(
        "Recovered seed phrase: {:?}",
        recovered_mnemonic.to_string()
    );
}
