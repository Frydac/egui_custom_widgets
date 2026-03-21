//! Reusable custom widgets built on top of `egui`.
//!
//! # Example
//!
//! ```ignore
//! use egui_custom_widgets::DigitwiseNumberEditor;
//!
//! fn show_editor(ui: &mut egui::Ui, value: &mut u64) {
//!     let output = DigitwiseNumberEditor::new("sample_count", value)
//!         .digits(9)
//!         .max(999_999_999)
//!         .dim_leading_zeroes(true)
//!         .show(ui);
//!
//!     if output.changed {
//!         // react to the new value
//!     }
//! }
//! ```

mod digitwise_number_editor;

pub use digitwise_number_editor::{
    DigitwiseNumberEditor, DigitwiseNumberEditorAction, DigitwiseNumberEditorOutput,
    focused_widget_is_digitwise_editor,
};
