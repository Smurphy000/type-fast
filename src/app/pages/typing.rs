use rand::{distributions::Uniform, prelude::Distribution};
use ratatui::{
    style::Stylize,
    text::{Line, Span, Text},
};

use serde::{Deserialize, Serialize};
use std::{
    fs,
    sync::OnceLock,
    time::{Duration, Instant},
};

static LANGUAGE: OnceLock<LanguagePrompt> = OnceLock::new();

#[derive(Debug, PartialEq, Clone, Copy)]
enum LetterState {
    Unpressed,
    Incorrect,
    Correct,
}

#[derive(Debug, Clone, Copy)]
struct TypingLetter {
    state: LetterState,
    value: char,
    position: usize,
}

impl TypingLetter {
    pub fn new(c: char, state: LetterState, position: usize) -> Self {
        Self {
            state,
            value: c,
            position,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct LanguagePrompt {
    name: String,
    #[serde(rename = "noLazyMode")]
    no_lazy_mode: bool,
    #[serde(rename = "orderedByFrequency")]
    ordered_by_frequency: bool,
    words: Vec<String>,
}

impl LanguagePrompt {
    // this object will be created via serde
    // once the words are available the caller will
    // want to generate a random subset to call the prompt
    // we will convert the strings into a sequence of characters
    // TODO
    pub fn generate(&self, word_count: u32) -> Vec<char> {
        let mut rng = rand::thread_rng();
        let uni = Uniform::from(0..self.words.len());
        // let mut words = vec![];
        let mut chars = vec![];

        for _ in 0..word_count {
            let idx = uni.sample(&mut rng);
            let word = self.words[idx].clone();

            let _: Vec<_> = word.chars().map(|c| chars.push(c)).collect();
            chars.push('•');
        }

        // for each word
        // push each char
        // push space at end of word
        // except last word
        if chars.last().unwrap() == &'•' {
            chars.pop();
        }
        chars
    }
}

#[derive(Debug)]
pub struct Typing<'a> {
    // position in the phrase
    position: usize,
    // current sequence of typed characters by the user
    pub typing: Vec<char>,
    // phrase the user is attempting to type
    phrase: Vec<char>,
    // state of each typed character position, used for rendering logic
    state: Vec<TypingLetter>, // this might need to be a custom type
    // materialized text for the UI to display, computed on any new input
    pub text: Text<'a>,

    language: &'a LanguagePrompt,

    start_time: Instant,
    duration: Duration,
}

impl<'a> Typing<'a> {
    pub fn new() -> Self {
        let l = LANGUAGE.get_or_init(|| {
            let data =
                fs::read_to_string("./src/language/english_10k.json").expect("issue reading file");

            serde_json::from_str(&data).expect("JSON format error")
        });

        let phrase = l.generate(10);
        let state: Vec<TypingLetter> = Self::setup_state(&phrase);

        let text = Self::setup_text(state.clone());
        Self {
            position: 0,
            typing: vec![],
            phrase: phrase,
            state: state,
            text: text,
            language: l,
            start_time: Instant::now(),
            duration: Duration::default(),
        }
    }

    // reset fields with current prompt
    pub fn reset(&mut self) {
        self.position = 0;
        self.typing = vec![];
        self.state = Self::setup_state(&self.phrase);
        self.text = Self::setup_text(self.state.clone());
        self.start_time = Instant::now()
    }

    //
    fn setup_state(phrase: &Vec<char>) -> Vec<TypingLetter> {
        phrase
            .iter()
            .enumerate()
            .map(|(i, x)| TypingLetter {
                state: LetterState::Unpressed,
                value: *x,
                position: i,
            })
            .collect()
    }

    fn setup_text(state: Vec<TypingLetter>) -> Text<'a> {
        let spans: Vec<Span> = state
            .into_iter()
            .enumerate()
            .map(|(i, x)| {
                if i == 0 {
                    Span::raw(x.value.to_string()).white().underlined()
                } else {
                    Span::raw(x.value.to_string()).white()
                }
            })
            .collect();
        Text::from(Line::from(spans))
    }

    // take in the current user input
    pub fn input(&mut self, c: char) -> bool {
        // overwrite start_time if typing is empty
        if self.typing.is_empty() {
            self.start_time = Instant::now();
        }
        self.typing.push(c);

        self.character_matching(c);

        if self.position >= self.phrase.len() {
            self.duration = self.start_time.elapsed();
            return true;
        }
        false
    }

    pub fn character_matching(&mut self, c: char) {
        let current_char = self.phrase[self.position];

        if c == current_char {
            if self.state[self.position].state != LetterState::Incorrect {
                let _ = std::mem::replace(
                    &mut self.state[self.position],
                    TypingLetter::new(c, LetterState::Correct, self.position),
                );
            }

            self.position += 1;
        } else {
            let _ = std::mem::replace(
                &mut self.state[self.position],
                TypingLetter::new(current_char, LetterState::Incorrect, self.position),
            );
        }
    }

    pub fn construct_text(&mut self) {
        let mut spans = vec![];

        // TODO would prefer not to clone here
        for i in self.state.clone().into_iter() {
            match i.state {
                LetterState::Unpressed => spans.push(Span::raw(i.value.to_string()).white()),
                LetterState::Incorrect => {
                    spans.push(Span::raw(i.value.to_string()).red());
                }
                LetterState::Correct => {
                    spans.push(Span::raw(i.value.to_string()).dark_gray());
                }
            }
        }

        // underline the current position, might be better to mem replace
        if self.position < self.phrase.len() {
            let current = spans[self.position].clone();
            spans[self.position] = current.underlined();
        }

        let text = Text::from(Line::from(spans));
        self.text = text;
    }

    pub fn calculate_statistics(&self) -> TypingStats {
        let wpm = (self.typing.len() as f32 / 5.0) / (self.duration.as_secs_f32() / 60 as f32);
        let acc = self.phrase.len() as f32 / self.typing.len() as f32;
        TypingStats {
            wpm: wpm,
            accuracy: acc * 100 as f32,
            awpm: wpm * acc,
        }
    }
}

#[derive(Debug)]
pub struct TypingStats {
    pub wpm: f32,
    pub accuracy: f32,
    pub awpm: f32,
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_insert_when_first_of_phrase_is_correct() {
        let mut t = Typing::new();

        t.input('t');

        println!("{:?}", t);
    }

    #[test]
    fn test_insert_when_first_of_phrase_is_incorrect() {
        let mut t = Typing::new();

        t.input('t');
        t.input('c');
        t.input('e');

        println!("{:?}", t);
        t.construct_text();

        println!("{:?}", t);
    }
}
