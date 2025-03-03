mod llio_api;
pub use llio_api::*;
mod i2c_api;
pub use i2c_api::*;
mod rtc_api;
pub use rtc_api::*;

// ///////////////////// UART TYPE
#[allow(dead_code)]  // we use this constant, but only in the `bin` view (not `lib`), so clippy complains, but this seems more discoverable here.
pub(crate) const BOOT_UART: u32 = UartType::Log as u32;

#[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, PartialEq, Eq)]
pub enum UartType {
    Kernel = 0,
    Log = 1,
    Application = 2,
    Invalid = 3,
}
// from/to for Xous messages
impl From<usize> for UartType {
    fn from(code: usize) -> Self {
        match code {
            0 => UartType::Kernel,
            1 => UartType::Log,
            2 => UartType::Application,
            _ => UartType::Invalid,
        }
    }
}
impl Into<usize> for UartType {
    fn into(self) -> usize {
        match self {
            UartType::Kernel => 0,
            UartType::Log => 1,
            UartType::Application => 2,
            UartType::Invalid => 3,
        }
    }
}
// for the actual bitmask going to hardware
impl Into<u32> for UartType {
    fn into(self) -> u32 {
        match self {
            UartType::Kernel => 0,
            UartType::Log => 1,
            UartType::Application => 2,
            UartType::Invalid => 3,
        }
    }
}

// ////////////////////////////////// OPCODES
#[derive(Debug, num_derive::FromPrimitive, num_derive::ToPrimitive)]
pub(crate) enum Opcode {
    /// not tested - set CRG parameters
    CrgMode, //(ClockMode),

    /// not tested -- set GPIO
    GpioDataOut, //(u32),
    GpioDataIn,
    GpioDataDrive, //(u32),
    GpioIntMask, //(u32),
    GpioIntAsFalling, //(u32),
    GpioIntPending,
    GpioIntEna, //(u32),
    GpioIntSubscribe, //(String<64>), //
    GpioIntHappened,

    /// set UART mux
    UartMux, //(UartType),

    // InfoLitexId, //(String<64>), // TODO: returns the ASCII string baked into the FPGA that describes the FPGA build, inside Registration
    InfoDna,
    InfoGit,
    InfoPlatform,
    InfoTarget,

    /// partially tested -- power
    PowerAudio, //(bool),
    PowerSelf, //(bool), // setting this to false allows the EC to turn off our power
    PowerBoostMode, //(bool),
    PowerCrypto,
    PowerCryptoStatus,
    WfiOverride,
    DebugPowerdown,
    GetActivity,
    DebugWakeup,
    EcSnoopAllow, //(bool),
    EcReset,
    EcPowerOn,
    SelfDestruct, //(u32), // requires a series of writes to enable

    /// vibe motor
    Vibe, //(VibePattern),

    /// not tested -- xadc
    AdcVbus,
    AdcVccInt,
    AdcVccAux,
    AdcVccBram,
    AdcUsbN,
    AdcUsbP,
    AdcTemperature,
    AdcGpio5,
    AdcGpio2,

    /// partially tested -- events
    EventComSubscribe, //(String<64>),
    EventRtcSubscribe, //(String<64>),
    EventUsbAttachSubscribe, //(String<64>),
    EventComEnable, //(bool),
    EventRtcEnable, //(bool),
    EventUsbAttachEnable, //(bool),
    EventActivityHappened,

    /// internal from handler to main loop
    EventComHappened,
    EventRtcHappened,
    EventUsbHappened,

    /// debug USB status
    DebugUsbOp,

    /// SuspendResume callback
    SuspendResume,

    /// Exit the server
    Quit,
}

#[derive(Debug, num_derive::FromPrimitive, num_derive::ToPrimitive)]
pub(crate) enum EventCallback {
    Event,
    Drop,
}

/*
Implementation note: we use a ScalarHook with a two-stage message passing so we don't leak
the local SID. The local SID should be considered a secret, and not shared. A two-stage
message passing system creates a dedicated, one-time use server and shares this SID with
the LLIO server, thus protecting the local SID from disclosure.
*/
#[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Copy, Clone)]
pub(crate) struct ScalarHook {
    pub sid: (u32, u32, u32, u32),
    pub id: u32,  // ID of the scalar message to send through (e.g. the discriminant of the Enum on the caller's side API)
    pub cid: xous::CID,   // caller-side connection ID for the scalar message to route to. Created by the caller before hooking.
}

