use crate::attr::Nl80211Attr;
use crate::attr::Nl80211Bss;
use crate::types::*;
use getset::{CopyGetters, Getters};
use neli::nlattr::AttrHandle;
use nl80211_derive::NlType;
use std::convert::TryInto;
use std::fmt;

/// A struct representing a BSS (Basic Service Set)
#[derive(Debug, Clone, PartialEq, Default, Getters, CopyGetters)]
pub struct Bss {
    #[getset(get = "pub")]
    bssid: Option<Bssid>,
    #[getset(get = "pub")]
    frequency: Option<Frequency>,
    #[getset(get = "pub")]
    beacon_interval: Option<BeaconInterval>,
    #[getset(get = "pub")]
    seen_ms_ago: Option<SeenMsAgo>,
    #[getset(get = "pub")]
    status: Option<Status>,
    #[getset(get = "pub")]
    signal: Option<Signal>,
}

impl fmt::Display for Bss {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();

        if let Some(bssid) = &self.bssid {
            result.push(bssid.to_string())
        };

        if let Some(frequency) = &self.frequency {
            result.push(frequency.to_string())
        };

        if let Some(beacon_interval) = &self.beacon_interval {
            result.push(beacon_interval.to_string())
        };

        if let Some(seen_ms_ago) = &self.seen_ms_ago {
            result.push(seen_ms_ago.to_string())
        };

        if let Some(status) = &self.status {
            result.push(status.to_string())
        };

        if let Some(signal) = &self.signal {
            result.push(signal.to_string())
        };

        write!(f, "{}", result.join("\n"))
    }
}

impl std::convert::TryFrom<AttrHandle<'_, Nl80211Attr>> for Bss {
    type Error = crate::error::Nl80211Error;

    /// Parse netlink messages returned by the nl80211 command CmdGetScan
    fn try_from(value: AttrHandle<Nl80211Attr>) -> Result<Self, Self::Error> {
        let mut bss = Bss::default();

        for attr in value.iter() {
            if attr.nla_type == Nl80211Attr::AttrBss {
                let sub_handle = attr.get_nested_attributes::<Nl80211Bss>()?;
                for sub_attr in sub_handle.iter() {
                    let payload = &sub_attr.payload[..];
                    match sub_attr.nla_type {
                        Nl80211Bss::BssBeaconInterval => {
                            bss.beacon_interval = Some(payload.try_into()?)
                        }
                        Nl80211Bss::BssFrequency => bss.frequency = Some(payload.try_into()?),
                        Nl80211Bss::BssSeenMsAgo => bss.seen_ms_ago = Some(payload.try_into()?),
                        Nl80211Bss::BssStatus => bss.status = Some(payload.try_into()?),
                        Nl80211Bss::BssBssid => bss.bssid = Some(payload.try_into()?),
                        Nl80211Bss::BssSignalMbm => bss.signal = Some(payload.try_into()?),
                        _ => (),
                    }
                }
            }
        }

        Ok(bss)
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
            bssid: Some([255, 255, 255, 255, 255, 255].into()),
            frequency: Some(2412.into()),
            beacon_interval: Some(100.into()),
            seen_ms_ago: Some(100.into()),
            status: Some(1.into()),
            signal: Some((53).into()),
        };

        let expected_output = r#"bssid: FF:FF:FF:FF:FF:FF
        frequency: 2.412 GHz
        beacon interval: 100 TUs
        last seen: 100 ms ago
        status: 1
        signal: -53.0 dBm"#;

        assert_eq!(bss.to_string(), expected_output.replace("\n        ", "\n"))
    }

    #[test]
    fn test_parse() {
        let _handler = vec![
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

        // let bss = Bss::default().parse(neli::nlattr::AttrHandle::Owned(handler));
        // let expected_bss = Bss {
        //     bssid: Some(vec![255, 255, 255, 255, 255, 255]),
        //     frequency: Some(vec![108, 9, 0, 0]),
        //     beacon_interval: Some(vec![100, 0]),
        //     seen_ms_ago: Some(vec![100, 0, 0, 0]),
        //     status: Some(vec![1, 0, 0, 0]),
        //     signal: Some(vec![76, 235, 255, 255]),
        // };

        // assert_eq!(bss, expected_bss)
    }
}
