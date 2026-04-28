use crate::{FRONT, facet_rows, power_run_2_8_output};

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
    let rows = facet_rows(FRONT).join("\n");
    format!("{FRONT_BEFORE_ROWS}{rows}{FRONT_AFTER_ROWS}")
}

pub fn power_run_2_8_html_snapshot() -> String {
    let output = power_run_2_8_output();
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
