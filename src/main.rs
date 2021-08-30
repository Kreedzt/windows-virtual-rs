use bindings::Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};
use bindings::Windows::Win32::UI::KeyboardAndMouseInput::{INPUT, SendInput, MOUSEEVENTF_LEFTDOWN,
INPUT_TYPE, INPUT_0, MOUSEINPUT, MOUSE_EVENT_FLAGS};

fn main() {
    unsafe {
        let mut input = INPUT {
                r#type: INPUT_TYPE(0x0),
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: MOUSE_EVENT_FLAGS(0x01),
                        time: 0,
                        dwExtraInfo: 0
                    }
                }
        };
        let ptr: *mut INPUT = &mut input;
        SendInput(
            1,
            ptr,
            std::mem::size_of::<INPUT>() as i32
        );
    }
    // unsafe {
    //     MessageBoxA(None, "Hello", "World", MB_OK);
    // }
}
