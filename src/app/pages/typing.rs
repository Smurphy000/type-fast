use log::trace;
use ratatui::{
    style::Stylize,
    text::{Line, Span, Text},
};

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
}

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

impl<'a> Typing<'a> {
    pub fn new(phrase: Vec<char>) -> Self {
        let state: Vec<TypingLetter> = phrase
            .iter()
            .enumerate()
            .map(|(i, x)| TypingLetter {
                state: LetterState::Unpressed,
                value: *x,
                position: i,
            })
            .collect();

        let spans: Vec<Span> = state
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, x)| {
                if i == 0 {
                    Span::raw(x.value.to_string()).gray().underlined()
                } else {
                    Span::raw(x.value.to_string()).gray()
                }
            })
            .collect();
        let text = Text::from(Line::from(spans));
        Self {
            position: 0,
            typing: vec![],
            phrase: phrase,
            state: state,
            text: text,
        }
    }

    // take in the current user input
    // TODO, refactor, but after we determine if the data format works for the ui
    pub fn input(&mut self, c: char) -> bool {
        // this can cause a panic if we exceed the end of the phrase
        // todo add some logging
        self.typing.push(c);

        self.character_matching(c);
        trace!(target: "typing", "{:?}, {:?}", self.position, self.phrase.len());
        if self.position >= self.phrase.len() {
            // todo return something here to notify end of typing
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
                LetterState::Unpressed => spans.push(Span::raw(i.value.to_string()).gray()),
                LetterState::Incorrect => {
                    spans.push(Span::raw(i.value.to_string()).red());
                }
                LetterState::Correct => {
                    spans.push(Span::raw(i.value.to_string()).green());
                }
            }
        }

        // underline the current position, might be better to mem replace
        if self.position < self.phrase.len() {
            let current = spans[self.position].clone();
            spans[self.position] = current.underlined();
        }

        let text = Text::from(Line::from(spans));
        trace!(target: "Typing", "text: {:?}", text);
        self.text = text;
    }
}

// String for what the user should be typing
// Need to track the current character the user should type, for comparison
// Need to get the currently pressed character, and store all user inputs
// then we need to construct contiguous sections of correct, incorrect, and untyped character
// to render to the screen

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_insert_when_first_of_phrase_is_correct() {
        let mut t = Typing::new(vec!['t', 'e', 's', 't']);

        t.input('t');

        println!("{:?}", t);
    }

    #[test]
    fn test_insert_when_first_of_phrase_is_incorrect() {
        let mut t = Typing::new(vec!['t', 'e', 's', 't']);

        t.input('t');
        t.input('c');
        t.input('e');

        println!("{:?}", t);
        t.construct_text();

        println!("{:?}", t);
    }
}
