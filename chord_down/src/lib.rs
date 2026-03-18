use regex::Regex;
use ron;
use ron::value::Value;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::LazyLock};

#[derive(Deserialize, Serialize, Debug)]
pub enum LineElement {
    Text(String),
    Chord(String),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Line {
    pub elements: Vec<LineElement>,
}
impl Line {
    pub fn from(string: &str) -> Self {
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.*)\](.*)").unwrap());
        let mut elements: Vec<LineElement> = vec![];
        for part in string.split('[') {
            if part == "" {
                continue;
            }
            if let Some(captures) = RE.captures(part) {
                elements.push(LineElement::Chord(captures[1].to_string()));
                elements.push(LineElement::Text(captures[2].to_string()));
            } else {
                elements.push(LineElement::Text(part.to_string()));
            }
        }
        Line { elements }
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SongPart {
    pub name: String, // Chorus, Bridge, Interlude, Outro ...
    pub lines: Vec<Line>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Block {
    Part(SongPart),
    Tabulatur(Vec<String>),
    Grid(Vec<String>),
    Unknown(Vec<String>),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Document {
    pub blocks: Vec<Block>,
}

#[derive(Deserialize, Serialize)]
pub struct Song {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub manual_meta: HashMap<String, String>,
    pub document: Document,
}

#[derive(Debug)]
struct ParsingState {
    in_a_block: bool,
    in_tabulatur: bool,
    in_grid: bool,
    last_part_name: Option<String>,
    last_line_blank: bool,
}

impl ParsingState {
    pub fn new() -> Self {
        ParsingState {
            in_a_block: false,
            in_tabulatur: false,
            in_grid: false,
            last_part_name: None,
            last_line_blank: false,
        }
    }
}
fn should_start_tabulatur(line: &str) -> bool {
    match line {
        "{sot}" | "{start_of_tab}" => true,
        _ => false,
    }
}

fn should_start_grid(line: &str) -> bool {
    match line {
        "{sog}" | "{start_of_grid}" => true,
        _ => false,
    }
}

fn should_set_meta_key_value(line: &str) -> Option<(String, String)> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\s*\{\s*(\w+)\s*:\s*(.*?)\s*\}\s*$").unwrap());
    if let Some(captures) = RE.captures(line) {
        return Some((captures[1].to_string(), captures[2].to_string()));
    }
    return None;
}

fn should_set_part_name(line: &str) -> Option<String> {
    let line = line.trim();
    if line == "{soc}" {
        return Some(String::from("Chorus"));
    } else if line == "{sov}" {
        return Some(String::from("Verse"));
    } else if line == "{sob}" {
        return Some(String::from("Bridge"));
    }

    // Outros, Intros, everything written as
    // [Blockname]
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\[(\w{5,})\]*$").unwrap());
    if let Some(captures) = RE.captures(line) {
        return Some(captures[1].to_string());
    };
    None
}
impl Song {
    fn empty() -> Self {
        Song {
            title: None,
            artist: None,
            manual_meta: HashMap::new(),
            document: Document { blocks: vec![] },
        }
    }

    fn parse_in_tabulatur(&mut self, line: &str, state: &mut ParsingState) -> () {
        if (line == "{eot}") || (line == "{end_of_tab}") {
            state.in_tabulatur = false;
            state.last_line_blank = false;

            return;
        };
        let Some(last_block) = self.document.blocks.last_mut() else {
            return;
        };
        match last_block {
            Block::Tabulatur(lines) => lines.push(line.to_string()),
            _ => (),
        }
        return;
    }

    fn parse_in_grid(&mut self, line: &str, state: &mut ParsingState) -> () {
        if (line == "{eog}") || (line == "{end_of_grid}") {
            state.in_grid = false;
            state.last_line_blank = false;
            return;
        };
        let Some(last_block) = self.document.blocks.last_mut() else {
            return;
        };
        match last_block {
            Block::Grid(lines) => lines.push(line.to_string()),
            _ => (),
        }
        return;
    }
    fn start_tabulatur(&mut self, state: &mut ParsingState) -> () {
        self.document.blocks.push(Block::Tabulatur(vec![]));
        state.in_tabulatur = true;
        state.last_line_blank = false;
    }
    fn start_grid(&mut self, state: &mut ParsingState) -> () {
        self.document.blocks.push(Block::Grid(vec![]));
        state.in_grid = true;
        state.last_line_blank = false;
    }
    fn start_part(&mut self, part_name: String, state: &mut ParsingState) -> () {
        self.document.blocks.push(Block::Part(SongPart {
            name: part_name.clone(),
            lines: vec![],
        }));
        state.last_part_name = Some(part_name);
        state.in_a_block = false;
        state.last_line_blank = false;
    }

    fn start_anonymous_part(&mut self, state: &mut ParsingState) -> () {
        self.start_part(String::from("(undef)"), state)
    }

    fn set_meta_key_value(&mut self, key: String, value: String) -> () {
        match key.as_str() {
            "t" | "tit" | "title" => self.title = Some(value),
            "a" | "art" | "artist" | "st" | "subtitle" => self.artist = Some(value),
            _ => {
                self.manual_meta.insert(key, value);
                ()
            }
        };
    }

    fn finish_part(&mut self, state: &mut ParsingState) -> () {
        state.last_part_name = None;
        state.in_a_block = false;
    }

    fn add_line_to_latest_part(&mut self, line: &str, state: &mut ParsingState) -> () {
        let Some(last_block) = self.document.blocks.last_mut() else {
            return;
        };
        match last_block {
            Block::Part(song_part) => song_part.lines.push(Line::from(line)),
            _ => (),
        }
        state.in_a_block = true;
        return;
    }

    fn handle_empty_line(&mut self, state: &mut ParsingState) -> () {
        self.finish_part(state);
    }

    fn parse_line(&mut self, line: &str, state: &mut ParsingState) -> () {
        let line = line.trim();
        if state.in_tabulatur {
            return self.parse_in_tabulatur(line, state);
        } else if should_start_tabulatur(line) {
            return self.start_tabulatur(state);
        }
        if state.in_grid {
            return self.parse_in_grid(line, state);
        } else if should_start_grid(line) {
            self.start_grid(state);
        }

        if let Some((key, value)) = should_set_meta_key_value(line) {
            return self.set_meta_key_value(key, value);
        }
        if let Some(part_name) = should_set_part_name(line) {
            return self.start_part(part_name, state);
        }
        if line == "" {
            return self.handle_empty_line(state);
        }
        if !state.in_a_block {
            self.start_anonymous_part(state);
        }
        self.add_line_to_latest_part(line, state);
    }
    pub fn parse(source: String) -> Self {
        let mut song = Song::empty();
        let mut state = ParsingState::new();
        for line in source.lines() {
            song.parse_line(line, &mut state);
        }
        song
    }

    pub fn as_ron(&self) -> Value {
        Value::Unit
    }
}
