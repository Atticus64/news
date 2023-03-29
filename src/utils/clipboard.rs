use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(link: &str) {
    let mut ctx = ClipboardContext::new().expect("Failed to create clipboard context");
    ctx.set_contents(link.to_owned())
        .expect("Failed to copy to clipboard");
}
