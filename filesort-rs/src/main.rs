use cursive::Cursive;
use cursive::align::HAlign;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView, TextView};
use cursive::traits::*;

fn main() {
    let mut siv = cursive::default();

    let text = "Welcome to filesort!";

    // We'll create a dialog with a TextView serving as a title
    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Filesort v0.0.1").h_align(HAlign::Center))
                // Use a DummyView as spacer
                .child(DummyView.fixed_height(1))
                // Disabling scrollable means the view cannot shrink.
                
                .child(Dialog::new()
                .title("Input folder: ")
                // Padding is (left, right, top, bottom)
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    EditView::new()
                        // Call `show_popup` when the user presses `Enter`
                        .on_submit(show_popup)
                        // Give the `EditView` a name so we can refer to it later.
                        .with_name("name")
                        // Wrap this in a `ResizedView` with a fixed width.
                        // Do this _after_ `with_name` or the name will point to the
                        // `ResizedView` instead of `EditView`!
                        .fixed_width(20),
                )
                .button("Ok", |s| {
                    // This will run the given closure, *ONLY* if a view with the
                    // correct type and the given name is found.
                    let name = s
                        .call_on_name("name", |view: &mut EditView| {
                            // We can return content from the closure!
                            view.get_content()
                        })
                        .unwrap();
    
                    // Run the next step
                    show_popup(s, &name);
                }))


                .child(Dialog::new()
                .title("Output folder:")
                // Padding is (left, right, top, bottom)
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    EditView::new()
                        // Call `show_popup` when the user presses `Enter`
                        .on_submit(show_popup)
                        // Give the `EditView` a name so we can refer to it later.
                        .with_name("name")
                        // Wrap this in a `ResizedView` with a fixed width.
                        // Do this _after_ `with_name` or the name will point to the
                        // `ResizedView` instead of `EditView`!
                        .fixed_width(20),
                )
                .button("Ok", |s| {
                    // This will run the given closure, *ONLY* if a view with the
                    // correct type and the given name is found.
                    let name = s
                        .call_on_name("name", |view: &mut EditView| {
                            // We can return content from the closure!
                            view.get_content()
                        })
                        .unwrap();
    
                    // Run the next step
                    show_popup(s, &name);
                }))
                
                .fixed_width(60),
        )
        .button("Ok", |s| s.quit())
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
                // .button("Ok", |s| s.)
        );
    }
}