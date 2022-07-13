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
    DieTemp = 0x34,
}

pub enum RegisterNvm {
    NBattStatus = 0xA8,
    NPackCfg = 0xB5,
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

impl StatusCode {
    pub fn has_status(look_for: StatusCode, within: u16) -> bool {
        (look_for as u16 & within) > 0
    }
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
impl ProtStatusCode {
    pub fn has_status(look_for: ProtStatusCode, within: u16) -> bool {
        (look_for as u16 & within) > 0
    }
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
impl ProtAlertCode {
    pub fn has_status(look_for: ProtStatusCode, within: u16) -> bool {
        (look_for as u16 & within) > 0
    }
}
