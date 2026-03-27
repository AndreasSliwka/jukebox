use regex::Regex;
use ron;
use serde::{Deserialize, Serialize};
use std::{any::type_name_of_val, collections::HashMap, sync::LazyLock};

#[derive(Deserialize, Serialize, Debug)]
pub struct LineElement {
    pub lyrics: Option<String>,
    pub chord: Option<String>,
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
            let mut lyrics: Option<String> = None;
            let mut chord: Option<String> = None;
            if let Some(captures) = RE.captures(part) {
                let (maybe_chord, maybe_lyrics) =
                    (captures[1].to_string(), captures[2].to_string());
                if maybe_chord != "" {
                    chord = Some(maybe_chord)
                };
                if maybe_lyrics != "" {
                    lyrics = Some(maybe_lyrics)
                }
            } else {
                lyrics = Some(part.to_string());
            }
            elements.push(LineElement { lyrics, chord });
        }
        Line { elements }
    }
    pub fn has_lyrics_has_chords(&self) -> (bool, bool) {
        let mut has_lyrics = false;
        let mut has_chords = false;
        for element in self.elements.iter() {
            match element.lyrics {
                Some(_) => has_lyrics = true,
                _ => (),
            }
            match element.chord {
                Some(_) => has_chords = true,
                _ => (),
            }
        }
        (has_lyrics, has_chords)
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub tags: Vec<String>,
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
    title_found: bool,
    artist_found: bool,
    verbose: bool,
}

impl ParsingState {
    pub fn new(verbose: bool) -> Self {
        ParsingState {
            in_a_block: false,
            in_tabulatur: false,
            in_grid: false,
            last_part_name: None,
            last_line_blank: false,
            title_found: false,
            artist_found: false,
            verbose,
        }
    }
    pub fn verbose(&self, message: String) {
        if self.verbose {
            println!("{}", message);
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
    if line == "{soi}" {
        return Some(String::from("Intro"));
    } else if line == "{soc}" {
        return Some(String::from("Chorus"));
    } else if line == "{sov}" {
        return Some(String::from("Verse"));
    } else if line == "{sob}" {
        return Some(String::from("Bridge"));
    } else if line == "{soo}" {
        return Some(String::from("Outro"));
    }

    // Outros, Intros, everything written as
    // [Blockname]
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\{comment:\s*(\w+)\}\s*$").unwrap());
    if let Some(captures) = RE.captures(line) {
        return Some(captures[1].to_string());
    };
    None
}

fn should_finish_current_path(line: &str, maybe_last_part: &Option<String>) -> bool {
    let Some(last_path) = maybe_last_part else {
        return false;
    };
    (last_path == "Intro" && line == "{eoi}")
        || (last_path == "Verse" && line == "{eov}")
        || (last_path == "Chorus" && line == "{eoc}")
        || (last_path == "Bridge" && line == "{eob}")
        || (last_path == "Outro" && line == "{eoo}")
}

impl Song {
    fn empty() -> Self {
        Song {
            title: String::from("undef"),
            artist: String::from("undef"),
            tags: vec![],
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
        state.verbose(String::from("  > parse_in_grid"));
        if (line == "{eog}") || (line == "{end_of_grid}") {
            state.verbose(String::from("  > finishing grid"));
            state.in_grid = false;
            state.last_line_blank = false;
            return;
        };
        let Some(last_block) = self.document.blocks.last_mut() else {
            state.verbose(format!(
                "!! last block ?? {}",
                type_name_of_val(&self.document.blocks.last())
            ));
            return;
        };
        state.verbose(format!(" last_block = {:#?}", last_block));

        match last_block {
            Block::Grid(lines) => lines.push(line.to_string()),
            Block::Unknown(_) => {
                state.verbose(String::from("Could not append to Block::Unknown"));
                return;
            }
            Block::Tabulatur(_) => {
                state.verbose(String::from("Could not append to Block::Unknown"));
                return;
            }
            Block::Part(_) => {
                state.verbose(String::from("Could not append to Block::Unknown"));
                return;
            }
        }
        return;
    }
    fn start_tabulatur(&mut self, state: &mut ParsingState) -> () {
        self.document.blocks.push(Block::Tabulatur(vec![]));
        state.in_tabulatur = true;
        state.last_line_blank = false;
    }
    fn start_grid(&mut self, state: &mut ParsingState) -> () {
        state.verbose(String::from("  -> starting grid"));
        self.document.blocks.push(Block::Grid(vec![]));
        state.verbose(format!(
            " last_block = {:#?}",
            self.document.blocks.last().unwrap()
        ));
        state.in_grid = true;
        state.last_line_blank = false;
    }
    fn start_part(&mut self, part_name: String, state: &mut ParsingState) -> () {
        self.document.blocks.push(Block::Part(SongPart {
            name: part_name.clone(),
            lines: vec![],
        }));
        state.last_part_name = Some(part_name);
        state.in_a_block = true;
        state.last_line_blank = false;
    }

    fn start_anonymous_part(&mut self, state: &mut ParsingState) -> () {
        self.start_part(String::from(""), state)
    }

    fn set_meta_key_value(&mut self, key: String, value: String, state: &mut ParsingState) -> () {
        match key.as_str() {
            "t" | "tit" | "title" => {
                state.title_found = true;
                self.title = value;
            }
            "a" | "art" | "artist" | "st" | "subtitle" => {
                state.artist_found = true;
                self.artist = value;
            }
            "tags" => {
                let mut new_tags: Vec<String> = value
                    .as_str()
                    .to_lowercase()
                    .split(',')
                    .map(str::trim)
                    .map(|str| String::from(str))
                    .collect();
                self.tags.append(&mut new_tags);
            }
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
            return self.start_grid(state);
        }
        if let Some(part_name) = should_set_part_name(line) {
            return self.start_part(part_name, state);
        }
        if let Some((key, value)) = should_set_meta_key_value(line) {
            return self.set_meta_key_value(key, value, state);
        }

        if line == "" {
            return self.handle_empty_line(state);
        }
        if !state.in_a_block {
            return self.start_anonymous_part(state);
        }
        if should_finish_current_path(line, &state.last_part_name) {
            return self.finish_part(state);
        }
        self.add_line_to_latest_part(line, state);
    }
    pub fn parse(source: &String, verbose: bool) -> Self {
        let mut song = Song::empty();
        let mut state = ParsingState::new(verbose);
        for line in source.lines() {
            state.verbose(format!("line: {}", line));
            song.parse_line(line, &mut state);
        }
        song
    }

    pub fn from_ron(stuff: String) -> Self {
        ron::from_str(&stuff).unwrap()
    }
}
