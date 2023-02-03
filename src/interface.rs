use crate::attr::*;
use crate::socket::Socket;
use crate::station::Station;
use crate::types::*;
use getset::{Getters, CopyGetters};
use neli::nlattr::AttrHandle;
use std::convert::{TryFrom, TryInto};
use std::fmt;

/// A struct representing a wifi interface
#[derive(Clone, Debug, PartialEq, Default, Getters, CopyGetters)]
pub struct Interface {
    // A netlink interface index. This index is used to fetch extra information with nl80211
    #[getset(get = "pub")]
    index: Option<Vec<u8>>,
    #[getset(get = "pub")]
    ssid: Option<Essid>,
    #[getset(get = "pub")]
    mac: Option<Mac>,
    #[getset(get = "pub")]
    name: Option<Name>,
    #[getset(get = "pub")]
    frequency: Option<Frequency>,
    #[getset(get = "pub")]
    channel: Option<Channel>,
    #[getset(get = "pub")]
    power: Option<Power>,
    #[getset(get = "pub")]
    phy: Option<Phy>,
    #[getset(get = "pub")]
    device: Option<Device>,
}

impl Interface {
    /// Get station info for this interface
    pub fn get_station_info(&self) -> Result<Station, crate::error::Nl80211Error> {
        if let Some(index) = &self.index {
            Socket::connect()?.get_station_info(index)
        } else {
            Err(neli::err::NlError::new("Invalid interface index {:?}").into())
        }
    }
}

impl TryFrom<AttrHandle<'_, Nl80211Attr>> for Interface {
    type Error = crate::error::Nl80211Error;

    fn try_from(value: AttrHandle<Nl80211Attr>) -> Result<Self, Self::Error> {
        let mut interface = Interface::default();

        for attr in value.iter() {
            let payload = &attr.payload[..];

            match attr.nla_type {
                Nl80211Attr::AttrIfindex => {
                    interface.index = Some(attr.payload.clone());
                }
                Nl80211Attr::AttrSsid => {
                    interface.ssid = Some(payload.into());
                }
                Nl80211Attr::AttrMac => {
                    interface.mac = Some(payload.into());
                }
                Nl80211Attr::AttrIfname => {
                    interface.name = Some(payload.into());
                }
                Nl80211Attr::AttrWiphyFreq => interface.frequency = Some(payload.try_into()?),
                Nl80211Attr::AttrChannelWidth => interface.channel = Some(payload.try_into()?),
                Nl80211Attr::AttrWiphyTxPowerLevel => interface.power = Some(payload.try_into()?),
                Nl80211Attr::AttrWiphy => interface.phy = Some(payload.try_into()?),
                Nl80211Attr::AttrWdev => interface.device = Some(payload.try_into()?),
                _ => (),
            }
        }

        Ok(interface)
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();

        if let Some(ssid) = &self.ssid {
            result.push(format!("essid : {}", ssid))
        };

        if let Some(mac) = &self.mac {
            result.push(format!("mac : {}", mac))
        };

        if let Some(name) = &self.name {
            result.push(format!("interface : {}", name))
        };

        if let Some(frequency) = &self.frequency {
            result.push(format!("frequency : {} Ghz", frequency.0 as f64 / 1000.00))
        };

        if let Some(chanel) = &self.channel {
            result.push(format!("channel : {}", chanel))
        };

        if let Some(power) = &self.power {
            result.push(format!("power : {} dBm", power.0 / 100))
        };

        if let Some(phy) = &self.phy {
            result.push(format!("phy : {}", phy))
        };

        if let Some(device) = &self.device {
            result.push(format!("device : {}", device))
        };

        write!(f, "{}", result.join("\n"))
    }
}

#[cfg(test)]
mod test_interface {
    use super::*;

    #[test]
    fn test_pretty_format() {
        let interface = Interface {
            index: Some(vec![3, 0, 0, 0]),
            ssid: Some("eduroam".into()),
            mac: Some([255, 255, 255, 255, 255, 255].into()),
            name: Some("wlp5s0".into()),
            frequency: Some(2412.into()),
            channel: Some(1.into()),
            power: Some(1700.into()),
            phy: Some(0.into()),
            device: Some(1.into()),
        };

        let expected_output = r#"essid : eduroam
        mac : FF:FF:FF:FF:FF:FF
        interface : wlp5s0
        frequency : 2.412 Ghz
        channel : 1
        power : 17 dBm
        phy : 0
        device : 1"#;

        assert_eq!(
            interface.to_string(),
            expected_output.replace("\n        ", "\n")
        )
    }

    // #[test]
    // fn test_parser() {
    //     let handler = vec![
    //         Nlattr {
    //             nla_len: 8,
    //             nla_type: AttrIfindex,
    //             payload: vec![3, 0, 0, 0],
    //         },
    //         Nlattr {
    //             nla_len: 11,
    //             nla_type: AttrIfname,
    //             payload: vec![119, 108, 112, 53, 115, 48],
    //         },
    //         Nlattr {
    //             nla_len: 8,
    //             nla_type: AttrWiphy,
    //             payload: vec![0, 0, 0, 0],
    //         },
    //         Nlattr {
    //             nla_len: 8,
    //             nla_type: AttrIftype,
    //             payload: vec![2, 0, 0, 0],
    //         },
    //         Nlattr {
    //             nla_len: 12,
    //             nla_type: AttrWdev,
    //             payload: vec![1, 0, 0, 0, 0, 0, 0, 0],
    //         },
    //         Nlattr {
    //             nla_len: 10,
    //             nla_type: AttrMac,
    //             payload: vec![255, 255, 255, 255, 255, 255],
    //         },
    //         Nlattr {
    //             nla_len: 8,
    //             nla_type: AttrGeneration,
    //             payload: vec![5, 0, 0, 0],
    //         },
    //         Nlattr {
    //             nla_len: 5,
    //             nla_type: Attr4addr,
    //             payload: vec![0],
    //         },
    //         Nlattr {
    //             nla_len: 8,
    //             nla_type: AttrWiphyFreq,
    //             payload: vec![108, 9, 0, 0],
    //         },
    //         Nlattr {
    //             nla_len: 8,
    //             nla_type: AttrWiphyChannelType,
    //             payload: vec![1, 0, 0, 0],
    //         },
    //         Nlattr {
    //             nla_len: 8,
    //             nla_type: AttrChannelWidth,
    //             payload: vec![1, 0, 0, 0],
    //         },
    //         Nlattr {
    //             nla_len: 8,
    //             nla_type: AttrCenterFreq1,
    //             payload: vec![108, 9, 0, 0],
    //         },
    //         Nlattr {
    //             nla_len: 8,
    //             nla_type: AttrWiphyTxPowerLevel,
    //             payload: vec![164, 6, 0, 0],
    //         },
    //         Nlattr {
    //             nla_len: 12,
    //             nla_type: AttrSsid,
    //             payload: vec![101, 100, 117, 114, 111, 97, 109],
    //         },
    //     ];

    //     let interface : Interface = neli::nlattr::AttrHandle::Owned(handler).try_into().unwrap();

    //     let expected_interface = Interface {
    //         index: Some(vec![3, 0, 0, 0]),
    //         ssid: Some(vec![101, 100, 117, 114, 111, 97, 109]),
    //         mac: Some(vec![255, 255, 255, 255, 255, 255]),
    //         name: Some(vec![119, 108, 112, 53, 115, 48]),
    //         frequency: Some(vec![108, 9, 0, 0]),
    //         channel: Some(vec![1, 0, 0, 0]),
    //         power: Some(vec![164, 6, 0, 0]),
    //         phy: Some(vec![0, 0, 0, 0]),
    //         device: Some(vec![1, 0, 0, 0, 0, 0, 0, 0]),
    //     };

    //     assert_eq!(interface, expected_interface)
    // }
}
