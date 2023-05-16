/// Exact Duplicate of `termion::event::Key` for compatibility with the UNIX implementations.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg(target_family = "windows")]
pub enum Key {
    /// Backspace.
    Backspace,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Backward Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    F(u8),
    /// Normal character.
    Char(char),
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    ///
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Null byte.
    Null,
    /// Esc key.
    Esc,

    #[doc(hidden)]
    __IsNotComplete,
}

/// Reexport of `termion::event::Key`  
#[cfg(target_family = "unix")]
pub use termion::event::Key;

#[cfg(target_family = "windows")]
use windows::Win32::System::Console::{
    GetStdHandle, ReadConsoleInputA, INPUT_RECORD, KEY_EVENT, KEY_EVENT_RECORD, STD_INPUT_HANDLE,
};

//todo need error enum
///Returns a Windows Virtual Key Code.  
///[Read about Virtual Key Codes in the `winuser.h` documentation.](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
#[cfg(target_family = "windows")]
pub fn getvk() -> Result<u16, ()> {
    let mut v: Vec<INPUT_RECORD> = Vec::new();
    let mut irec: &mut [INPUT_RECORD] = &mut v[..];
    let mut ev_rec: KEY_EVENT_RECORD;
    let mut numberofeventsread: u32 = 1;
    'key: loop {
        unsafe {
            ReadConsoleInputA(
                match GetStdHandle(STD_INPUT_HANDLE) {
                    Ok(handle) => handle,
                    Err(_) => return Err(()),
                },
                &mut irec,
                &mut numberofeventsread,
            );
            if irec.len() <= 0 { continue 'key; }
        }

        if irec[0].EventType == KEY_EVENT as u16 {
            unsafe {
                ev_rec = irec[0].Event.KeyEvent;
            }
            if ev_rec.bKeyDown.into() {
                let vk = ev_rec.wVirtualKeyCode;
                return Ok(vk);
                match vk {
                    //todo we are meant to exclude certain keys here because reasons
                    _ => return Ok(vk),
                }
            }
        }
    }
}   

/// Wrapper function around `getvk()` and `vk_to_key()`.
#[cfg(target_family = "windows")]
pub fn getkey() -> Result<Key, ()> {
    let vk = getvk()?;
    return vk_to_key(vk)
}   

/// Matches a VK code to it's relevant `getkey::Key`
#[cfg(target_family = "windows")]
pub fn vk_to_key(vk: u16) -> Result<Key, ()> {
    return match vk {
        0x08 => Ok(Key::Backspace),
        0x25 => Ok(Key::Left),
        0x27 => Ok(Key::Right),
        0x26 => Ok(Key::Up),
        0x28 => Ok(Key::Down),
        0x24 => Ok(Key::Home),
        0x23 => Ok(Key::End),
        0x21 => Ok(Key::PageUp),
        0x22 => Ok(Key::PageDown),

        // ! This is the Equivalent of Shift + Tab. On Windows, Shift is it's own VK, therefore we use an undefined Keycode
        0x07 => Ok(Key::BackTab),

        0x2E => Ok(Key::Delete),
        0x2D => Ok(Key::Insert),

        // ? Unlike Native Termion, Windows Supports all the way from F1-F24.  
        // This means that termion functions will not inherently support this, if F(n) => n > 12 
        0x70..=0x87 => Ok(Key::F((vk - 0x70) as u8)),

        // ? Windows VKs map these to their ASCII chars naturally.
        0x20 | 0x30..=0x39 => Ok(Key::Char((vk as u8) as char)),

        // ? These Map to ASCII numbers + 0x30
        0x60..=0x69 => Ok(Key::Char(((vk - 0x30) as u8) as char)),

        // ? Windows VKs map to the uppercase ASCII chars naturally.
        // To make them lowercase efficiently, we use `(vk | 32) as char`
        0x41..=0x5A => Ok(Key::Char(((vk | 32) as u8) as char)),

        //todo add 0x6A..=0x6F

        // Note that Windows VKs don't support modifier keys, so we use the null character here.
        0x12 => Ok(Key::Alt('\0')),

        // Note that Windows VKs don't support modifier keys, so we use the null character here.
        0x11 => Ok(Key::Ctrl('\0')),

        // Null byte.
        // ! This is impossible with Windows' VKs, therefore we use an undefined Keycode
        0x03 => Ok(Key::Null),
        // Esc key.
        0x1B => Ok(Key::Esc),
        _ => Err(())
    }
}

#[cfg(target_family = "unix")]
use std::io::{self, Write};
#[cfg(target_family = "unix")]
use termion::{input::TermRead, raw::IntoRawMode};

/// Awaits Input, then returns a `termion::event::Key`, or an Empty Error.
#[cfg(target_family = "unix")]
pub fn getkey() -> Result<Key, ()> {
    let stdin = io::stdin();
    //setting up stdout and going into raw mode
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    //detecting keydown events
    return match stdin.keys().next() {
        Some(k) => {
            return match k {
                Ok(k) => Ok(k),
                Err(_) => Err(())
            }
        },
        None => Err(())
    }
}
