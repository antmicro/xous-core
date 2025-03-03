pub const SERVER_NAME_KBD: &str      = "_Matrix keyboard driver_";

#[derive(Debug, Default, Copy, Clone)]
pub struct ScanCode {
    /// base key value
    pub key: Option<char>,
    /// tap blue shift key, then key
    pub shift: Option<char>,
    /// hold blue shift key, then key
    pub hold: Option<char>,
    /// hold orange shift key, then key
    pub alt: Option<char>,
}

#[derive(Debug, Copy, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum KeyMap {
    Qwerty,
    Azerty,
    Qwertz,
    Dvorak,
    Braille,
    Undefined,
}
impl From<usize> for KeyMap {
    fn from(code: usize) -> Self {
        match code {
            0 => KeyMap::Qwerty,
            1 => KeyMap::Azerty,
            2 => KeyMap::Qwertz,
            3 => KeyMap::Dvorak,
            254 => KeyMap::Braille, // braille has only one mapping available
            _ => KeyMap::Qwerty,
        }
    }
}
impl Into<usize> for KeyMap {
    fn into(self) -> usize {
        match self {
            // note: these indicese correspond to the position on the keyboard menu
            KeyMap::Qwerty => 0,
            KeyMap::Azerty => 1,
            KeyMap::Qwertz => 2,
            KeyMap::Dvorak => 3,
            KeyMap::Braille => 254,
            KeyMap::Undefined => 255,
        }
    }
}

#[derive(Debug, num_derive::FromPrimitive, num_derive::ToPrimitive)]
pub(crate) enum Opcode {
    /// set which keyboard mapping is present
    SelectKeyMap, //(KeyMap),
    GetKeyMap,

    /// request for ScanCodes
    RegisterListener,

    /// request for raw keyups/downs
    RegisterRawListener,

    /// set repeat delay, rate; both in ms
    SetRepeat, //(u32, u32),

    /// set chording interval (how long to wait for all keydowns to happen before interpreting as a chord), in ms (for braille keyboards)
    SetChordInterval, //(u32),

    /// used by host mode emulation and debug UART to inject keys
    InjectKey, //(char),

    /// used by the interrupt handler to transfer results to the main loop
    HandlerTrigger,

    /// used to turn keyboard vibrate on and off
    Vibe,

    /// Suspend/resume callback
    SuspendResume,
}

// this structure is used to register a keyboard listener. Currently, we only accept
// one trusted listener (enforced by name server and structurally in the code),
// which is the GAM.
#[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Copy, Clone)]
pub(crate) struct KeyboardRegistration {
    pub server_name: xous_ipc::String::<64>,
    pub listener_op_id: usize,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RowCol {
    pub r: u8,
    pub c: u8,
}
impl RowCol {
    #[allow(dead_code)]
    pub fn new(r: u8, c: u8) -> RowCol {
        RowCol { r, c }
    }
}

#[derive(Debug)]
pub struct KeyRawStates {
    pub keydowns: Vec::<RowCol>,
    pub keyups: Vec::<RowCol>,
}
impl KeyRawStates {
    pub fn new() -> Self {
        KeyRawStates {
            keydowns: Vec::with_capacity(16),
            keyups: Vec::with_capacity(16),
        }
    }
}
