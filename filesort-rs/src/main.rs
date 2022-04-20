use cursive::Cursive;
use cursive::align::HAlign;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView, TextView};
use cursive::traits::*;

fn main() {
    let mut siv = cursive::default();

    // Some description text. We want it to be long, but not _too_ long.
    let text = "Welcome to filesort!";

    // We'll create a dialog with a TextView serving as a title
    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Filesort v0.0.1").h_align(HAlign::Center))
                // Use a DummyView as spacer
                .child(DummyView.fixed_height(1))
                // Disabling scrollable means the view cannot shrink.

                .child(TextView::new("Input folder path:"))
                .child(EditView::new()
                    // TODO Make below verify existance
                    // .on_submit(show_popup)
                    .with_name("instr"))
                .child(TextView::new("Output folder path:"))
                .child(EditView::new()
                    // TODO Make below verify existance
                    // .on_submit(show_popup)
                    .with_name("outstr"))
                .fixed_width(60),
        )
        .button("Ok", |s| {
            {
                let instr =
                    s.call_on_name("input", |view: &mut EditView| {
                        view.get_content()
                    }).unwrap();
        })
        .button("Quit", |s| s.quit())
        .h_align(HAlign::Center),
    );

siv.run();
}

// This will replace the current layer with a new popup.
// If the name is empty, we'll show an error message instead.
fn show_popup(s: &mut Cursive, name: &str) {
    if name.is_empty() {
        // Try again as many times as we need!
        s.add_layer(Dialog::info("Please enter a name!"));
    } else {
        let content = format!("Hello {}!", name);
        // Remove the initial popup
        s.pop_layer();
        // And put a new one instead
        s.add_layer(
            Dialog::around(TextView::new(content))
                .button("Quit", |s| s.quit()),
        );
    }
}