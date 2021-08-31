use bindings::Windows::Win32::UI::KeyboardAndMouseInput::{
    SendInput, INPUT, INPUT_0, INPUT_MOUSE, INPUT_TYPE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEINPUT, MOUSE_EVENT_FLAGS,
};
use bindings::Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};

fn main() {
    unsafe {
        let input_1 = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_RIGHTDOWN,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        let input_2 = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_RIGHTUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        let mut input_arr = [input_1, input_2];
        let ptr = &mut input_arr as *mut INPUT;
        SendInput(2, ptr, std::mem::size_of::<INPUT>() as i32);
    }

    // unsafe {
    //     MessageBoxA(None, "Hello", "World", MB_OK);
    // }
}
