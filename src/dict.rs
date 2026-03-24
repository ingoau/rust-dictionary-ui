use serde::Deserialize;
use std::env;

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
struct Definition {
    word: String,
    phonetics: Vec<Phonetic>,
    #[serde(rename = "sourceUrls")]
    source_urls: Vec<String>,
    meanings: Vec<Meaning>,
}

pub fn main() {
    let client = reqwest::blocking::Client::new();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let word = &args[1];

        let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}");

        let definition_response: Vec<Definition> = client
            .get(url)
            .send()
            .expect("Failed to fetch definition")
            .json()
            .expect("Failed to parse response");

        for (i, word) in definition_response.iter().enumerate() {
            println!("{}. {}", i + 1, word.word);

            for (_, phonetic) in word.phonetics.iter().enumerate() {
                if let Some(text) = &phonetic.text {
                    println!("     {}", text)
                }
            }

            for (_, meaning) in word.meanings.iter().enumerate() {
                println!("   {}", meaning.part_of_speech);
                for (i, def) in meaning.definitions.iter().enumerate() {
                    println!("     {}. {}", i + 1, def.definition);
                    if let Some(example) = &def.example {
                        println!("        \"{}\"", example)
                    }
                }
            }

            for (_, source) in word.source_urls.iter().enumerate() {
                println!("\n   {}\n\n", source)
            }
        }
    } else {
        println!("Input a word")
    }
}
