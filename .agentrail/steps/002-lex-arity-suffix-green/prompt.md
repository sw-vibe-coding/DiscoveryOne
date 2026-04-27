# Step: lex-arity-suffix-green

Implement the minimum `d1-lex` and `d1 lex` CLI path needed to
turn `d1_lex_arity_suffix` from RED to GREEN.

## Source-of-truth

- `docs/plan.md` section 3, M1 TDD sequence item 2.
- `docs/design.md` section 3 for the DiscoveryOne token set.
- `tests/lexer/arity_suffix.expected` for the required output.
- `../tuplet/docs/lexer.md` dump-format notes for token naming.

## Work

1. Implement a small hand-written ASCII lexer in `crates/d1-lex`
   that recognizes the tokens needed by
   `coord2 -> (x y)`: identifiers, right arrow, left/right
   parens, whitespace skipping, and EOF.
2. Expose a stable token dump helper that prints one token per
   line matching `tests/lexer/arity_suffix.expected`.
3. Wire `d1-cli` so `d1 lex FILE` reads the file and prints the
   token dump to stdout with exit 0. Leave all other subcommands
   as stubs.
4. Re-run or rebase `work/reg-rs/d1_lex_arity_suffix.*` so the
   baseline expects the green token dump and exit code 0.
5. Update `docs/status.md` to show the arity-suffix lexer case
   green and the next M1 work as refactor/token-display cleanup.

## Pre-commit gates

- `REG_RS_DATA_DIR=work/reg-rs reg-rs run -p d1_lex_arity_suffix`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt -- --check`
- `markdown-checker -f "**/*.md"`
- `sw-checklist` if installed; known M0 metadata failures are
  already documented in `docs/learnings.md`.

## Deliverables

- Updated `crates/d1-lex/src/lib.rs`
- Updated `crates/d1-cli`
- Updated `work/reg-rs/d1_lex_arity_suffix.rgt`
- Updated `work/reg-rs/d1_lex_arity_suffix.out`
- Removed stale `work/reg-rs/d1_lex_arity_suffix.err` if the
  green run produces no stderr
- Updated `docs/status.md`

## Commit / complete protocol

Commit the deliverables plus `.agentrail/` with message:
`feat: implement d1 lex arity suffix case`.

Then push and `agentrail complete` with reward 1.
