fn main() {
    windows::build! {
        Windows::Win32::UI::WindowsAndMessaging::{
            MessageBoxA,
            EnumWindows,
            GetWindowTextW,
            GetWindowRect
        },
        Windows::Win32::UI::KeyboardAndMouseInput::{
            INPUT,
            SendInput,
            INPUT_TYPE,
            MOUSEINPUT,
            MOUSE_EVENT_FLAGS,
        },
    };
}
