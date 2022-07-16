/// Type of thermistor
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ThermistorType {
    /// 10kΩ NTC thermistor
    Ntc10KOhm = 0,
    /// 100kΩ NTC thermistor
    Ntc100KOhm = 1 << 11,
}

/// Charge Pump Voltage Configuration.
/// Set according to the desired gate drive. Note that there is a trade-off in
/// quiescent vs. gate-drive.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChargePumpVoltageConfiguration {
    /// 6V setting
    Cp6V = 0,
    /// 8V setting
    Cp8V = 1 << 8,
    /// 10V setting
    Cp10V = 1 << 9,
}

/// Always-on Regulator Configuration.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AlwaysOnRegulatorConfiguration {
    /// ALDO is disabled.
    Disabled = 0,
    /// ALDO is enabled. Output is 3.4V.
    Enabled3p4V = 1 << 14,
    /// ALDO is enabled Output is 1.8V.
    Enabled1p8V = 1 << 15,
}

/// Enable Pckp and Batt Channels update.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BatteryPackUpdate {
    /// Pckp/Batt channels update every 22.4s
    UpdateEvery22p4s = 0,
    /// Pckp/Batt channels update after all cell measurements are completed
    AfterMeasurementsCompleted = 1 << 13,
}
