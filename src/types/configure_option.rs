use std::fmt::{Display, Formatter};

pub enum HidDeviceType {
    JoyRight1,
    JoyLeft2,
    FullKey3,
    JoyLeft4,
    JoyRight5,
    FullKey6,
    LarkHvcLeft,
    LarkHvcRight,
    LarkNesLeft,
    LarkNesRight,
    Lucia,
    Palma,
    FullKey13,
    FullKey15,
    DebugPad,
    System19,
    System20,
    System21,
    Lagon,
    Lager,
}

impl Display for HidDeviceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HidDeviceType::JoyRight1 => write!(f, "{}", 1),
            HidDeviceType::JoyLeft2 => write!(f, "{}", 2),
            HidDeviceType::FullKey3 => write!(f, "{}", 3),
            HidDeviceType::JoyLeft4 => write!(f, "{}", 4),
            HidDeviceType::JoyRight5 => write!(f, "{}", 5),
            HidDeviceType::FullKey6 => write!(f, "{}", 6),
            HidDeviceType::LarkHvcLeft => write!(f, "{}", 7),
            HidDeviceType::LarkHvcRight => write!(f, "{}", 8),
            HidDeviceType::LarkNesLeft => write!(f, "{}", 9),
            HidDeviceType::LarkNesRight => write!(f, "{}", 10),
            HidDeviceType::Lucia => write!(f, "{}", 11),
            HidDeviceType::Palma => write!(f, "{}", 12),
            HidDeviceType::FullKey13 => write!(f, "{}", 13),
            HidDeviceType::FullKey15 => write!(f, "{}", 15),
            HidDeviceType::DebugPad => write!(f, "{}", 17),
            HidDeviceType::System19 => write!(f, "{}", 19),
            HidDeviceType::System20 => write!(f, "{}", 20),
            HidDeviceType::System21 => write!(f, "{}", 21),
            HidDeviceType::Lagon => write!(f, "{}", 22),
            HidDeviceType::Lager => write!(f, "{}", 28),
        }
    }
}

pub enum ConfigureOption {
    MainLoopSleepTime(u64),
    ButtonClickSleepTime(u64),
    EchoCommands(bool),
    PrintDebugResultCodes(bool),
    KeySleepTime(u64),
    FingerDiameter(u32),
    PollRate(u64),
    FreezeRate(u64),
    ControllerType(HidDeviceType),
}

impl Display for ConfigureOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigureOption::MainLoopSleepTime(i) => write!(f, "mainLoopSleepTime {}", i),
            ConfigureOption::ButtonClickSleepTime(i) => write!(f, "buttonClickSleepTime {}", i),
            ConfigureOption::EchoCommands(b) => write!(f, "echoCommands {}", b),
            ConfigureOption::PrintDebugResultCodes(b) => write!(f, "printDebugResultCodes {}", b),
            ConfigureOption::KeySleepTime(i) => write!(f, "keySleepTime {}", i),
            ConfigureOption::FingerDiameter(i) => write!(f, "fingerDiameter {}", i),
            ConfigureOption::PollRate(i) => write!(f, "pollRate {}", i),
            ConfigureOption::FreezeRate(i) => write!(f, "freezeRate {}", i),
            ConfigureOption::ControllerType(dt) => write!(f, "controllerType {}", dt),
        }
    }
}
