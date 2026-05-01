fn main() {
    print!(
        r#"<section class="viewer-3d" aria-label="3D viewer">
  <header class="viewer-3d-header"><span>3D Viewer</span><strong>Power</strong></header>
  <div class="viewer-3d-export" data-definition="Power" data-symbol-count="54">
    <span data-symbol="n" data-face="Front" data-position="0,0,0">n</span>
    <span data-symbol="e" data-face="Front" data-position="2,0,0">e</span>
    <span data-symbol="p" data-face="Front" data-position="7,0,0">p</span>
    <span data-symbol="loop" data-face="Front" data-position="0,2,0">loop</span>
  </div>
  <div class="viewer-3d-assets" data-module="three" data-entry="viewer3d" data-export="window.discoveryOne3dSymbols"></div>
  <output class="viewer-3d-status" aria-label="3D viewer status">Ready: Power symbol export is wired to the viewer3d three.js scene bundle.</output>
</section>
"#
    );
}
