use std::error::Error;

use crate::LANGUAGE_CODES_DESCRIPTION_MAP;




const LANG_PATH: &str = "./langs/";
const LANG_REPO_URL: &'static str = "https://raw.githubusercontent.com/open-dict-data/ipa-dict/master/data/";

pub async fn download(languages: String) -> Result<(), Box<dyn Error>> {
    let langs = split_languages(languages);
    create_languages_dir()?;
    for i in langs {
        let text = reqwest::get(LANG_REPO_URL.to_string() + &i + ".txt")
            .await?
            .text()
            .await?;
        std::fs::write(LANG_PATH.to_string() + &i + ".txt", text)?;
    }
    Ok(())
}


fn split_languages(languages: String) -> Vec<String> {
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
    langs
}



fn create_languages_dir() -> std::io::Result<()> {
    if let Err(_) = std::fs::metadata(&LANG_PATH) {
        std::fs::create_dir(LANG_PATH)?;
    }
    Ok(())
}
