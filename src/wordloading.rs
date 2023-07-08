use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, ErrorKind, Error};

fn load_words_ipa(path: &str) -> io::Result<HashMap<String, Vec<String>>> {
    let file = File::open(path)?;
    let mut result = HashMap::new();
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let mut spliterator = line.split("\t");
        let word = spliterator.next()
            .ok_or(Error::new(ErrorKind::InvalidData, "missing word"))?;
        let ipas = spliterator.next()
            .ok_or(Error::new(ErrorKind::InvalidData, "missing ipa"))?;

        let ipas = ipas.split("/")
            .skip(1)
            .step_by(2)
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        result.insert(word.to_string(), ipas);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_words_ipa() -> io::Result<()> {
        let res = load_words_ipa("./langs/de.txt")?;
        assert_eq!(res.get("zytogen").unwrap(), &vec!["t͡sytoˈɡeːn".to_string()]);

        Ok(())
    }
}

