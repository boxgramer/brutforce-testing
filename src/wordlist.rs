use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use rayon::prelude::*;

pub struct Wordlist {
    pub data: Vec<(String, Vec<String>)>,
}

impl Wordlist {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn from(cliargs: Vec<(String, PathBuf)>) -> Result<Self, Box<dyn Error>> {
        let mut word_list = Wordlist::new();
        word_list.data = cliargs
            .par_iter()
            .map(|(key, filepath)| {
                let lines: Vec<String> = Self::read_file(filepath)?.collect();
                Ok((key.clone(), lines))
            })
            .collect::<Result<Vec<(String, Vec<String>)>, std::io::Error>>()?;

        Ok(word_list)
    }

    pub fn read_file<P>(path: P) -> Result<impl Iterator<Item = String>, std::io::Error>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(reader
            .lines()
            .filter_map(Result::ok)
            .filter(|l| !l.trim().is_empty()))
    }
}
