use std::collections::HashMap;

pub struct Poe;

pub trait Cipher {
    fn encrypt(data: &str) -> String;
    fn decrypt(data: &str) -> String;
}

impl Cipher for Poe {
    fn encrypt(data: &str) -> String {
        let hm: HashMap<char, char> = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .into_iter()
            .zip("52-†81346,709*‡.$();?¶]¢:[".chars().into_iter())
            .collect();
        data.to_lowercase()
            .chars()
            .map(|x| hm.get(&x).unwrap_or(&' '))
            .collect()
    }
    fn decrypt(data: &str) -> String {
        let hm: HashMap<char, char> = "52-†81346,709*‡.$();?¶]¢:["
            .chars()
            .into_iter()
            .zip("abcdefghijklmnopqrstuvwxyz".chars().into_iter())
            .collect();
        data.to_lowercase()
            .chars()
            .map(|x| hm.get(&x).unwrap_or(&' '))
            .collect()
    }
}
