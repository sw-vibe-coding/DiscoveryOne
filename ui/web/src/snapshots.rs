use crate::edit_state::editable_facet_text;
use crate::runtime::{run_output, run_output_with_facet_edit};
use crate::{DEFINITIONS, FRONT, INTERNAL, LIBRARY_ROWS, LibraryRow, LibrarySort, POWER_SOURCE, facet_rows, sorted_library_rows};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

macro_rules! library_grid_html {
    ($library_rows:expr) => {{
        let rows = $library_rows
            .iter()
            .map(|row: &LibraryRow| {
                format!(
                    r#"      <tr data-definition="{name}"><td>{name}</td><td>{arity}</td><td>{category}</td><td>{aspects}</td></tr>"#,
                    name = row.name,
                    arity = row.arity,
                    category = row.category,
                    aspects = row.aspects
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            r#"<section class="library-grid" aria-label="Library definitions">
  <table>
    <thead><tr><th>Name</th><th>Arity</th><th>Type</th><th>Aspects</th></tr></thead>
    <tbody>
{rows}
    </tbody>
  </table>
</section>
"#
        )
    }};
}

const FRONT_BEFORE_ROWS: &str = r#"<main class="app">
  <header class="topbar">
    <h1>DiscoveryOne</h1>
    <label class="definition-picker"><span>Definition</span><select aria-label="Definition"><option value="Power">Power</option></select></label>
    <nav class="face-selector" aria-label="Face">
      <button type="button" class="face-button selected" aria-pressed="true">Front</button>
      <button type="button" class="face-button" aria-pressed="false">Left</button>
      <button type="button" class="face-button" aria-pressed="false">Right</button>
      <button type="button" class="face-button" aria-pressed="false">Top</button>
      <button type="button" class="face-button" aria-pressed="false">Bottom</button>
      <button type="button" class="face-button" aria-pressed="false">Rear</button>
    </nav>
  </header>
  <section class="workspace">
    <article class="facet-view" data-definition="Power" data-face="front">
      <header class="facet-header"><span>Power</span><strong>Front</strong></header>
      <pre class="facet-grid" aria-label="Power Front facet">"#;
const FRONT_AFTER_ROWS: &str = r#"</pre>
    </article>
  </section>
</main>
"#;

pub fn power_front_facet_html_snapshot() -> String {
    let rows = facet_rows(DEFINITIONS[0], FRONT).join("\n");
    format!("{FRONT_BEFORE_ROWS}{rows}{FRONT_AFTER_ROWS}")
}

pub fn library_grid_html_snapshot() -> String {
    library_grid_html!(LIBRARY_ROWS.to_vec())
}

pub fn library_grid_html_snapshot_sorted(sort: LibrarySort) -> String {
    library_grid_html!(sorted_library_rows(sort))
}

pub fn power_run_2_8_html_snapshot() -> String {
    let output = run_output(DEFINITIONS[0]);
    format!(
        r#"<aside class="run-panel" data-definition="Power">
  <header class="run-header"><span>RunPanel</span><strong>Power</strong></header>
  <div class="run-inputs" aria-label="Power inputs">
    <label><span>n</span><input value="2" readonly></label>
    <label><span>e</span><input value="8" readonly></label>
    <button type="button">Run</button>
  </div>
  <output class="run-output" aria-label="Power output">{output}</output>
</aside>
"#
    )
}

pub fn power_front_edit_run_html_snapshot() -> String {
    let edited_front = editable_facet_text(DEFINITIONS[0], FRONT).replacen("1 →", "2 →", 1);
    let output = run_output_with_facet_edit(DEFINITIONS[0], FRONT, &edited_front, "5", "0");
    format!(
        r#"<aside class="run-panel" data-definition="Power" data-edited-face="front">
  <header class="run-header"><span>RunPanel</span><strong>Power</strong></header>
  <div class="run-inputs" aria-label="Power inputs">
    <label><span>n</span><input value="5" readonly></label>
    <label><span>e</span><input value="0" readonly></label>
    <button type="button">Run</button>
  </div>
  <output class="run-output" aria-label="Power output">{output}</output>
</aside>
"#
    )
}

pub fn minted_run_html_snapshot(definition_index: usize) -> String {
    let definition = DEFINITIONS[definition_index];
    let rows = facet_rows(definition, INTERNAL).join("\n");
    let output = run_output(definition);
    let name = definition.name;
    format!(
        r#"<section class="workspace">
  <article class="facet-view" data-definition="{name}" data-face="internal">
    <header class="facet-header"><span>{name}</span><strong>Internal</strong></header>
    <pre class="facet-grid" aria-label="{name} Internal facet">{rows}</pre>
  </article>
  <aside class="run-panel" data-definition="{name}">
    <header class="run-header"><span>RunPanel</span><strong>{name}</strong></header>
    <div class="run-inputs" aria-label="Power inputs">
      <label><span>n</span><input value="2" readonly></label>
      <label><span>e</span><input value="8" readonly></label>
      <button type="button">Run</button>
    </div>
    <output class="run-output" aria-label="Power output">{output}</output>
  </aside>
</section>
"#
    )
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = discoveryOne3dSymbols))]
pub fn power_3d_symbols_json_snapshot() -> String {
    let source = d1_source::load_layered(POWER_SOURCE).expect("Power fixture should load");
    let symbols = source.definitions[0].symbols.iter().map(|symbol| {
        format!(
            r#"    {{"text":"{}","face":"{}","x":{},"y":{},"z":{}}}"#,
            symbol.text, symbol.aspect, symbol.x, symbol.y, symbol.z
        )
    }).collect::<Vec<_>>().join(",\n");
    format!(
        r#"{{
  "definition": "Power",
  "symbol_count": {},
  "symbols": [
{}
  ]
}}
"#,
        source.definitions[0].symbols.len(),
        symbols
    )
}
