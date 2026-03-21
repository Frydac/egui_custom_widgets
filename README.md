# egui_custom_widgets

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

## Development

```bash
cargo fmt
cargo test
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
