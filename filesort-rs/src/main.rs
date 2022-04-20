use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView};
use cursive::traits::*;

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(Dialog::text("This is a survey!\nPress <Next> when you're ready.")
        .title("Filesort-rs v0.0.1")
        .button("Next", show_next));


    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::around(EditView::new()
            .on_submit(ok)
            .with_name("Name")
            .fixed_width(10))
        .title("Filesort-rs v0.0.1")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        })
        ); 
}

// fn show_answer(s: &mut Cursive, msg: &str) {
//     s.pop_layer();
//     s.add_layer(Dialog::text(msg)
//         .title("Results")
//         .button("Finish", |s| s.quit()));
// }