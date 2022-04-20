use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, TextView, LinearLayout};
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();

    // Create a dialog with an edit text and a button.
    // The user can either hit the <Ok> button,
    // or press Enter on the edit text.

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Filesort-rs v0.0.1").h_align(HAlign::Center))
                .child(Dialog::new()
                    .title("Filesort-rs v0.0.1")
                    // Padding is (left, right, top, bottom)
                    .padding_lrtb(1, 1, 1, 0)
                    .title("Input folder: ")
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
                .fixed_width(30),
                )
        )
            .button("Next", |s| {
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
        s.add_layer(Dialog::info("Please enter an input path!"));
    } 
    // TODO Confirm path validity and perms
    else {
        let content = format!("Input path={}", name);
        // Remove the initial popup
        s.pop_layer();
        // And put a new one instead
        output_popup(s, &name);
    }

}

fn output_popup(s: &mut Cursive, instr:  &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Filesort-rs v0.0.1").h_align(HAlign::Center))
                .child(Dialog::new()
                    .title("Filesort-rs v0.0.1")
                    // Padding is (left, right, top, bottom)
                    .padding_lrtb(1, 1, 1, 0)
                    .title("Output folder: ")
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
                .fixed_width(30),
                )
        )
            .button("Next", |s| {
                // This will run the given closure, *ONLY* if a view with the
                // correct type and the given name is found.
                let name = s
                    .call_on_name("name", |view: &mut EditView| {
                        // We can return content from the closure!
                        view.get_content()
                    })
                    .unwrap();
                // Run the next step
                print_config(s, instrin, &name)

                
            })
            // let &str instrin = instr;
            .button("Quit", |s| s.quit())
            .h_align(HAlign::Center),
    );
    
}

fn print_config(s: &mut Cursive, instrin: &str, outstr: &str) {
    let content = format!("Input path:{}\nOutput path:{}", instrin, outstr);
    s.pop_layer();
    s.add_layer(
        Dialog::text(content),
    );
}
