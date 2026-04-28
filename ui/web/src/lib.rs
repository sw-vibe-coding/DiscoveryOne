use yew::prelude::*;

mod components;

use components::{BuildFooter, DefinitionPicker, FaceSelector, FacetView};

pub(crate) const POWER_SOURCE: &str = include_str!("../../../examples/power.d1");
pub(crate) const DEFINITIONS: &[&str] = &["Power"];
pub(crate) const FACES: [Face; 6] = [
    Face {
        label: "Front",
        query: "front",
    },
    Face {
        label: "Left",
        query: "left",
    },
    Face {
        label: "Right",
        query: "right",
    },
    Face {
        label: "Top",
        query: "top",
    },
    Face {
        label: "Bottom",
        query: "bottom",
    },
    Face {
        label: "Rear",
        query: "rear",
    },
];
pub(crate) const FRONT: Face = FACES[0];
const SNAPSHOT_BEFORE_ROWS: &str = r#"<main class="app">
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
const SNAPSHOT_AFTER_ROWS: &str = r#"</pre>
    </article>
  </section>
</main>
"#;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Face {
    pub(crate) label: &'static str,
    pub(crate) query: &'static str,
}

#[function_component(App)]
pub fn app() -> Html {
    let current_face = use_state(|| FRONT);

    html! {
        <main class="app">
            <header class="topbar">
                <img class="badge" src="discovery-one-badge.png" alt="DiscoveryOne badge" />
                <div class="titleblock">
                    <h1>{ "DiscoveryOne" }</h1>
                    <span>{ "M6 web shell" }</span>
                </div>
                <DefinitionPicker />
                <FaceSelector current={*current_face} on_select={
                    let current_face = current_face.clone();
                    Callback::from(move |face| current_face.set(face))
                } />
            </header>
            <section class="workspace">
                <FacetView face={*current_face} />
            </section>
            <BuildFooter />
        </main>
    }
}

pub(crate) fn facet_rows(face: Face) -> Vec<String> {
    d1_source::emit_layered(POWER_SOURCE, Some(face.query))
        .expect("bundled Power fixture should project")
        .trim_end()
        .lines()
        .map(str::to_owned)
        .collect()
}

pub fn power_front_facet_html_snapshot() -> String {
    let rows = facet_rows(FRONT).join("\n");
    format!("{SNAPSHOT_BEFORE_ROWS}{rows}{SNAPSHOT_AFTER_ROWS}")
}
