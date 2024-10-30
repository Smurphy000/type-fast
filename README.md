# Type Fast

Terminal typing test to help you improve on typing speed and accuracy and runs in the terminal!

Meant to be lightweight, fast, and customizable.

## Keybindings

| Key     | Action                | Location      |
| ------- | --------------------- | ------------- |
| j       | ↑                     | Menu Nav      |
| k       | ↓                     | Menu Nav      |
| g       | Select First          | Menu Nav      |
| G       | Select Last           | Menu Nav      |
| h       | Select None           | Menu Nav      |
| ENTER   | Select Current        | Menu Nav      |
| ALT + 1 | Inc word count        | Typing prompt |
| ALT + 2 | Toggle capitalization | Typing prompt |
| ALT + 3 | Toggle punctuation    | Typing prompt |
| ALT + 4 | Toggle Zen mode       | Typing prompt |
| →       | Skip current prompt   | Typing prompt |
| ←       | Reset current prompt  | Typing prompt |
| ESC     | Pause                 | Typing prompt |
| ESC     | Resume                | Pause Screen  |
| q       | Return to menu        | Pause Screen  |

## Remaining Work

- [ ] cli option support
- [ ] configuration file support
- [ ] implement capitalization and puctuation into prompt generation
- [ ] statistics tracking
  - [ ] visualization for wpm over time

## Disclaimer

Current limitations include:

- only supports English
- prompt are randomly generated with a uniform distribution from the word bank

## Setup and Installation

Clone this repo

`cargo run`

or

`cargo install --path .`

or

`cargo install --git <url>`

## Usage

`type-fast` will run the program.

## Credits

Word bank comes from MonkeyType.

## Contribution

Feel free to contribute
