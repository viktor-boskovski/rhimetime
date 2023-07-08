mod downloader;
mod wordloading;
mod scoring;

const USAGE: &str = include_str!("txt/usage.txt");

pub const LANGUAGE_CODES_DESCRIPTION_MAP: [(&str, &str); 30] = [
("ar","Arabic (Modern Standard)"),
("de","German"),
("en_UK","English (Received Pronunciation)"),
("en_US","English (General American)"),
("eo","Esperanto"),
("es_ES","Spanish (Spain)"),
("es_MX","Spanish (Mexico)"),
("fa","Persian"),
("fi","Finnish"),
("fr_FR","French (France)"),
("fr_QC","French (Québec)"),
("is","Icelandic"),
("ja","Japanese"),
("jam","Jamaican Creole"),
("km","Khmer"),
("ko","Korean"),
("ma","Malay (Malaysian and Indonesian)"),
("nb","Norwegian Bokmål"),
("nl","Dutch"),
("or","Odia"),
("ro","Romanian"),
("sv","Swedish"),
("sw","Swahili"),
("tts","Isan"),
("vi_C","Vietnamese (Central)"),
("vi_N","Vietnamese (Northern)"),
("vi_S","Vietnamese (Southern)"),
("yue","Cantonese"),
("zh_hans","Mandarin"),
("zh_hant","Mandarin"),];



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("{USAGE}")
    }

    if args.len() == 2 {
        // TODO: display help for usage depending on arg
    }

    if args.len() == 3 {
        if args[1] == "download" {
            if args[2] == "list" {
                println!("Code:\t\tDescription:\n");
                for (code, description) in LANGUAGE_CODES_DESCRIPTION_MAP {
                    println!("{code}\t\t{description}")
                }
            }
            else {
                downloader::download(args[2].to_string()).await?
            }
        }
    }

    Ok(())
}
