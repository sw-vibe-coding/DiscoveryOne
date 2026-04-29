# Saga: discoveryone-library-grid

Goal: implement M8: a Web UI library/dictionary grid of definitions with stable sorting by arity, type, and aspect, covered by reg-rs HTML snapshots, while preserving demo ergonomics discovered during review.

Milestones:
1. Add a RED library-grid snapshot baseline for bundled definitions and stable default ordering.
2. Implement the first library grid view in the Yew app using existing bundled definition metadata.
3. Add sort controls and stable sort order for arity, type, aspect, and name tie-breaks.
4. Extend fixtures/metadata toward the planned ~6 definition grid without changing parser/checker scope.
5. Add acceptance coverage and update docs/status for the M8 demo.
6. Fix Edit mode to edit visible 2D facet syntax rather than raw/internal source text.