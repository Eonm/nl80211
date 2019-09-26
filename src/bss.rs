use crate::attr::Nl80211Attr;
use crate::attr::Nl80211Bss;
use crate::nl80211traits::ParseNlAttr;
use crate::nl80211traits::PrettyFormat;
use crate::parse_attr::{parse_hex, parse_i32, parse_u16, parse_u32};
use neli::nlattr::AttrHandle;

/// A struct representing a BSS (Basic Service Set)
#[derive(Debug, Clone, PartialEq)]
pub struct Bss {
    pub bssid: Option<Vec<u8>>,
    /// Frequency in MHz (u32)
    pub frequency: Option<Vec<u8>>,
    /// Beacon interval of the (I)BSS (u16)
    pub beacon_interval: Option<Vec<u8>>,
    /// Age of this BSS entry in ms (u32)
    pub seen_ms_ago: Option<Vec<u8>>,
    /// Status, if this BSS is "used" (u8)
    pub status: Option<Vec<u8>>,
    /// Signal strength of probe response/beacon in mBm (100 * dBm) (i32)
    pub signal: Option<Vec<u8>>,
}

impl Bss {
    pub fn default() -> Bss {
        Bss {
            bssid: None,
            frequency: None,
            beacon_interval: None,
            seen_ms_ago: None,
            status: None,
            signal: None,
        }
    }
}

impl PrettyFormat for Bss {
    fn pretty_format(&self) -> String {
        let mut result = Vec::new();

        if let Some(bssid) = &self.bssid {
            result.push(format!("bssid : {}", parse_hex(bssid)))
        };

        if let Some(frequency) = &self.frequency {
            result.push(format!(
                "frequency : {} Ghz",
                parse_u32(frequency) as f32 / 1000.00
            ))
        };

        if let Some(beacon_interval) = &self.beacon_interval {
            result.push(format!(
                "beacon interval : {} TUs",
                parse_u16(beacon_interval)
            ))
        };

        if let Some(seen_ms_ago) = &self.seen_ms_ago {
            result.push(format!("last seen : {} ms", parse_u32(seen_ms_ago)))
        };

        if let Some(status) = &self.status {
            result.push(format!("status : {}", parse_u32(status)))
        };

        if let Some(signal) = &self.signal {
            result.push(format!(
                "signal : {:?} dBm",
                parse_i32(signal) as f32 / 100.00
            ))
        };

        result.join("\n")
    }
}

impl ParseNlAttr for Bss {
    /// Parse netlink messages returned by the nl80211 command CmdGetScan
    fn parse(&mut self, handle: AttrHandle<Nl80211Attr>) -> Bss {
        for attr in handle.iter() {
            println!("{:?}", attr);
            match attr.nla_type {
                Nl80211Attr::AttrBss => {
                    let sub_handle = attr.get_nested_attributes::<Nl80211Bss>().unwrap();
                    for sub_attr in sub_handle.iter() {
                        match sub_attr.nla_type {
                            Nl80211Bss::BssBeaconInterval => {
                                self.beacon_interval = Some(sub_attr.payload.clone())
                            }
                            Nl80211Bss::BssFrequency => {
                                self.frequency = Some(sub_attr.payload.clone())
                            }
                            Nl80211Bss::BssSeenMsAgo => {
                                self.seen_ms_ago = Some(sub_attr.payload.clone())
                            }
                            Nl80211Bss::BssStatus => self.status = Some(sub_attr.payload.clone()),
                            Nl80211Bss::BssBssid => self.bssid = Some(sub_attr.payload.clone()),
                            Nl80211Bss::BssSignalMbm => {
                                self.signal = Some(sub_attr.payload.clone())
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

#[cfg(test)]
mod test_bss {
    use super::*;
    use crate::attr::Nl80211Attr::*;
    use neli::nlattr::Nlattr;

    #[test]
    fn test_pretty_format() {
        let bss = Bss {
            bssid: Some(vec![255, 255, 255, 255, 255, 255]),
            frequency: Some(vec![108, 9, 0, 0]),
            beacon_interval: Some(vec![100, 0]),
            seen_ms_ago: Some(vec![100, 0, 0, 0]),
            status: Some(vec![1, 0, 0, 0]),
            signal: Some(vec![76, 235, 255, 255]),
        };

        let expected_output = r#"bssid : FF:FF:FF:FF:FF:FF
        frequency : 2.412 Ghz
        beacon interval : 100 TUs
        last seen : 100 ms
        status : 1
        signal : -53.0 dBm"#;

        assert_eq!(
            bss.pretty_format(),
            expected_output.replace("\n        ", "\n")
        )
    }

    #[test]
    fn test_parse() {
        let handler = vec![
            Nlattr {
                nla_len: 8,
                nla_type: AttrGeneration,
                payload: vec![28, 4, 0, 0],
            },
            Nlattr {
                nla_len: 8,
                nla_type: AttrIfindex,
                payload: vec![3, 0, 0, 0],
            },
            Nlattr {
                nla_len: 12,
                nla_type: AttrWdev,
                payload: vec![1, 0, 0, 0, 0, 0, 0, 0],
            },
            Nlattr {
                nla_len: 728,
                nla_type: AttrBss,
                payload: vec![
                    10, 0, 1, 0, 255, 255, 255, 255, 255, 255, 0, 0, 4, 0, 14, 0, 12, 0, 3, 0, 132,
                    12, 93, 163, 39, 0, 0, 0, 95, 1, 6, 0, 0, 8, 83, 70, 82, 45, 49, 99, 50, 56, 1,
                    8, 130, 132, 139, 150, 36, 48, 72, 108, 3, 1, 1, 7, 6, 68, 69, 32, 1, 13, 20,
                    32, 1, 0, 35, 2, 16, 0, 42, 1, 0, 50, 4, 12, 18, 24, 96, 48, 24, 1, 0, 0, 15,
                    172, 2, 2, 0, 0, 15, 172, 4, 0, 15, 172, 2, 1, 0, 0, 15, 172, 2, 12, 0, 11, 5,
                    1, 0, 80, 0, 0, 70, 5, 114, 8, 1, 0, 0, 45, 26, 188, 9, 27, 255, 255, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 22, 1, 8, 4, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 127, 8, 4, 0, 8, 0, 0, 0, 0,
                    64, 221, 131, 0, 80, 242, 4, 16, 74, 0, 1, 16, 16, 68, 0, 1, 2, 16, 59, 0, 1,
                    3, 16, 71, 0, 16, 65, 133, 194, 155, 156, 12, 135, 126, 154, 135, 125, 82, 84,
                    30, 42, 138, 16, 33, 0, 8, 83, 97, 103, 101, 109, 99, 111, 109, 16, 35, 0, 8,
                    83, 97, 103, 101, 109, 99, 111, 109, 16, 36, 0, 6, 49, 50, 51, 52, 53, 54, 16,
                    66, 0, 7, 48, 48, 48, 48, 48, 48, 49, 16, 84, 0, 8, 0, 6, 0, 80, 242, 4, 0, 1,
                    16, 17, 0, 10, 83, 97, 103, 101, 109, 99, 111, 109, 65, 80, 16, 8, 0, 2, 32, 8,
                    16, 60, 0, 1, 3, 16, 73, 0, 6, 0, 55, 42, 0, 1, 32, 221, 9, 0, 16, 24, 2, 1, 0,
                    12, 0, 0, 221, 26, 0, 80, 242, 1, 1, 0, 0, 80, 242, 2, 2, 0, 0, 80, 242, 4, 0,
                    80, 242, 2, 1, 0, 0, 80, 242, 2, 221, 24, 0, 80, 242, 2, 1, 1, 132, 0, 3, 164,
                    0, 0, 39, 164, 0, 0, 66, 67, 94, 0, 98, 50, 47, 0, 0, 12, 0, 13, 0, 187, 118,
                    116, 163, 39, 0, 0, 0, 19, 1, 11, 0, 0, 8, 83, 70, 82, 45, 49, 99, 50, 56, 1,
                    8, 130, 132, 139, 150, 36, 48, 72, 108, 3, 1, 1, 5, 4, 0, 1, 0, 0, 7, 6, 68,
                    69, 32, 1, 13, 20, 32, 1, 0, 35, 2, 16, 0, 42, 1, 0, 50, 4, 12, 18, 24, 96, 48,
                    24, 1, 0, 0, 15, 172, 2, 2, 0, 0, 15, 172, 4, 0, 15, 172, 2, 1, 0, 0, 15, 172,
                    2, 12, 0, 11, 5, 1, 0, 80, 0, 0, 70, 5, 114, 8, 1, 0, 0, 45, 26, 188, 9, 27,
                    255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61,
                    22, 1, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 127, 8,
                    4, 0, 8, 0, 0, 0, 0, 64, 221, 49, 0, 80, 242, 4, 16, 74, 0, 1, 16, 16, 68, 0,
                    1, 2, 16, 71, 0, 16, 65, 133, 194, 155, 156, 12, 135, 126, 154, 135, 125, 82,
                    84, 30, 42, 138, 16, 60, 0, 1, 3, 16, 73, 0, 6, 0, 55, 42, 0, 1, 32, 221, 9, 0,
                    16, 24, 2, 1, 0, 12, 0, 0, 221, 26, 0, 80, 242, 1, 1, 0, 0, 80, 242, 2, 2, 0,
                    0, 80, 242, 4, 0, 80, 242, 2, 1, 0, 0, 80, 242, 2, 221, 24, 0, 80, 242, 2, 1,
                    1, 132, 0, 3, 164, 0, 0, 39, 164, 0, 0, 66, 67, 94, 0, 98, 50, 47, 0, 0, 6, 0,
                    4, 0, 100, 0, 0, 0, 6, 0, 5, 0, 17, 21, 0, 0, 8, 0, 2, 0, 108, 9, 0, 0, 8, 0,
                    12, 0, 0, 0, 0, 0, 8, 0, 10, 0, 100, 0, 0, 0, 8, 0, 7, 0, 76, 235, 255, 255, 8,
                    0, 9, 0, 1, 0, 0, 0,
                ],
            },
        ];

        let bss = Bss::default().parse(neli::nlattr::AttrHandle::Owned(handler));
        let expected_bss = Bss {
            bssid: Some(vec![255, 255, 255, 255, 255, 255]),
            frequency: Some(vec![108, 9, 0, 0]),
            beacon_interval: Some(vec![100, 0]),
            seen_ms_ago: Some(vec![100, 0, 0, 0]),
            status: Some(vec![1, 0, 0, 0]),
            signal: Some(vec![76, 235, 255, 255]),
        };

        assert_eq!(bss, expected_bss)
    }
}
