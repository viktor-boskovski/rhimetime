use crate::LANGUAGE_CODES_DESCRIPTION_MAP;

pub fn download(languages: String) {
    let langs: Vec<String> = 
        languages.split(',')
        .map(|lang| lang.to_string())
        .collect();
    for lang in &langs {
        if !LANGUAGE_CODES_DESCRIPTION_MAP.map(|(code, _)| code)
            .contains(&lang.as_str()) {
                println!("Language not found!") // TODO: show what languages are supported
            }
    }
    initiate_download(&langs);
}

fn initiate_download(languages: &Vec<String>) {
    // TODO: check if folder exists. -> create if necessary
    // download from repo
}

