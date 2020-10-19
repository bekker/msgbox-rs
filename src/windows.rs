use std::iter::once;
use std::ptr::null_mut;
use winapi::um::winuser::{MessageBoxW, MB_ICONERROR, MB_ICONINFORMATION, MB_OK, MB_SYSTEMMODAL};

use common::{IconType, MsgBoxError};

pub fn create(
    title: &str,
    content: &str,
    icon_type: IconType,
) -> std::result::Result<(), MsgBoxError> {
    let lp_text: Vec<u16> = content.encode_utf16().chain(once(0)).collect();
    let lp_caption: Vec<u16> = title.encode_utf16().chain(once(0)).collect();

    let window_type = match icon_type {
        IconType::Error => MB_OK | MB_ICONERROR | MB_SYSTEMMODAL,
        IconType::Info => MB_OK | MB_ICONINFORMATION | MB_SYSTEMMODAL,
        IconType::None => MB_OK | MB_SYSTEMMODAL,
    };

    unsafe {
        /*
         * https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw#return-value
         * If the return value is zero, creating message box has failed
         */
        match MessageBoxW(
            null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            window_type,
        ) {
            0 => Err(MsgBoxError::Create(())),
            _ => Ok(()),
        }
    }
}
