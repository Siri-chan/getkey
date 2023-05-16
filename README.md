# getkey

**IMPORTANT NOTE:** `getvk()` currently hangs on windows. I do not have the patience to do any more with the `windows` crate, so it can be somebody else's problem.  
This is also why this crate is not on crates.io

# What is getkey?

getkey is a library that provides an easy way to get a key from the terminal in any OS that supports either Termion or Windows.  
This is not inherently cross-platform (see below), but should provide memory-safe convenience functions on either platform.  
This program is completely public-domain, and is open to contribution, if you believe you can improve the implementation, open a pull request.

## Quick-Start

Add to Cargo.toml:

```toml
[dependencies]
getkey = { git = "<https://github.com/Siri-chan/getkey.git>" }
```

<!--
```toml
[dependencies]
getkey = "*"
```
-->

Then code:

```rs
// This code is also in `examples/quickstart.rs`, you can `cargo run --example quickstart` from within the getkey repo to see it in action.
use getkey::*;

fn main() {
    println!("Press the Up Arrow to start.");
    loop {
        let key = getkey();
        match key {
            Ok(k) => if k == Key::Up {
                println!("Started!\n Finished!");
                return;
            }
            _ => {}
        }
        println!("Wrong Key!\nPress the Up Arrow to start.");
    }
}
```

## UNIX

UNIX support is just a singular function, `getkey()`, that natively returns the first `termion::event::Key` that is pressed.

## Windows

**IMPORTANT:** I have not tested this code on Windows, and have already had enough headaches with the Windows API. If you want to try to test/fix it, feel free.

Windows support is divided into 3 functions: `getvk()`, `vk_to_key()` and `getkey()`.

- `getvk()` retrieves input directly as a [Virtual Key Code](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes). These are also constants in `windows::Win32::UI::Input::KeyboardAndMouse`, denoted by `VK_*`.  
    If you are writing an application solely for Windows, this is likely what you want to use.
    **NOTE:** `getvk()` currently hangs on windows. I do not have the patience to do any more with the `windows` crate, so it can be somebody else's problem.  
    This is also why this crate is not on crates.io
- `vk_to_key()` converts a Virtual Key Code into a `termion::event::Key`.  
    This function is intended for internal use, solely for converting to a `termion::event::Key`.  
    **This is NOT a perfect conversion. There is a list of caveats below.**
- `getkey()` combines these two functions to directly return a `termion::event::Key`.  
    This function is likely what you want to use if writing an app that needs to be cross-platform with a UNIX-like.  
    **Keep in mind the list of caveats with `vk_to_key()` when using this function.**

### Caveats with using `vk_to_key()`

`vk_to_key()` is a function that transforms a Windows VK to a `termion::event::Key`. These are fundamemtally different structures, that represent different things.
Windows VKs represent an individual keystroke, where `termion::event::Key` represents an individual key input.
This leads to a few things being lost in translation.

- Control, Alt and Shift are their own keystroke in Windows, but a Modifier in `termion`.  
This means that the `Alt(char)`, `Shift(char)` and `Ctrl(char)` variants don't work as intended.  
This means that case-sensitivity, or other Modifier-Key combinations aren't particularly compatible.
- The `termion::event::Key` enum contains fewer other codes than Windows VKs do, so some key-types may be lost, such as any of the `VK_VOLUME_*` keys.

## Suggestions for writing cross-platform code

If you intend to write cross-platform code, it is important to know the differences between the two representations.  
When using `getkey()`, be aware that the Windows implementation is not a one-to-one conversion to `termion::event::Key`. You can read more about that above.  
I suggest either limiting your programs to fit within `getkey()`'s constraints or writing your own implementation. This will save you a lot of headache.

## Contributing

I am a one person team. Open a pull request, and be patient. I will test and merge it eventually.
