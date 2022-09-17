#![allow(dead_code)]

use dioxus::html::input_data::keyboard_types::Key;
use rand::seq::SliceRandom;
use std::{str::FromStr, vec::Vec};

const LEFT_QWERTY_KEYS: &str = "QWERT ASDFG ZXCVB";
const RIGHT_QWERTY_KEYS: &str = "YUIOP HJKL\' NM,./";

#[derive(PartialEq)]
pub(crate) struct KeyState {
    key: Key,
    enabled: bool,
}
impl KeyState {
    pub(crate) fn new(key: &Key, enabled: bool) -> Self {
        KeyState {
            key: key.clone(),
            enabled,
        }
    }

    pub(crate) fn key(&self) -> &Key {
        &self.key
    }

    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }
}

pub(crate) struct KeyboardState {
    keys: Vec<Vec<KeyState>>,
}
impl KeyboardState {
    pub(crate) fn new(dictionary: &WordDictionary) -> Self {
        let keys = dictionary
            .keys()
            .split_whitespace()
            .map(|row| {
                row.chars()
                    .map(|key| KeyState {
                        key: Key::from_str(&key.to_string()).expect("Non-existent key supplied"),
                        enabled: false,
                    })
                    .collect()
            })
            .collect();

        KeyboardState { keys }
    }

    pub(crate) fn update_for(&mut self, key: &KeyState) {
        self.keys.iter_mut().for_each(|row| {
            if let Some(key_state) = row.iter_mut().find(|key_state| {
                // FIXME: slow, ugly, stupid
                key_state.key.to_string().to_uppercase() == key.key.to_string().to_uppercase()
            }) {
                key_state.enabled = key.enabled
            }
        });
    }

    pub(crate) fn keys(&self) -> &Vec<Vec<KeyState>> {
        self.keys.as_ref()
    }
}

pub(crate) struct WordDictionary<'a> {
    buffer: Vec<&'a str>,
    keys: String,
}
impl<'a> WordDictionary<'a> {
    pub(crate) fn keys(&self) -> &str {
        self.keys.as_ref()
    }
}

pub(crate) struct AppDictionaries<'a> {
    pub(crate) left: WordDictionary<'a>,
    pub(crate) right: WordDictionary<'a>,
}
impl Default for AppDictionaries<'_> {
    fn default() -> Self {
        Self {
            left: WordDictionary {
                buffer: vec!["QWERT", "ASDFG", "ZXCVB", "FWWET"],
                keys: LEFT_QWERTY_KEYS.to_owned(),
            },
            right: WordDictionary {
                buffer: vec!["YUIOP", "HJKL:", "BNM<>", ",./'"],
                keys: RIGHT_QWERTY_KEYS.to_owned(),
            },
        }
    }
}

#[derive(Default, Clone)]
pub(crate) struct WordData {
    input: String,
    last_word: String,
    buffer: Vec<String>,
    keys: Vec<Vec<String>>,
}

impl WordData {
    /// Copies `amount` of elements from provided `dictionary` and constructs [WordBuffer] from them,
    pub(crate) fn new(amount: usize, dictionary: &WordDictionary) -> Self {
        let mut rng = rand::thread_rng();

        let buffer = dictionary
            .buffer
            .choose_multiple(&mut rng, amount)
            .map(|str| str.to_string())
            .collect::<Vec<String>>();

        WordData {
            buffer,
            ..Default::default()
        }
    }

    pub(crate) fn submit(&mut self) {
        self.last_word = self.input.clone();
        if !self.buffer.is_empty() {
            self.buffer.remove(0);
        }
        self.input.clear();
    }

    pub(crate) fn last_word(&self) -> &str {
        self.last_word.as_ref()
    }

    pub(crate) fn next_word(&self) -> Option<&str> {
        match self.buffer.get(0) {
            Some(word) => Some(word.as_str()),
            None => None,
        }
    }

    pub(crate) fn push_str(&mut self, string: &str) {
        self.input.push_str(string)
    }

    pub(crate) fn push(&mut self, ch: char) {
        self.input.push(ch)
    }

    pub(crate) fn input(&self) -> &str {
        self.input.as_ref()
    }

    pub(crate) fn pop(&mut self) -> Option<char> {
        self.input.pop()
    }

    pub(crate) fn buffer(&self) -> &Vec<String> {
        self.buffer.as_ref()
    }

    pub(crate) fn drain(&mut self) {
        self.buffer.drain(..);
    }

    pub(crate) fn keys(&self) -> &Vec<Vec<String>> {
        self.keys.as_ref()
    }
}
