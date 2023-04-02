use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(link: &str) {
    let mut ctx = ClipboardContext::new().expect("Failed to get");
    ctx.set_contents(link.to_string()).expect("Failed to get");
    assert_eq!(ctx.get_contents().unwrap(), link.to_string());
}
