pub enum Register {
    DevName = 0x21,
    Status = 0x00,
    RepCap = 0x05,
    RepSoc = 0x06,
    VCell = 0x1A,
    Temp = 0x1B,
    Current = 0x1C,
    TimeToEmpty = 0x11,
    TimeToFull = 0x20,
    ProtStatus = 0xD9,
    ProtAlrt = 0xAF,
    CommStat = 0x61,
    Cell1 = 0xD8,
    Cell2 = 0xD7,
    Cell3 = 0xD6,
    Cell4 = 0xD5,
    Batt = 0xDA,
    Pckp = 0xDB,
    DieTemp = 0x34,
    Config = 0x0B,
    Config2 = 0xAB,
    VAlrtTh = 0x01,
    TAlrtTh = 0x02,
    SAlrtTh = 0x03,
    IAlrtTh = 0xAC,
    AgeForecast = 0xB9,
    Age = 0x07,
    Cycles = 0x17,
    RCell = 0x14,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RegisterNvm {
    NBattStatus = 0xA8,
    NPackCfg = 0xB5,
    NConfig = 0xB0,
    NVAlrtTh = 0x8C,
    NTAlrtTh = 0x8D,
    NSAlrtTh = 0x8F,
    NIAlrtTh = 0x8E,
}

/// All flags contained within the status register
pub enum StatusCode {
    /// Power-On Reset. This bit is set to a 1 when the device detects that
    /// a software or hardware POR event has occurred. This bit must be
    /// cleared by system software to detect the next POR event.
    /// POR is set to 1 at power-up.
    PowerOnReset = 0b0000_0000_0000_0010,
    /// Minimum Current Alert Threshold Exceeded. This bit is set to a 1
    /// whenever a Current register reading is below the minimum IAlrtTh
    /// value. This bit is cleared automatically when Current rises above
    /// minimum IAlrtTh value. Imn is set to 0 at power-up.
    MinCurrentExceeded = 0b0000_0000_0000_0100,
    /// Maximum Current Alert Threshold Exceeded. This bit is set to a 1
    /// whenever a Current register reading is above the maximum IAlrtTh value.
    /// This bit is cleared automatically when Current falls below maximum
    /// IAlrtTh value. Imx is set to 0 at power-up.
    MaxCurrentExceeded = 0b0000_0000_0100_0000,
    /// State-of-Charge 1% Change Alert. This is set to 1 whenever the RepSOC
    /// register crosses an integer percentage boundary such as 50.0%, 51.0%, etc.
    /// Must be cleared by host software. dSOCi is set to 0 at power-up.
    Soc1PercentChange = 0b0000_0000_1000_0000,
    /// Minimum Voltage Alert Threshold Exceeded. This bit is set to a 1 whenever
    /// a VCell register reading is below the minimum VAlrtTh value.
    /// This bit may or may not need to be cleared by system software to
    /// detect the next event. See Config.VS bit description.
    /// Vmn is set to 0 at power-up.
    MinVoltageExceeded = 0b0000_0001_0000_0000,
    /// Minimum Temperature Alert Threshold Exceeded. This bit is set to a 1 whenever
    /// a Temperature register reading is below the minimum TAlrtTh value.
    /// This bit may or may not need to be cleared by system software to
    /// detect the next event. See Config.TS bit description.
    /// Tmn is set to 0 at power-up.
    MinTemperatureExceeded = 0b0000_0010_0000_0000,
    /// Minimum SOC Alert Threshold Exceeded. This bit is set to a 1 whenever
    /// SOC falls below the minimum SAlrtTh value. This bit may or may not
    /// need to be cleared by system software to detect the next event.
    /// See Config.SS and MiscCFG.SACFG bit descriptions.
    /// Smn is set to 0 at power-up.
    MinSocExceeded = 0b0000_0100_0000_0000,
    /// Maximum Voltage Alert Threshold Exceeded. This bit is set to a 1 whenever
    /// a VCell register reading is above the maximum VAlrtTh value.
    /// This bit may or may not need to be cleared by system software to detect
    /// the next event. See Config.VS bit description. Vmx is set to 0 at power-up.
    MaxVoltageExceeded = 0b0001_0000_0000_0000,
    /// Maximum Temperature Alert Threshold Exceeded. This bit is set to a 1 whenever
    /// a Temperature register reading is above the maximum TAlrtTh value.
    /// This bit may or may not need to be cleared by system software to detect
    /// the next event. See Config.TS bit description. Tmx is set to 0 at power-up.
    MaxTemperatureExceeded = 0b0010_0000_0000_0000,
    /// Maximum SOC Alert Threshold Exceeded. This bit is set to a 1 whenever
    /// SOC rises above the maximum SAlrtTh value. This bit may or may not need to
    /// be cleared by system software to detect the next event. See Config.SS and
    /// MiscCFG.SACFG bit descriptions. Smx is set to 0 at power-up.
    MaxSocExceeded = 0b0100_0000_0000_0000,
    /// Protection Alert. This bit is set to a 1 when there is a protection event.
    /// The details of which protection event can be found in the ProtAlrts register.
    /// This bit must be cleared by system software to detect the next protection event.
    /// However, prior to clearing this bit, the ProtAlrts register must first be written
    /// to 0x0000. ProtAlrt is set to 0 at power-up.
    ProtectionAlert = 0b1000_0000_0000_0000,
}
/// All fault states of the protection state machine
pub enum ProtStatusCode {
    /// Flag to indicate ship state
    Ship = 0b0000_0000_0000_0001,
    /// Datasheet does not specify what this means
    ResDFault = 0b0000_0000_0000_0010,
    /// Overdischarge current (Discharging fault)
    OverdischargeCurrent = 0b0000_0000_0000_0100,
    /// Undervoltage (Discharging fault)
    Undervoltage = 0b0000_0000_0000_1000,
    /// Overtemperature for discharging (Discharging fault)
    OvertemperatureDischarging = 0b0000_0000_0001_0000,
    /// Overtemperature for die temperature (Discharging fault)
    OvertemperatureDie = 0b0000_0000_0010_0000,
    /// Permanent failure detected
    PermFail = 0b0000_0000_0100_0000,
    /// Muticell imbalance (Charging fault)
    MulticellImbalance = 0b0000_0000_1000_0000,
    /// Prequal timeout (Charging fault)
    PrequalTimeout = 0b0000_0001_0000_0000,
    /// Capacity overflow (Charging fault)
    CapacityOverflow = 0b0000_0010_0000_0000,
    /// Overcharge current (Charging fault)
    OverchargeCurrent = 0b0000_0100_0000_0000,
    /// Overvoltage (Charging fault)
    Overvoltage = 0b0000_1000_0000_0000,
    /// Undertemperature for charging (Charging fault)
    UndertemperatureCharging = 0b0001_0000_0000_0000,
    /// Full detection (Charging fault)
    Full = 0b0010_0000_0000_0000,
    /// Overtemperature for charging (Charging fault)
    OvertemperatureCharging = 0b0100_0000_0000_0000,
    /// Charge communication watchdog timer (Charging fault)
    ChargeWatchDogTimer = 0b1000_0000_0000_0000,
}

/// All fault states of the protection state machine
pub enum ProtAlertCode {
    /// This bit is set when a leakage detection fault has been detected.
    LeakageDetectionFault = 0b0000_0000_0000_0001,
    /// Datasheet does not specify what this means
    ResDFault = 0b0000_0000_0000_0010,
    /// Overdischarge current (Discharging fault)
    OverdischargeCurrent = 0b0000_0000_0000_0100,
    /// Undervoltage (Discharging fault)
    Undervoltage = 0b0000_0000_0000_1000,
    /// Overtemperature for discharging (Discharging fault)
    OvertemperatureDischarging = 0b0000_0000_0001_0000,
    /// Overtemperature for die temperature (Discharging fault)
    OvertemperatureDie = 0b0000_0000_0010_0000,
    /// Permanent failure detected
    PermFail = 0b0000_0000_0100_0000,
    /// Muticell imbalance (Charging fault)
    MulticellImbalance = 0b0000_0000_1000_0000,
    /// Prequal timeout (Charging fault)
    PrequalTimeout = 0b0000_0001_0000_0000,
    /// Capacity overflow (Charging fault)
    CapacityOverflow = 0b0000_0010_0000_0000,
    /// Overcharge current (Charging fault)
    OverchargeCurrent = 0b0000_0100_0000_0000,
    /// Overvoltage (Charging fault)
    Overvoltage = 0b0000_1000_0000_0000,
    /// Undertemperature for charging (Charging fault)
    UndertemperatureCharging = 0b0001_0000_0000_0000,
    /// Full detection (Charging fault)
    Full = 0b0010_0000_0000_0000,
    /// Overtemperature for charging (Charging fault)
    OvertemperatureCharging = 0b0100_0000_0000_0000,
    /// Charge communication watchdog timer (Charging fault)
    ChargeWatchDogTimer = 0b1000_0000_0000_0000,
}

pub enum CommStatCode {
    /// Set this bit to 1 to forcefully turn off DIS FET ignoring
    /// all other conditions if nProtCfg.CmOvrdEn is enabled.
    /// DIS FET remains off as long as this bit stays to 1. Clear to 0 for
    /// normal operation. Write Protection must be disabled before writing
    /// to the DISOff bit.
    DischargeOff = 1 << 9,
    /// Set this bit to 1 to forcefully turn off CHG FET ignoring all other
    /// conditions if nProtCfg.CmOvrdEn is enabled. CHG FET remains off as
    /// long as this bit stays set to 1. Clear to 0 for normal operation.
    /// Write Protection must be disabled before writing to the CHGOff bit.
    ChargeOff = 1 << 8,
    ///  Write protects register pages 1Dh
    WriteProtection5 = 1 << 7,
    ///  Write protects register pages 1Ch
    WriteProtection4 = 1 << 6,
    ///  Write protects register pages 18h, 19h
    WriteProtection3 = 1 << 5,
    ///  Write protects register pages 01h, 02h, 03h, 04h, 0Bh, 0Dh
    WriteProtection2 = 1 << 4,
    ///  Write protects register pages 1Ah, 1Bh, 1Eh
    WriteProtection1 = 1 << 3,
    /// This bit indicates the results of the previous SHA-256 or nonvolatile
    /// memory related command sent to the command register. This bit sets if
    /// there was an error executing the command or if the Full Reset command
    /// is executed. Once set, the bit must be cleared by system software in
    /// order to detect the next error. Write Protection must be disabled before
    /// the NVError bit can be cleared by the host.
    NonvolatileError = 1 << 2,
    /// This read only bit tracks if nonvolatile memory is busy or idle.
    /// NVBusy defaults to 0 after reset indicating nonvolatile memory is idle.
    /// This bit sets after a nonvolatile related command is sent to the command
    /// register, and clears automatically after the operation completes.
    NonvolatileBusy = 1 << 1,
    /// Write Protection Global Enable. Set to 1 to write protect all register pages.
    /// Clear to 0 to allow individual write protect bits (WP1–WP5) to be disabled.
    WriteProtectionGlobal = 1,
}

pub fn has_code(look_for: u16, within: u16) -> bool {
    (look_for & within) > 0
}

/// Set the kth bit (0 indexed) of n
pub(crate) fn set_bit(n: u16, k: u8) -> u16 {
    n | (1 << k)
}

/// Clear the kth bit (0 indexed) of n
pub(crate) fn clear_bit(n: u16, k: u8) -> u16 {
    n & !(1 << k)
}
