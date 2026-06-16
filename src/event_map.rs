use rotex_window::{KeyCode, NativeKeyCode};
use winit::keyboard::{KeyCode as WinitKeyCode, NativeKeyCode as WinitNativeKeyCode, PhysicalKey};

pub fn native_key_code_from_winit(code: WinitNativeKeyCode) -> NativeKeyCode {
    match code {
        WinitNativeKeyCode::Unidentified => NativeKeyCode::Unidentified,
        WinitNativeKeyCode::Android(code) => NativeKeyCode::Android(code),
        WinitNativeKeyCode::MacOS(code) => NativeKeyCode::MacOS(code),
        WinitNativeKeyCode::Windows(code) => NativeKeyCode::Windows(code),
        WinitNativeKeyCode::Xkb(code) => NativeKeyCode::Xkb(code),
    }
}

pub fn key_code_from_winit(code: WinitKeyCode) -> KeyCode {
    match code {
        WinitKeyCode::Backquote => KeyCode::Backquote,
        WinitKeyCode::Backslash => KeyCode::Backslash,
        WinitKeyCode::BracketLeft => KeyCode::BracketLeft,
        WinitKeyCode::BracketRight => KeyCode::BracketRight,
        WinitKeyCode::Comma => KeyCode::Comma,
        WinitKeyCode::Digit0 => KeyCode::Digit0,
        WinitKeyCode::Digit1 => KeyCode::Digit1,
        WinitKeyCode::Digit2 => KeyCode::Digit2,
        WinitKeyCode::Digit3 => KeyCode::Digit3,
        WinitKeyCode::Digit4 => KeyCode::Digit4,
        WinitKeyCode::Digit5 => KeyCode::Digit5,
        WinitKeyCode::Digit6 => KeyCode::Digit6,
        WinitKeyCode::Digit7 => KeyCode::Digit7,
        WinitKeyCode::Digit8 => KeyCode::Digit8,
        WinitKeyCode::Digit9 => KeyCode::Digit9,
        WinitKeyCode::Equal => KeyCode::Equal,
        WinitKeyCode::IntlBackslash => KeyCode::IntlBackslash,
        WinitKeyCode::IntlRo => KeyCode::IntlRo,
        WinitKeyCode::IntlYen => KeyCode::IntlYen,
        WinitKeyCode::KeyA => KeyCode::KeyA,
        WinitKeyCode::KeyB => KeyCode::KeyB,
        WinitKeyCode::KeyC => KeyCode::KeyC,
        WinitKeyCode::KeyD => KeyCode::KeyD,
        WinitKeyCode::KeyE => KeyCode::KeyE,
        WinitKeyCode::KeyF => KeyCode::KeyF,
        WinitKeyCode::KeyG => KeyCode::KeyG,
        WinitKeyCode::KeyH => KeyCode::KeyH,
        WinitKeyCode::KeyI => KeyCode::KeyI,
        WinitKeyCode::KeyJ => KeyCode::KeyJ,
        WinitKeyCode::KeyK => KeyCode::KeyK,
        WinitKeyCode::KeyL => KeyCode::KeyL,
        WinitKeyCode::KeyM => KeyCode::KeyM,
        WinitKeyCode::KeyN => KeyCode::KeyN,
        WinitKeyCode::KeyO => KeyCode::KeyO,
        WinitKeyCode::KeyP => KeyCode::KeyP,
        WinitKeyCode::KeyQ => KeyCode::KeyQ,
        WinitKeyCode::KeyR => KeyCode::KeyR,
        WinitKeyCode::KeyS => KeyCode::KeyS,
        WinitKeyCode::KeyT => KeyCode::KeyT,
        WinitKeyCode::KeyU => KeyCode::KeyU,
        WinitKeyCode::KeyV => KeyCode::KeyV,
        WinitKeyCode::KeyW => KeyCode::KeyW,
        WinitKeyCode::KeyX => KeyCode::KeyX,
        WinitKeyCode::KeyY => KeyCode::KeyY,
        WinitKeyCode::KeyZ => KeyCode::KeyZ,
        WinitKeyCode::Minus => KeyCode::Minus,
        WinitKeyCode::Period => KeyCode::Period,
        WinitKeyCode::Quote => KeyCode::Quote,
        WinitKeyCode::Semicolon => KeyCode::Semicolon,
        WinitKeyCode::Slash => KeyCode::Slash,
        WinitKeyCode::AltLeft => KeyCode::AltLeft,
        WinitKeyCode::AltRight => KeyCode::AltRight,
        WinitKeyCode::Backspace => KeyCode::Backspace,
        WinitKeyCode::CapsLock => KeyCode::CapsLock,
        WinitKeyCode::ContextMenu => KeyCode::ContextMenu,
        WinitKeyCode::ControlLeft => KeyCode::ControlLeft,
        WinitKeyCode::ControlRight => KeyCode::ControlRight,
        WinitKeyCode::Enter => KeyCode::Enter,
        WinitKeyCode::SuperLeft => KeyCode::SuperLeft,
        WinitKeyCode::SuperRight => KeyCode::SuperRight,
        WinitKeyCode::ShiftLeft => KeyCode::ShiftLeft,
        WinitKeyCode::ShiftRight => KeyCode::ShiftRight,
        WinitKeyCode::Space => KeyCode::Space,
        WinitKeyCode::Tab => KeyCode::Tab,
        WinitKeyCode::Convert => KeyCode::Convert,
        WinitKeyCode::KanaMode => KeyCode::KanaMode,
        WinitKeyCode::Lang1 => KeyCode::Lang1,
        WinitKeyCode::Lang2 => KeyCode::Lang2,
        WinitKeyCode::Lang3 => KeyCode::Lang3,
        WinitKeyCode::Lang4 => KeyCode::Lang4,
        WinitKeyCode::Lang5 => KeyCode::Lang5,
        WinitKeyCode::NonConvert => KeyCode::NonConvert,
        WinitKeyCode::Delete => KeyCode::Delete,
        WinitKeyCode::End => KeyCode::End,
        WinitKeyCode::Help => KeyCode::Help,
        WinitKeyCode::Home => KeyCode::Home,
        WinitKeyCode::Insert => KeyCode::Insert,
        WinitKeyCode::PageDown => KeyCode::PageDown,
        WinitKeyCode::PageUp => KeyCode::PageUp,
        WinitKeyCode::ArrowDown => KeyCode::ArrowDown,
        WinitKeyCode::ArrowLeft => KeyCode::ArrowLeft,
        WinitKeyCode::ArrowRight => KeyCode::ArrowRight,
        WinitKeyCode::ArrowUp => KeyCode::ArrowUp,
        WinitKeyCode::NumLock => KeyCode::NumLock,
        WinitKeyCode::Numpad0 => KeyCode::Numpad0,
        WinitKeyCode::Numpad1 => KeyCode::Numpad1,
        WinitKeyCode::Numpad2 => KeyCode::Numpad2,
        WinitKeyCode::Numpad3 => KeyCode::Numpad3,
        WinitKeyCode::Numpad4 => KeyCode::Numpad4,
        WinitKeyCode::Numpad5 => KeyCode::Numpad5,
        WinitKeyCode::Numpad6 => KeyCode::Numpad6,
        WinitKeyCode::Numpad7 => KeyCode::Numpad7,
        WinitKeyCode::Numpad8 => KeyCode::Numpad8,
        WinitKeyCode::Numpad9 => KeyCode::Numpad9,
        WinitKeyCode::NumpadAdd => KeyCode::NumpadAdd,
        WinitKeyCode::NumpadBackspace => KeyCode::NumpadBackspace,
        WinitKeyCode::NumpadClear => KeyCode::NumpadClear,
        WinitKeyCode::NumpadClearEntry => KeyCode::NumpadClearEntry,
        WinitKeyCode::NumpadComma => KeyCode::NumpadComma,
        WinitKeyCode::NumpadDecimal => KeyCode::NumpadDecimal,
        WinitKeyCode::NumpadDivide => KeyCode::NumpadDivide,
        WinitKeyCode::NumpadEnter => KeyCode::NumpadEnter,
        WinitKeyCode::NumpadEqual => KeyCode::NumpadEqual,
        WinitKeyCode::NumpadHash => KeyCode::NumpadHash,
        WinitKeyCode::NumpadMemoryAdd => KeyCode::NumpadMemoryAdd,
        WinitKeyCode::NumpadMemoryClear => KeyCode::NumpadMemoryClear,
        WinitKeyCode::NumpadMemoryRecall => KeyCode::NumpadMemoryRecall,
        WinitKeyCode::NumpadMemoryStore => KeyCode::NumpadMemoryStore,
        WinitKeyCode::NumpadMemorySubtract => KeyCode::NumpadMemorySubtract,
        WinitKeyCode::NumpadMultiply => KeyCode::NumpadMultiply,
        WinitKeyCode::NumpadParenLeft => KeyCode::NumpadParenLeft,
        WinitKeyCode::NumpadParenRight => KeyCode::NumpadParenRight,
        WinitKeyCode::NumpadStar => KeyCode::NumpadStar,
        WinitKeyCode::NumpadSubtract => KeyCode::NumpadSubtract,
        WinitKeyCode::Escape => KeyCode::Escape,
        WinitKeyCode::Fn => KeyCode::Fn,
        WinitKeyCode::FnLock => KeyCode::FnLock,
        WinitKeyCode::PrintScreen => KeyCode::PrintScreen,
        WinitKeyCode::ScrollLock => KeyCode::ScrollLock,
        WinitKeyCode::Pause => KeyCode::Pause,
        WinitKeyCode::BrowserBack => KeyCode::BrowserBack,
        WinitKeyCode::BrowserFavorites => KeyCode::BrowserFavorites,
        WinitKeyCode::BrowserForward => KeyCode::BrowserForward,
        WinitKeyCode::BrowserHome => KeyCode::BrowserHome,
        WinitKeyCode::BrowserRefresh => KeyCode::BrowserRefresh,
        WinitKeyCode::BrowserSearch => KeyCode::BrowserSearch,
        WinitKeyCode::BrowserStop => KeyCode::BrowserStop,
        WinitKeyCode::Eject => KeyCode::Eject,
        WinitKeyCode::LaunchApp1 => KeyCode::LaunchApp1,
        WinitKeyCode::LaunchApp2 => KeyCode::LaunchApp2,
        WinitKeyCode::LaunchMail => KeyCode::LaunchMail,
        WinitKeyCode::MediaPlayPause => KeyCode::MediaPlayPause,
        WinitKeyCode::MediaSelect => KeyCode::MediaSelect,
        WinitKeyCode::MediaStop => KeyCode::MediaStop,
        WinitKeyCode::MediaTrackNext => KeyCode::MediaTrackNext,
        WinitKeyCode::MediaTrackPrevious => KeyCode::MediaTrackPrevious,
        WinitKeyCode::Power => KeyCode::Power,
        WinitKeyCode::Sleep => KeyCode::Sleep,
        WinitKeyCode::AudioVolumeDown => KeyCode::AudioVolumeDown,
        WinitKeyCode::AudioVolumeMute => KeyCode::AudioVolumeMute,
        WinitKeyCode::AudioVolumeUp => KeyCode::AudioVolumeUp,
        WinitKeyCode::WakeUp => KeyCode::WakeUp,
        WinitKeyCode::Meta => KeyCode::Meta,
        WinitKeyCode::Hyper => KeyCode::Hyper,
        WinitKeyCode::Turbo => KeyCode::Turbo,
        WinitKeyCode::Abort => KeyCode::Abort,
        WinitKeyCode::Resume => KeyCode::Resume,
        WinitKeyCode::Suspend => KeyCode::Suspend,
        WinitKeyCode::Again => KeyCode::Again,
        WinitKeyCode::Copy => KeyCode::Copy,
        WinitKeyCode::Cut => KeyCode::Cut,
        WinitKeyCode::Find => KeyCode::Find,
        WinitKeyCode::Open => KeyCode::Open,
        WinitKeyCode::Paste => KeyCode::Paste,
        WinitKeyCode::Props => KeyCode::Props,
        WinitKeyCode::Select => KeyCode::Select,
        WinitKeyCode::Undo => KeyCode::Undo,
        WinitKeyCode::Hiragana => KeyCode::Hiragana,
        WinitKeyCode::Katakana => KeyCode::Katakana,
        WinitKeyCode::F1 => KeyCode::F1,
        WinitKeyCode::F2 => KeyCode::F2,
        WinitKeyCode::F3 => KeyCode::F3,
        WinitKeyCode::F4 => KeyCode::F4,
        WinitKeyCode::F5 => KeyCode::F5,
        WinitKeyCode::F6 => KeyCode::F6,
        WinitKeyCode::F7 => KeyCode::F7,
        WinitKeyCode::F8 => KeyCode::F8,
        WinitKeyCode::F9 => KeyCode::F9,
        WinitKeyCode::F10 => KeyCode::F10,
        WinitKeyCode::F11 => KeyCode::F11,
        WinitKeyCode::F12 => KeyCode::F12,
        WinitKeyCode::F13 => KeyCode::F13,
        WinitKeyCode::F14 => KeyCode::F14,
        WinitKeyCode::F15 => KeyCode::F15,
        WinitKeyCode::F16 => KeyCode::F16,
        WinitKeyCode::F17 => KeyCode::F17,
        WinitKeyCode::F18 => KeyCode::F18,
        WinitKeyCode::F19 => KeyCode::F19,
        WinitKeyCode::F20 => KeyCode::F20,
        WinitKeyCode::F21 => KeyCode::F21,
        WinitKeyCode::F22 => KeyCode::F22,
        WinitKeyCode::F23 => KeyCode::F23,
        WinitKeyCode::F24 => KeyCode::F24,
        WinitKeyCode::F25 => KeyCode::F25,
        WinitKeyCode::F26 => KeyCode::F26,
        WinitKeyCode::F27 => KeyCode::F27,
        WinitKeyCode::F28 => KeyCode::F28,
        WinitKeyCode::F29 => KeyCode::F29,
        WinitKeyCode::F30 => KeyCode::F30,
        WinitKeyCode::F31 => KeyCode::F31,
        WinitKeyCode::F32 => KeyCode::F32,
        WinitKeyCode::F33 => KeyCode::F33,
        WinitKeyCode::F34 => KeyCode::F34,
        WinitKeyCode::F35 => KeyCode::F35,
        _ => KeyCode::Unidentified(NativeKeyCode::Unidentified),
    }
}

pub fn physical_key_from_winit(key: PhysicalKey) -> KeyCode {
    match key {
        PhysicalKey::Code(code) => key_code_from_winit(code),
        PhysicalKey::Unidentified(native) => {
            KeyCode::Unidentified(native_key_code_from_winit(native))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::keyboard::{KeyCode as WinitKeyCode, NativeKeyCode as WinitNativeKeyCode};

    #[test]
    fn maps_letter_keys() {
        assert_eq!(key_code_from_winit(WinitKeyCode::KeyW), KeyCode::KeyW);
        assert_eq!(key_code_from_winit(WinitKeyCode::KeyA), KeyCode::KeyA);
    }

    #[test]
    fn maps_digit_keys() {
        assert_eq!(key_code_from_winit(WinitKeyCode::Digit0), KeyCode::Digit0);
        assert_eq!(key_code_from_winit(WinitKeyCode::Digit9), KeyCode::Digit9);
    }

    #[test]
    fn maps_modifier_keys() {
        assert_eq!(
            key_code_from_winit(WinitKeyCode::ShiftLeft),
            KeyCode::ShiftLeft
        );
        assert_eq!(
            key_code_from_winit(WinitKeyCode::ControlRight),
            KeyCode::ControlRight
        );
        assert_eq!(
            key_code_from_winit(WinitKeyCode::SuperLeft),
            KeyCode::SuperLeft
        );
    }

    #[test]
    fn maps_numpad_keys() {
        assert_eq!(
            key_code_from_winit(WinitKeyCode::NumpadEnter),
            KeyCode::NumpadEnter
        );
        assert_eq!(
            key_code_from_winit(WinitKeyCode::NumpadMultiply),
            KeyCode::NumpadMultiply
        );
    }

    #[test]
    fn maps_function_keys() {
        assert_eq!(key_code_from_winit(WinitKeyCode::F1), KeyCode::F1);
        assert_eq!(key_code_from_winit(WinitKeyCode::F12), KeyCode::F12);
        assert_eq!(key_code_from_winit(WinitKeyCode::F35), KeyCode::F35);
    }

    #[test]
    fn maps_media_keys() {
        assert_eq!(
            key_code_from_winit(WinitKeyCode::MediaPlayPause),
            KeyCode::MediaPlayPause
        );
        assert_eq!(
            key_code_from_winit(WinitKeyCode::AudioVolumeUp),
            KeyCode::AudioVolumeUp
        );
    }

    #[test]
    fn maps_unidentified_physical_key() {
        let key = PhysicalKey::Unidentified(WinitNativeKeyCode::Windows(0x1E));
        assert_eq!(
            physical_key_from_winit(key),
            KeyCode::Unidentified(NativeKeyCode::Windows(0x1E))
        );
    }

    #[test]
    fn maps_native_key_codes() {
        assert_eq!(
            native_key_code_from_winit(WinitNativeKeyCode::Xkb(42)),
            NativeKeyCode::Xkb(42)
        );
        assert_eq!(
            native_key_code_from_winit(WinitNativeKeyCode::Unidentified),
            NativeKeyCode::Unidentified
        );
    }
}
