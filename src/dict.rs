use serde::Deserialize;

#[derive(Deserialize)]
struct MeaningDefinition {
    definition: String,
    example: Option<String>,
}

#[derive(Deserialize)]
struct Meaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<MeaningDefinition>,
}

#[derive(Deserialize)]
struct Phonetic {
    text: Option<String>,
}

#[derive(Deserialize)]
pub struct Definition {
    pub word: String,
    phonetics: Vec<Phonetic>,
    #[serde(rename = "sourceUrls")]
    source_urls: Vec<String>,
    meanings: Vec<Meaning>,
}

pub fn get_defenition(word: &str) -> Vec<Definition> {
    let client = reqwest::blocking::Client::new();

    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}");

    let definition_response: Vec<Definition> = client
        .get(url)
        .send()
        .expect("Failed to fetch definition")
        .json()
        .expect("Failed to parse response");

    definition_response
}
