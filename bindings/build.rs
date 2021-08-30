fn main() {
    windows::build! {
        Windows::Win32::UI::WindowsAndMessaging::MessageBoxA,
        Windows::Win32::UI::KeyboardAndMouseInput::{
            INPUT,
            SendInput,
            INPUT_TYPE,
            MOUSEINPUT,
            MOUSE_EVENT_FLAGS
        },
    };
}
