const LANGUAGE_CODES: [&str; 29] = [
    "ar", "de", "en_UK", "en_US", "eo", "es_ES", "es_MX", "fa", "fi", "fr_FR", "fr_QC", "is", "ja",
    "jam", "km", "ko", "ma", "nb", "nl", "or", "ro", "sv", "sw", "tts", "vi_C", "vi_N", "vi_S",
    "yue", "zh",
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 && args[1] == "download" && args[2] == "list" {
        println!(
            "
ar	Arabic (Modern Standard)
de	German
en_UK	English (Received Pronunciation)
en_US	English (General American)
eo	Esperanto
es_ES	Spanish (Spain)
es_MX	Spanish (Mexico)
fa	Persian
fi	Finnish
fr_FR	French (France)
fr_QC	French (Québec)
is	Icelandic
ja	Japanese
jam	Jamaican Creole
km	Khmer
ko	Korean
ma	Malay (Malaysian and Indonesian)
nb	Norwegian Bokmål
nl	Dutch
or	Odia
ro	Romanian
sv	Swedish
sw	Swahili
tts	Isan
vi_C	Vietnamese (Central)
vi_N	Vietnamese (Northern)
vi_S	Vietnamese (Southern)
yue	Cantonese
zh	Mandarin"
        )
    }

    if args[1] == "download" {
        initiate_download(args[2].to_string());
    }
}

fn initiate_download(languages: String) {
    // TODO: do this
}
