use bindings::Windows::Win32::Foundation::{BOOL, HWND, LPARAM, PWSTR, RECT};
use bindings::Windows::Win32::UI::KeyboardAndMouseInput::{
    SendInput, INPUT, INPUT_0, INPUT_MOUSE, INPUT_TYPE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEINPUT, MOUSE_EVENT_FLAGS,
};
use bindings::Windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, MessageBoxA, MB_OK, WNDENUMPROC,
    GetWindowTextW,
    // GetWindow,
    // GW_HWNDFIRST
    GetWindowRect
};

extern "system" fn enum_window(window: HWND, l: LPARAM) -> BOOL {
    unsafe {
        let mut text: [u16; 512] = [0; 512];

        let len = GetWindowTextW(window, PWSTR(text.as_mut_ptr()), text.len() as i32);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        let mut rect = RECT {
            top: 0,
            left: 0,
            right: 0,
            bottom: 0
        };
        let res = GetWindowRect(window, (&mut rect) as (*mut RECT));

        if !text.is_empty() {
            println!("{}-{:?}", text, rect);
        }

        return true.into();
    }
}

fn main() {
    // 获取窗口
    unsafe {
        EnumWindows(Some(enum_window), LPARAM(0));
    }

    // 模拟点击
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
        // SendInput(2, ptr, std::mem::size_of::<INPUT>() as i32);
    }

    // unsafe {
    //     MessageBoxA(None, "Hello", "World", MB_OK);
    // }
}
