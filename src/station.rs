use crate::attr::{Nl80211Attr, Nl80211StaInfo};
use crate::nl80211traits::*;
use crate::parse_attr::{parse_hex, parse_i8, parse_u32, parse_u8};
use neli::nlattr::AttrHandle;

/// A struct representing a remote station (Access Point)
#[derive(Clone, Debug, PartialEq)]
pub struct Station {
    /// Signal strength average (i8, dBm)
    pub average_signal: Option<Vec<u8>>,
    /// Count of times beacon loss was detected (u32)
    pub beacon_loss: Option<Vec<u8>>,
    /// Station bssid (u8)
    pub bssid: Option<Vec<u8>>,
    /// Time since the station is last connected in seconds (u32)
    pub connected_time: Option<Vec<u8>>,
    /// Reception bitrate (u8)
    pub rx_bitrate: Option<Vec<u8>>,
    /// Total received packets (MSDUs and MMPDUs) from this station (u32)
    pub rx_packets: Option<Vec<u8>>,
    /// Signal strength of last received PPDU (u8, dBm)
    pub signal: Option<Vec<u8>>,
    /// Transmission bitrate (u8)
    pub tx_bitrate: Option<Vec<u8>>,
    /// Total failed packets (MPDUs) to this station (u32)
    pub tx_failed: Option<Vec<u8>>,
    /// Total transmitted packets (MSDUs and MMPDUs) to this station (u32)
    pub tx_packets: Option<Vec<u8>>,
    /// Total retries (MPDUs) to this station (u32)
    pub tx_retries: Option<Vec<u8>>,
}

impl Station {
    pub fn default() -> Station {
        Station {
            bssid: None,
            connected_time: None,
            beacon_loss: None,
            signal: None,
            average_signal: None,
            rx_packets: None,
            tx_packets: None,
            rx_bitrate: None,
            tx_bitrate: None,
            tx_retries: None,
            tx_failed: None,
        }
    }
}

impl ParseNlAttr for Station {
    /// Parse netlink messages returned by the nl80211 command CmdGetStation
    fn parse(&mut self, handle: AttrHandle<Nl80211Attr>) -> Station {
        for attr in handle.iter() {
            match attr.nla_type {
                Nl80211Attr::AttrMac => {
                    self.bssid = Some(attr.payload.clone());
                }
                Nl80211Attr::AttrStaInfo => {
                    let sub_handle = attr.get_nested_attributes::<Nl80211StaInfo>().unwrap();
                    for sub_attr in sub_handle.iter() {
                        match sub_attr.nla_type {
                            Nl80211StaInfo::StaInfoSignal => {
                                self.signal = Some(sub_attr.payload.clone())
                            }
                            Nl80211StaInfo::StaInfoSignalAvg => {
                                self.average_signal = Some(sub_attr.payload.clone())
                            }
                            Nl80211StaInfo::StaInfoBeaconLoss => {
                                self.beacon_loss = Some(sub_attr.payload.clone())
                            }
                            Nl80211StaInfo::StaInfoConnectedTime => {
                                self.connected_time = Some(sub_attr.payload.clone())
                            }
                            Nl80211StaInfo::StaInfoRxPackets => {
                                self.rx_packets = Some(sub_attr.payload.clone())
                            }
                            Nl80211StaInfo::StaInfoTxPackets => {
                                self.tx_packets = Some(sub_attr.payload.clone())
                            }
                            Nl80211StaInfo::StaInfoTxRetries => {
                                self.tx_retries = Some(sub_attr.payload.clone())
                            }
                            Nl80211StaInfo::StaInfoTxFailed => {
                                self.tx_failed = Some(sub_attr.payload.clone())
                            }
                            Nl80211StaInfo::StaInfoRxBitrate => {
                                let bit_rate_handle =
                                    sub_attr.get_nested_attributes::<Nl80211StaInfo>().unwrap();
                                for sub_sub_attr in bit_rate_handle.iter() {
                                    match sub_sub_attr.nla_type {
                                        Nl80211StaInfo::StaInfoRxBytes => {
                                            self.rx_bitrate = Some(sub_sub_attr.payload.clone())
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            Nl80211StaInfo::StaInfoTxBitrate => {
                                let bit_rate_handle =
                                    sub_attr.get_nested_attributes::<Nl80211StaInfo>().unwrap();
                                for sub_sub_attr in bit_rate_handle.iter() {
                                    match sub_sub_attr.nla_type {
                                        Nl80211StaInfo::StaInfoRxBytes => {
                                            self.tx_bitrate = Some(sub_sub_attr.payload.clone())
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
        self.to_owned()
    }
}

impl PrettyFormat for Station {
    fn pretty_format(&self) -> String {
        let mut result = Vec::new();

        if let Some(bssid) = &self.bssid {
            result.push(format!("bssid : {}", parse_hex(bssid)))
        };

        if let Some(connected_time) = &self.connected_time {
            result.push(format!(
                "connected time : {} minutes",
                parse_u32(connected_time) as f32 / 60.0
            ))
        };

        if let Some(beacon_loss) = &self.beacon_loss {
            result.push(format!("beacon loss : {}", parse_u32(beacon_loss)))
        };

        if let Some(signal) = &self.signal {
            result.push(format!("signal : {} dBm", parse_i8(signal)))
        };

        if let Some(average_signal) = &self.average_signal {
            result.push(format!("average signal : {} dBm", parse_i8(average_signal)))
        };

        if let Some(rx_packets) = &self.rx_packets {
            result.push(format!("rx packets : {}", parse_u32(rx_packets)))
        };

        if let Some(tx_packets) = &self.tx_packets {
            result.push(format!("tx packets : {}", parse_u32(tx_packets)))
        };

        if let Some(rx_bitrate) = &self.rx_bitrate {
            result.push(format!("rx bitrate : {} Mb/s", parse_u8(rx_bitrate) * 10))
        };

        if let Some(tx_bitrate) = &self.tx_bitrate {
            result.push(format!("tx bitrate : {} Mb/s", parse_u8(tx_bitrate) * 10))
        }

        if let Some(tx_retries) = &self.tx_retries {
            result.push(format!("tx retries : {}", parse_u32(tx_retries)))
        }

        if let Some(tx_failed) = &self.tx_failed {
            result.push(format!("tx failed : {}", parse_u32(tx_failed)))
        }

        result.join("\n")
    }
}

#[cfg(test)]
mod tests_station {
    use super::*;
    use crate::attr::Nl80211Attr::AttrMac;
    use crate::attr::Nl80211Attr::AttrStaInfo;
    use neli::nlattr::Nlattr;

    #[test]
    fn test_pretty_format() {
        let station = Station {
            average_signal: Some(vec![197]),
            beacon_loss: Some(vec![0, 0, 0, 0]),
            bssid: Some(vec![255, 255, 255, 255, 255, 255]),
            connected_time: Some(vec![118, 21, 0, 0]),
            rx_bitrate: Some(vec![12]),
            rx_packets: Some(vec![108, 126, 6, 0]),
            signal: Some(vec![195]),
            tx_bitrate: Some(vec![13]),
            tx_failed: Some(vec![45, 0, 0, 0]),
            tx_packets: Some(vec![14, 89, 2, 0]),
            tx_retries: Some(vec![9, 111, 0, 0]),
        };

        let expected_output = r#"bssid : FF:FF:FF:FF:FF:FF
        connected time : 91.566666 minutes
        beacon loss : 0
        signal : -61 dBm
        average signal : -59 dBm
        rx packets : 425580
        tx packets : 153870
        rx bitrate : 120 Mb/s
        tx bitrate : 130 Mb/s
        tx retries : 28425
        tx failed : 45"#;

        assert_eq!(
            station.pretty_format(),
            expected_output.replace("\n        ", "\n")
        )
    }

    #[test]
    fn test_parser() {
        let handler = vec![
            Nlattr {
                nla_len: 10,
                nla_type: AttrMac,
                payload: vec![46, 46, 46, 46, 46, 46],
            },
            Nlattr {
                nla_len: 2404,
                nla_type: AttrStaInfo,
                payload: vec![
                    8, 0, 16, 0, 17, 27, 0, 0, 8, 0, 1, 0, 248, 2, 0, 0, 8, 0, 2, 0, 43, 98, 156,
                    29, 8, 0, 3, 0, 99, 123, 109, 1, 12, 0, 23, 0, 43, 98, 156, 29, 0, 0, 0, 0, 12,
                    0, 24, 0, 99, 123, 109, 1, 0, 0, 0, 0, 5, 0, 7, 0, 218, 0, 0, 0, 5, 0, 13, 0,
                    215, 0, 0, 0, 20, 0, 25, 0, 5, 0, 0, 0, 216, 0, 0, 0, 5, 0, 1, 0, 213, 0, 0, 0,
                    20, 0, 26, 0, 5, 0, 0, 0, 212, 0, 0, 0, 5, 0, 1, 0, 211, 0, 0, 0, 28, 0, 8, 0,
                    8, 0, 5, 0, 16, 4, 0, 0, 6, 0, 1, 0, 16, 4, 0, 0, 5, 0, 2, 0, 13, 0, 0, 0, 28,
                    0, 14, 0, 8, 0, 5, 0, 134, 1, 0, 0, 6, 0, 1, 0, 134, 1, 0, 0, 5, 0, 2, 0, 4, 0,
                    0, 0, 8, 0, 9, 0, 226, 128, 7, 0, 8, 0, 10, 0, 9, 170, 2, 0, 8, 0, 11, 0, 27,
                    130, 0, 0, 8, 0, 12, 0, 47, 0, 0, 0, 8, 0, 27, 0, 196, 160, 0, 0, 8, 0, 18, 0,
                    0, 0, 0, 0, 28, 0, 15, 0, 4, 0, 2, 0, 4, 0, 3, 0, 5, 0, 4, 0, 1, 0, 0, 0, 6, 0,
                    5, 0, 100, 0, 0, 0, 12, 0, 17, 0, 254, 0, 0, 0, 170, 0, 0, 0, 12, 0, 28, 0,
                    183, 3, 0, 0, 0, 0, 0, 0, 12, 0, 29, 0, 225, 254, 0, 0, 0, 0, 0, 0, 5, 0, 30,
                    0, 216, 0, 0, 0, 5, 0, 34, 0, 46, 0, 0, 0, 56, 8, 31, 0, 128, 0, 1, 0, 12, 0,
                    1, 0, 168, 103, 5, 0, 0, 0, 0, 0, 12, 0, 2, 0, 71, 169, 2, 0, 0, 0, 0, 0, 12,
                    0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76, 0, 6,
                    0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 61, 39, 1, 0, 8,
                    0, 4, 0, 23, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0, 0, 8, 0, 8,
                    0, 0, 0, 0, 0, 8, 0, 9, 0, 38, 56, 109, 1, 8, 0, 10, 0, 71, 169, 2, 0, 128, 0,
                    2, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76,
                    0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0, 0, 0,
                    0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0, 0, 8,
                    0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0, 128, 0,
                    3, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76,
                    0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0, 0, 0,
                    0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0, 0, 8,
                    0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0, 128, 0,
                    4, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76,
                    0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0, 0, 0,
                    0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0, 0, 8,
                    0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0, 128, 0,
                    5, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76,
                    0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0, 0, 0,
                    0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0, 0, 8,
                    0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0, 128, 0,
                    6, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76,
                    0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0, 0, 0,
                    0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0, 0, 8,
                    0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0, 128, 0,
                    7, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 180, 0, 0, 0, 0, 0, 0,
                    0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 180,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 115, 64, 0, 0, 8, 0, 10, 0, 180, 0, 0,
                    0, 128, 0, 8, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 2, 0, 0, 0,
                    0, 0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0,
                    2, 0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0,
                    0, 0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 32, 1, 0, 0, 8, 0, 10, 0, 2, 0, 0, 0,
                    128, 0, 9, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 10, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 11, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 12, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 13, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 14, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 15, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 16, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0, 52,
                    0, 17, 0, 12, 0, 1, 0, 109, 25, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 4, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0,
                ],
            },
        ];

        let station = Station::default().parse(neli::nlattr::AttrHandle::Owned(handler));
        let expected_station = Station {
            average_signal: Some(vec![215]),
            beacon_loss: Some(vec![0, 0, 0, 0]),
            bssid: Some(vec![46, 46, 46, 46, 46, 46]),
            connected_time: Some(vec![17, 27, 0, 0]),
            rx_bitrate: Some(vec![4]),
            rx_packets: Some(vec![226, 128, 7, 0]),
            signal: Some(vec![218]),
            tx_bitrate: Some(vec![13]),
            tx_failed: Some(vec![47, 0, 0, 0]),
            tx_packets: Some(vec![9, 170, 2, 0]),
            tx_retries: Some(vec![27, 130, 0, 0]),
        };

        assert_eq!(station, expected_station)
    }
}
