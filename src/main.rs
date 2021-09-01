use std::convert::TryInto;

use bindings::Windows::Win32::Foundation::{BOOL, HWND, LPARAM, PWSTR, RECT, WPARAM};
use bindings::Windows::Win32::UI::KeyboardAndMouseInput::{
    SendInput, SetActiveWindow, INPUT, INPUT_0, INPUT_MOUSE, INPUT_TYPE, MOUSEEVENTF_ABSOLUTE,
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN,
    MOUSEEVENTF_RIGHTUP, MOUSEINPUT, MOUSE_EVENT_FLAGS,
};
use bindings::Windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows,
    // GetWindow,
    // GW_HWNDFIRST
    GetWindowRect,
    GetWindowTextW,
    MessageBoxA,
    SendMessageA,
    SendMessageCallbackA,
    SetForegroundWindow,
    SetWindowPos,
    HWND_TOP,
    MB_OK,
    SWP_SHOWWINDOW,
    WM_KEYDOWN,
    WM_KEYUP,
    WM_SYSKEYDOWN,
    WM_SYSKEYUP,
    WNDENUMPROC,
    VK_TAB,
    VK_LBUTTON
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
            bottom: 0,
        };

        GetWindowRect(window, (&mut rect) as *mut RECT);

        // let mut v = Vec::with_capacity(3);
        //

        // if !text.is_empty() {
        //     println!("{}, {:?}", text, rect);
        // }

        if text == "便笺" {
            println!("{}-{:?}", text, rect);
            // v.push(rect);
            // return false.into();
            if rect.top != 1 {
                println!("Correct: {:?}", rect);

                SetActiveWindow(window);

                SetForegroundWindow(window);

                // SendMessageCallbackA(window, WM_KEYDOWN, WPARAM(0x31), LPARAM(0), None, 0);
                // SendMessageCallbackA(window, WM_KEYUP, WPARAM(0x31), LPARAM(0), None, 0);

                // SendMessageCallbackA(window, WM_SYSKEYDOWN, WPARAM(0x31), LPARAM(0), None, 0);
                // SendMessageCallbackA(window, WM_SYSKEYUP, WPARAM(0x31), LPARAM(0), None, 0);
                //
                // SendMessageA(window, WM_SYSKEYDOWN, WPARAM(0x09), LPARAM(0));
                // SendMessageA(window, WM_SYSKEYUP, WPARAM(0x09), LPARAM(0));

                SendMessageA(window, WM_SYSKEYDOWN, WPARAM(VK_TAB.try_into().unwrap()), LPARAM(0));
                SendMessageA(window, WM_SYSKEYUP, WPARAM(VK_TAB.try_into().unwrap()), LPARAM(0));

                // SetWindowPos(window, HWND_TOP, 0, 30, 720, 720, SWP_SHOWWINDOW);
            }
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
        let move_1 = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        let input_1 = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_LEFTDOWN,
                    time: 1000,
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
                    dwFlags: MOUSEEVENTF_LEFTUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        let mut input_arr = [move_1, input_1, input_2];
        let ptr = &mut input_arr as *mut INPUT;

        // SendInput(
        //     input_arr.len() as u32,
        //     ptr,
        //     std::mem::size_of::<INPUT>() as i32,
        // );
    }

    // unsafe {
    //     MessageBoxA(None, "Hello", "World", MB_OK);
    // }
}
