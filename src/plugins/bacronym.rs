// given a word, generate an acronym for it.
// e.g. .bacronym foo returns "football opulent owl"

use super::Plugin;
use std::collections::HashMap;
use regex::{Regex, Captures};
use rand::{Rng, seq::SliceRandom};
use memorable_wordlist;
use matrix_sdk::{
    Client, RoomState, async_trait
};

pub struct BacronymPlugin {
    words: HashMap<char, Vec<String>>,
    pattern: Regex
}

impl BacronymPlugin {
    pub fn new() -> Self {
        Self {
            words: build_word_map(),
            pattern: Regex::new(r"^\.(bacronym|b)\s*(?P<acronym>[a-zA-Z ]{1,50})?$").unwrap()
        }
    }
}

#[async_trait]
impl Plugin for BacronymPlugin {
    async fn room_message(&self, client: &Client, room: &RoomState, msg_body: &str) {
        if let Some(captures) = self.pattern.captures(msg_body) {
            let acronym = get_regex_capture(captures, "acronym").unwrap_or_else(random_letter);
            let response = bacronym(&self.words, &acronym.to_lowercase());
            self.send_message(client, room, &response).await;
        }
    }
}

fn get_regex_capture(captures: Captures, name: &str) -> Option<String> {
    captures.name(name).map(|arg_val| arg_val.as_str().to_string())
}

fn random_letter() -> String {
    char::from(rand::thread_rng().gen_range(b'a'..b'z')).to_string()
}

fn bacronym(prefix_list: &HashMap<char, Vec<String>>, acronym: &str) -> String {
    acronym
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|initial| randomly_choose_word_with_initial(prefix_list, initial))
        .collect::<Option<Vec<String>>>()
        .map(|words| words.join(" "))
        .unwrap_or("Could not build a bacronym :(".to_owned())
}

// take a list of words and turn it into a hashmap of "initial character" -> "vec of words that start with that character"
fn build_word_map() -> HashMap<char, Vec<String>> {
    memorable_wordlist::WORDS
        .into_iter()
        .fold(HashMap::new(), |mut acc, word| {
            match word.chars().nth(0) {
                None => acc,
                Some(initial) => {
                    let words = acc.get_mut(&initial);
                    if let Some(wordvec) = words {
                        wordvec.push(word.to_string());
                    } else {
                        let wordvec = vec![word.to_string()];
                        acc.insert(initial, wordvec);
                    }
                    acc
                }
            }
        })
}

fn randomly_choose_word_with_initial(word_map: &HashMap<char, Vec<String>>, initial: char) -> Option<String> {
    let words_with_initial = word_map.get(&initial)?;
    words_with_initial
        .choose(&mut rand::thread_rng())
        .map(|s| s.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_arg_finds_acronym_arg() {
        let plugin = BacronymPlugin::new();

        assert_eq!(plugin.pattern.captures(".bacronym").is_some(), true);
        assert_eq!(plugin.pattern.captures(".b").is_some(), true);
        assert_eq!(plugin.pattern.captures(".b ").is_some(), true);
        assert_eq!(get_regex_capture(plugin.pattern.captures(".bacronym foobar").unwrap(), "acronym"), Some("foobar".to_owned()));
        assert_eq!(get_regex_capture(plugin.pattern.captures(".b foobar").unwrap(), "acronym"), Some("foobar".to_owned()));
    }

    #[test]
    fn bacronym_creates_bacronym() {
        let plugin = BacronymPlugin::new();
        assert_eq!(bacronym(&plugin.words, "hello").split_whitespace().count(), 5);
    }

    #[test]
    fn randomly_letter_selects_letter() {
        let letter = random_letter();
        let chars = letter.chars().collect::<Vec<char>>();
        assert_eq!(chars.len() == 1, true);
        assert_eq!(chars[0].is_alphabetic(), true);
    }

    #[test]
    fn randomly_choose_word_with_initial_chooses_word() {
        let mut word_map : HashMap<char, Vec<String>> = HashMap::new();
        word_map.insert('a', vec!["apple".to_owned()]);

        assert_eq!(randomly_choose_word_with_initial(&word_map, 'a'), Some("apple".to_owned()));
        assert_eq!(randomly_choose_word_with_initial(&word_map, 'z'), None);
    }

    #[test]
    fn build_prefix_list_has_data() {
        let word_map = build_word_map();
        assert_eq!(word_map.get(&'a').unwrap().len() > 0, true);
    }
}