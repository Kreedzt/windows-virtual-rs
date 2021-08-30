use bindings::Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};

fn main() {
    println!("Hello, world!");

    unsafe {
        MessageBoxA(None, "Hello", "World", MB_OK);
    }
}
