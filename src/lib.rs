use std::collections::HashMap;

pub struct Poe;

pub trait Cipher {
    fn encrypt(data: &str) -> String;
    fn decrypt(data: &str) -> String;
    fn detect(data: &str) -> String;
}

impl Cipher for Poe {
    fn encrypt(data: &str) -> String {
        let hm: HashMap<char, char> = "abcdefghijklmnopqrstuvwxyz "
            .chars()
            .into_iter()
            .zip("52-†81346,709*‡.$();?¶]¢:[ ".chars().into_iter())
            .collect();
        data.to_lowercase()
            .chars()
            .map(|x| hm.get(&x).unwrap_or(&'▣'))
            .collect()
    }
    fn decrypt(data: &str) -> String {
        let hm: HashMap<char, char> = "52-†81346,709*‡.$();?¶]¢:[ "
            .chars()
            .into_iter()
            .zip("abcdefghijklmnopqrstuvwxyz ".chars().into_iter())
            .collect();
        data.to_lowercase()
            .chars()
            .map(|x| hm.get(&x).unwrap_or(&'▣'))
            .collect()
    }
    fn detect(data: &str) -> String {
        let mut counter_enc = 0;
        let dec_l = "52-†81346,709*‡.$();?¶]¢:[".chars().collect::<Vec<char>>();
        let mut counter_dec = 0;
        data.chars().for_each(|x| {
            if dec_l.contains(&x) {
                counter_dec += 1
            } else {
                counter_enc += 1
            }
        });
        if counter_dec > counter_enc {
            return Poe::decrypt(data);
        }
        return Poe::encrypt(data);
    }
}
