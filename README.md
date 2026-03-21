# egui_custom_widgets

[![CI](https://github.com/Frydac/egui_custom_widgets/actions/workflows/ci.yml/badge.svg)](https://github.com/Frydac/egui_custom_widgets/actions/workflows/ci.yml)

Reusable custom widgets built on top of `egui`.

This repository currently exposes `DigitwiseNumberEditor`, a numeric editor that:

- focuses and edits individual digits
- supports arrow-key navigation and increment/decrement
- supports direct digit replacement from keyboard input
- supports vertical drag adjustment per digit
- can dim leading zeroes and show digit grouping separators

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
egui_custom_widgets = { git = "https://github.com/Frydac/egui_custom_widgets" }
```

## Example

```rust
use egui_custom_widgets::DigitwiseNumberEditor;

fn show_editor(ui: &mut egui::Ui, value: &mut u64) {
    let output = DigitwiseNumberEditor::new("sample_count", value)
        .digits(9)
        .max(999_999_999)
        .dim_leading_zeroes(true)
        .show(ui);

    if output.changed {
        // react to the new value
    }
}
```

Run the included demo app with:

```bash
cargo run --example digitwise_number_editor
```

## Development

Run the same checks as CI with:

```bash
./scripts/check.sh
```

That script runs formatting, tests, and the example build check.

To enable the included pre-commit hook so these checks run automatically before each commit:

```bash
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit
```

```bash
cargo fmt
cargo test
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
