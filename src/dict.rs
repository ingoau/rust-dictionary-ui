use serde::Deserialize;

#[derive(Deserialize)]
pub struct MeaningDefinition {
    pub definition: String,
    pub example: Option<String>,
}

#[derive(Deserialize)]
pub struct Meaning {
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    pub definitions: Vec<MeaningDefinition>,
}

#[derive(Deserialize)]
pub struct Phonetic {
    pub text: Option<String>,
}

#[derive(Deserialize)]
pub struct Definition {
    pub word: String,
    pub phonetics: Vec<Phonetic>,
    #[serde(rename = "sourceUrls")]
    pub source_urls: Vec<String>,
    pub meanings: Vec<Meaning>,
}

pub fn get_definition(word: &str) -> Vec<Definition> {
    let client = reqwest::blocking::Client::new();

    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}");

    let definition_response: Vec<Definition> = client
        .get(url)
        .send()
        .expect("Failed to fetch definition")
        .json()
        .unwrap_or_else(|_| Vec::new());

    definition_response
}
