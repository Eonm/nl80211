use crate::attr::*;
use crate::nl80211traits::ParseNlAttr;
use crate::nl80211traits::PrettyFormat;
use crate::parse_attr::parse_u32;
use crate::parse_attr::parse_u64;
use crate::socket::Socket;
use crate::station::Station;
use neli::nlattr::AttrHandle;

use crate::parse_attr::{parse_hex, parse_string};

/// A struct representing a wifi interface
#[derive(Clone, Debug, PartialEq)]
pub struct Interface {
    /// A netlink interface index. This index is used to fetch extra information with nl80211
    pub index: Option<Vec<u8>>,
    /// Interface essid
    pub ssid: Option<Vec<u8>>,
    /// Interface MAC address
    pub mac: Option<Vec<u8>>,
    /// Interface name (u8, String)
    pub name: Option<Vec<u8>>,
    /// Interface frequency of the selected channel (u32, MHz)
    pub frequency: Option<Vec<u8>>,
    /// Interface chanel
    pub channel: Option<Vec<u8>>,
    /// Interface transmit power level in signed mBm units.
    pub power: Option<Vec<u8>>,
    /// index of wiphy to operate on, cf. /sys/class/ieee80211/<phyname>/index
    pub phy: Option<Vec<u8>>,
    /// Wireless device identifier, used for pseudo-devices that don't have a netdev (u64)
    pub device: Option<Vec<u8>>,
}

impl Interface {
    pub fn default() -> Interface {
        Interface {
            index: None,
            ssid: None,
            mac: None,
            name: None,
            frequency: None,
            channel: None,
            power: None,
            phy: None,
            device: None,
        }
    }

    /// Get station info for this interface
    pub fn get_station_info(&self) -> Result<Station, neli::err::NlError> {
        if let Some(index) = &self.index {
            Socket::connect()?.get_station_info(index)
        } else {
            Err(neli::err::NlError::new("Invalid interface index {:?}"))
        }
    }
}

impl ParseNlAttr for Interface {
    /// Parse netlink messages returned by the nl80211 command CmdGetInterface
    fn parse(&mut self, handle: AttrHandle<Nl80211Attr>) -> Interface {
        for attr in handle.iter() {
            match attr.nla_type {
                Nl80211Attr::AttrIfindex => {
                    self.index = Some(attr.payload.clone());
                }
                Nl80211Attr::AttrSsid => {
                    self.ssid = Some(attr.payload.clone());
                }
                Nl80211Attr::AttrMac => {
                    self.mac = Some(attr.payload.clone());
                }
                Nl80211Attr::AttrIfname => {
                    self.name = Some(attr.payload.clone());
                }
                Nl80211Attr::AttrWiphyFreq => self.frequency = Some(attr.payload.clone()),
                Nl80211Attr::AttrChannelWidth => self.channel = Some(attr.payload.clone()),
                Nl80211Attr::AttrWiphyTxPowerLevel => self.power = Some(attr.payload.clone()),
                Nl80211Attr::AttrWiphy => self.phy = Some(attr.payload.clone()),
                Nl80211Attr::AttrWdev => self.device = Some(attr.payload.clone()),
                _ => (),
            }
        }
        self.to_owned()
    }
}

impl PrettyFormat for Interface {
    fn pretty_format(&self) -> String {
        let mut result = Vec::new();

        if let Some(ssid) = &self.ssid {
            result.push(format!("essid : {}", parse_string(&ssid)))
        };

        if let Some(mac) = &self.mac {
            result.push(format!("mac : {}", parse_hex(&mac)))
        };

        if let Some(name) = &self.name {
            result.push(format!("interface : {}", parse_string(&name)))
        };

        if let Some(frequency) = &self.frequency {
            result.push(format!(
                "frequency : {} Ghz",
                parse_u32(frequency) as f64 / 1000.00
            ))
        };

        if let Some(chanel) = &self.channel {
            result.push(format!("channel : {}", parse_u32(chanel)))
        };

        if let Some(power) = &self.power {
            result.push(format!("power : {} dBm", parse_u32(power) / 100))
        };

        if let Some(phy) = &self.phy {
            result.push(format!("phy : {}", parse_u32(phy)))
        };

        if let Some(device) = &self.device {
            result.push(format!("device : {}", parse_u64(device)))
        };

        result.join("\n")
    }
}

#[cfg(test)]
mod test_interface {
    use super::*;
    use crate::attr::Nl80211Attr::*;
    use neli::nlattr::Nlattr;

    #[test]
    fn test_pretty_format() {
        let interface = Interface {
            index: Some(vec![3, 0, 0, 0]),
            ssid: Some(vec![101, 100, 117, 114, 111, 97, 109]),
            mac: Some(vec![255, 255, 255, 255, 255, 255]),
            name: Some(vec![119, 108, 112, 53, 115, 48]),
            frequency: Some(vec![108, 9, 0, 0]),
            channel: Some(vec![1, 0, 0, 0]),
            power: Some(vec![164, 6, 0, 0]),
            phy: Some(vec![0, 0, 0, 0]),
            device: Some(vec![1, 0, 0, 0, 0, 0, 0, 0]),
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
            interface.pretty_format(),
            expected_output.replace("\n        ", "\n")
        )
    }

    #[test]
    fn test_parser() {
        let handler = vec![
            Nlattr {
                nla_len: 8,
                nla_type: AttrIfindex,
                payload: vec![3, 0, 0, 0],
            },
            Nlattr {
                nla_len: 11,
                nla_type: AttrIfname,
                payload: vec![119, 108, 112, 53, 115, 48],
            },
            Nlattr {
                nla_len: 8,
                nla_type: AttrWiphy,
                payload: vec![0, 0, 0, 0],
            },
            Nlattr {
                nla_len: 8,
                nla_type: AttrIftype,
                payload: vec![2, 0, 0, 0],
            },
            Nlattr {
                nla_len: 12,
                nla_type: AttrWdev,
                payload: vec![1, 0, 0, 0, 0, 0, 0, 0],
            },
            Nlattr {
                nla_len: 10,
                nla_type: AttrMac,
                payload: vec![255, 255, 255, 255, 255, 255],
            },
            Nlattr {
                nla_len: 8,
                nla_type: AttrGeneration,
                payload: vec![5, 0, 0, 0],
            },
            Nlattr {
                nla_len: 5,
                nla_type: Attr4addr,
                payload: vec![0],
            },
            Nlattr {
                nla_len: 8,
                nla_type: AttrWiphyFreq,
                payload: vec![108, 9, 0, 0],
            },
            Nlattr {
                nla_len: 8,
                nla_type: AttrWiphyChannelType,
                payload: vec![1, 0, 0, 0],
            },
            Nlattr {
                nla_len: 8,
                nla_type: AttrChannelWidth,
                payload: vec![1, 0, 0, 0],
            },
            Nlattr {
                nla_len: 8,
                nla_type: AttrCenterFreq1,
                payload: vec![108, 9, 0, 0],
            },
            Nlattr {
                nla_len: 8,
                nla_type: AttrWiphyTxPowerLevel,
                payload: vec![164, 6, 0, 0],
            },
            Nlattr {
                nla_len: 12,
                nla_type: AttrSsid,
                payload: vec![101, 100, 117, 114, 111, 97, 109],
            },
        ];

        let interface = Interface::default().parse(neli::nlattr::AttrHandle::Owned(handler));

        let expected_interface = Interface {
            index: Some(vec![3, 0, 0, 0]),
            ssid: Some(vec![101, 100, 117, 114, 111, 97, 109]),
            mac: Some(vec![255, 255, 255, 255, 255, 255]),
            name: Some(vec![119, 108, 112, 53, 115, 48]),
            frequency: Some(vec![108, 9, 0, 0]),
            channel: Some(vec![1, 0, 0, 0]),
            power: Some(vec![164, 6, 0, 0]),
            phy: Some(vec![0, 0, 0, 0]),
            device: Some(vec![1, 0, 0, 0, 0, 0, 0, 0]),
        };

        assert_eq!(interface, expected_interface)
    }
}
