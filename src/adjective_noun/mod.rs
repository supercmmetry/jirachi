use rand;
use rand::Rng;
use anyhow;
use crate::Wishable;

pub struct Jirachi {
    adjectives: Vec<String>,
    nouns: Vec<String>
}

impl Jirachi {
    pub fn new() -> anyhow::Result<Self> {
        let raw_adjectives = include_bytes!("../../res/adjective_noun/adjectives.txt");
        let raw_nouns = include_bytes!("../../res/adjective_noun/nouns.txt");

        let mut tmp_str = String::new();

        let mut adjectives = vec![];
        let mut nouns = vec![];

        for byte in raw_adjectives.iter() {
            if byte.clone() == '\n' as u8 || byte.clone() == '\r' as u8 {
                if tmp_str.len() > 0 {
                    adjectives.push(tmp_str.clone());
                    tmp_str = String::new();
                }
            } else {
                tmp_str.push(byte.clone() as char);
            }
        }

        if tmp_str.len() > 0 {
            adjectives.push(tmp_str.clone());
        }

        tmp_str = String::new();

        for byte in raw_nouns.iter() {
            if byte.clone() == '\n' as u8 || byte.clone() == '\r' as u8 {
                if tmp_str.len() > 0 {
                    nouns.push(tmp_str.clone());
                    tmp_str = String::new();
                }
            } else {
                tmp_str.push(byte.clone() as char);
            }
        }

        if tmp_str.len() > 0 {
            nouns.push(tmp_str.clone());
        }

        if adjectives.len() == 0 || nouns.len() == 0 {
            return Err(anyhow!("Some crucial resources were not loaded."));
        }

        Ok(Self {
            adjectives,
            nouns
        })
    }

    fn get_random_noun(&self) -> String {
        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(0, self.nouns.len() - 1);

        return self.nouns[rand_index].clone()
    }

    fn get_random_adjective(&self) -> String {
        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(0, self.adjectives.len() - 1);

        return self.adjectives[rand_index].clone()
    }
}

impl Wishable for Jirachi {
    fn wish(&mut self) -> anyhow::Result<String> {
        let noun = self.get_random_noun().to_lowercase();
        let adjective = self.get_random_adjective().to_lowercase();

        Ok([adjective, "-".to_string(), noun].concat())
    }
}
