use nl80211_derive::NlType;

/// Signal strength average (i8, dBm)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "average signal: ")]
#[fmt(cast="signal")]
pub struct AverageSignal(i8);

/// Count of times beacon loss was detected (u32)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "beacon loss: ", after = " dmbs")]
pub struct BeaconLoss(u32);

/// Time since the station is last connected in seconds (u32)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "connected time: ", after = " s")]
pub struct ConnectedTime(u32);

/// Reception bitrate (u8)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "reception bitrate: ")]
#[fmt(cast="bitrate")]
pub struct RxBitRate(u8);

/// Total received packets (MSDUs and MMPDUs) from this station (u32)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "totalt received packets: ")]
pub struct RxPackets(u32);

/// Signal strength of last received PPDU (u8, dBm)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "signal strength: ", after = "dBm")]
pub struct Signal(i8);

/// Transmission bitrate (u8)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "transmission bitrate: ")]
#[fmt(cast="bitrate")]
pub struct TxBitRate(u32);

/// Total failed packets (MPDUs) to this station (u32)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "total failed packets: ")]
pub struct TxFailed(u32);

/// Total transmitted packets (MSDUs and MMPDUs) to this station (u32)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "total transmited packets: ")]
pub struct TxPackets(u32);

/// Total retries (MPDUs) to this station (u32)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "total retries: ")]
pub struct TxRetries(u32);

#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "bssid: ")]
pub struct Bssid(String);

/// Frequency in MHz (u32)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "frequency: ", after = " MHz")]
pub struct Frequency(u32);

/// Beacon interval of the (I)BSS (u16)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "beacon interval :")]
pub struct BeaconInterval(u16);

/// Age of this BSS entry in ms (u32)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "seen ago: ", after = " ms")]
pub struct SeenMsAgo(u32);

/// Status, if this BSS is "used" (u8)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "status: ")]
pub struct Status(u8);

/// Interface essid
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "essid: ")]
pub struct Essid(String);

/// Interface name (u8, String)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "name: ")]
pub struct InterfaceName(String);

/// Interface transmit power level in signed mBm units.
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "power: ")]
pub struct Power(u32);

/// Interface MAC address
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "mac: ")]
pub struct Mac(String);

/// Interface chanel
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "chanel: ")]
pub struct Channel(u32);

/// index of wiphy to operate on, cf. /sys/class/ieee80211/<phyname>/index
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "phy: ")]
pub struct Phy(u32);

/// Wireless device identifier, used for pseudo-devices that don't have a netdev (u64)
#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "device: ")]
pub struct Device(u64);

#[derive(Debug, Clone, NlType, PartialEq)]
#[fmt(before = "name: ")]
pub struct Name(String);

pub fn signal(input: i8) -> f64 {
    input as f64 / 1000.00
}

pub fn power(input: u32) -> u64 {
    input / 100
}

pub fn bitrate(input: u8) -> u64 {
    input as u64 * 10
}
