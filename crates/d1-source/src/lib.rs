//! Voxel loader for DiscoveryOne source files.
//!
//! Parses `.d1` (layered text) and `.d1.json` (canonical
//! coordinates) into a positioned-symbol set. See
//! `docs/design.md` section 1 for the file formats.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSet {
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub name: String,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub text: String,
    pub x: usize,
    pub y: usize,
    pub z: i32,
    pub aspect: String,
}

macro_rules! definition {
    ($name:expr) => {
        Definition {
            name: $name.trim().to_string(),
            symbols: Vec::new(),
        }
    };
}

macro_rules! render_face {
    ($set:expr, $face:expr) => {{
        let symbols = $set
            .definitions
            .iter()
            .flat_map(|definition| &definition.symbols);
        let mut rows = Vec::<Vec<char>>::new();
        for symbol in symbols.filter(|symbol| symbol.aspect.eq_ignore_ascii_case($face)) {
            rows.resize(rows.len().max(symbol.y + 1), Vec::new());
            let row = &mut rows[symbol.y];
            row.resize(row.len().max(symbol.x + symbol.text.len()), ' ');
            for (offset, ch) in symbol.text.chars().enumerate() {
                row[symbol.x + offset] = ch;
            }
        }
        rows.into_iter()
            .map(|row| row.into_iter().collect::<String>().trim_end().to_string())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }};
}

pub fn load_layered(source: &str) -> Result<SourceSet, String> {
    let mut definitions = Vec::<Definition>::new();
    let mut aspect = String::from("Front");
    let (mut z, mut y) = (0, 0);

    for line in source.lines().filter(|line| !line.trim_start().is_empty()) {
        let trimmed = line.trim_start();
        if let Some(name) = line.strip_prefix('*') {
            definitions.push(definition!(name));
            y = 0;
            continue;
        }
        if trimmed.starts_with('@') {
            (aspect, z) = parse_tags(trimmed, &aspect, z)?;
            y = 0;
            continue;
        }
        if let Some(definition) = definitions.last_mut() {
            push_symbols(definition, trimmed, y, z, &aspect);
            y += 1;
        }
    }
    Ok(SourceSet { definitions })
}

pub fn emit_layered(source: &str, face: Option<&str>) -> Result<String, String> {
    let source_set = load_layered(source)?;
    match face {
        Some("front") | Some("Front") => Ok(render_face!(source_set, "front")),
        Some(face) => Err(format!("unsupported face: {face}")),
        None => Ok(source.strip_suffix('\n').unwrap_or(source).to_string() + "\n"),
    }
}

fn parse_tags(line: &str, aspect: &str, z: i32) -> Result<(String, i32), String> {
    let mut next_aspect = aspect.to_string();
    let mut next_z = z;
    let mut words = line.split_whitespace();
    while let Some(word) = words.next() {
        match word {
            "@front" | "@left" | "@right" | "@top" | "@bottom" | "@rear" | "@internal" => {
                next_aspect = match word {
                    "@front" => "Front",
                    "@left" => "Left",
                    "@right" => "Right",
                    "@top" => "Top",
                    "@bottom" => "Bottom",
                    "@rear" => "Rear",
                    _ => "Internal",
                }
                .to_string();
            }
            "@z" => next_z = words.next().ok_or("@z missing depth")?.parse().unwrap_or(0),
            _ => {}
        }
    }
    Ok((next_aspect, next_z))
}

fn push_symbols(definition: &mut Definition, line: &str, y: usize, z: i32, aspect: &str) {
    let mut cursor = 0;
    for text in line.split_whitespace() {
        let x = cursor + line[cursor..].find(text).unwrap_or(0);
        cursor = x + text.len();
        definition.symbols.push(Symbol {
            text: text.to_string(),
            x,
            y,
            z,
            aspect: aspect.into(),
        });
    }
}
