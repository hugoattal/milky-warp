#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "linux")]
extern crate x11;


#[cfg(target_os = "windows")]
pub fn get_screen_index_from_cursor_pos(cursor_pos: (i32, i32)) -> u32 {
    use winapi::shared::windef::{POINT, HMONITOR, HDC, LPRECT};
    use winapi::shared::minwindef::{LPARAM, BOOL};
    use winapi::um::winuser::{MonitorFromPoint, MONITOR_DEFAULTTONEAREST, EnumDisplayMonitors};

    let point = POINT { x: cursor_pos.0, y: cursor_pos.1 };

    let monitor = unsafe { MonitorFromPoint(point, MONITOR_DEFAULTTONEAREST) };
    let mut index = 0;

    static mut GLOBAL_MONITOR: HMONITOR = std::ptr::null_mut();
    static mut GLOBAL_INDEX: u32 = 0;

    unsafe {
        GLOBAL_MONITOR = monitor;
        GLOBAL_INDEX = index;
    }

    unsafe extern "system" fn monitor_enum_proc(hmonitor: HMONITOR, _: HDC, _: LPRECT, _: LPARAM) -> BOOL {
        if hmonitor == GLOBAL_MONITOR {
            return 0;
        }
        GLOBAL_INDEX += 1;
        1
    }

    unsafe {
        EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            Some(monitor_enum_proc),
            0,
        )
    };

    unsafe {
        index = GLOBAL_INDEX;
    }

    return index;
}


#[cfg(target_os = "linux")]
pub fn get_screen_index_from_cursor_pos(cursor_pos: (i32, i32)) -> u32 {
    return 0;
}

#[cfg(target_os = "macos")]
pub fn get_screen_index_from_cursor_pos(cursor_pos: (i32, i32)) -> u32 {
    return 0;
}
