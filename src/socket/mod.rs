use crate::bss::Bss;
use crate::station::Station;
use crate::nl80211traits::ParseNlAttr;
// use crate::station::parse_station;
use neli::consts::{NlFamily,NlmF,Nlmsg};
use neli::socket::NlSocket;
use neli::nl::Nlmsghdr;
use neli::genl::Genlmsghdr;
use crate::consts::{NL_80211_GENL_NAME, NL_80211_GENL_VERSION};
use crate::attr::Nl80211Attr;
use crate::cmd::Nl80211Cmd;
use crate::interface::Interface;
use neli::nlattr::Nlattr;

/// A generic netlink socket to send commands and receive messages
pub struct Socket {
    pub sock: NlSocket,
    pub family_id: u16,
}

impl Socket {
    /// Create a new nl80211 socket with netlink
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use nl80211::{Socket, Nl80211Attr, Nl80211Cmd, NL_80211_GENL_VERSION, PrettyFormat};
    /// # use neli::nlattr::Nlattr;
    /// # use neli::genl::Genlmsghdr;
    /// # use neli::nl::Nlmsghdr;
    /// # use neli::err::NlError;
    /// # use neli::consts::{NlFamily,NlmF,Nlmsg};
    ///
    /// # fn main() -> Result<(), neli::err::NlError> {
    ///     // Create a new nl80211 socket and use this socket to send nl80211 commands
    ///     let mut nl80211sock = Socket::connect()?;
    ///
    ///     let attrs: Vec<Nlattr<Nl80211Attr, Vec<u8>>> = vec![];
    ///
    ///     // Create a netlink message header with the command CMD_GET_INTERFACE and some payloads
    ///     let genlhdr = Genlmsghdr::new(Nl80211Cmd::CmdGetInterface, NL_80211_GENL_VERSION, attrs)?;
    ///
    ///     // Crate a netlink header
    ///     let nlhdr = {
    ///         let len = None;
    ///         let nl_type = nl80211sock.family_id;
    ///         let flags = vec![NlmF::Request, NlmF::Dump]; // Dump info
    ///         let seq = None;
    ///         let pid = None;
    ///         let payload = genlhdr;
    ///         Nlmsghdr::new(len, nl_type, flags, seq, pid, payload)
    ///     };
    ///
    ///     // Send netlink message
    ///     nl80211sock.sock.send_nl(nlhdr)?;
    ///
    ///     let mut iter = nl80211sock.sock.iter::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>();
    ///     while let Some(Ok(response)) = iter.next() {
    ///         match response.nl_type {
    ///            Nlmsg::Error => panic!("Error"),
    ///            Nlmsg::Done => break,
    ///            _ => {
    ///             // Parsing netlink messages here
    ///            }
    ///         }
    ///     }
    ///
    /// #   Ok(())
    /// # }
    /// ```
    pub fn connect() -> Result<Self, neli::err::NlError> {
        let family_id = {
            NlSocket::new(NlFamily::Generic, true)?
                .resolve_genl_family(NL_80211_GENL_NAME)?
        };

        let track_seq = true;
        let mut nl80211sock = NlSocket::new(NlFamily::Generic, track_seq)?;

        let pid = None;
        let groups = None;
        nl80211sock.bind(pid, groups)?;

        Ok(Self {
            sock: nl80211sock,
            family_id: family_id,
        })
    }

    /// Get information for all your wifi interfaces
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use nl80211::{Socket, PrettyFormat};
    ///
    /// # fn main() -> Result<(), neli::err::NlError>{
    ///     let wifi_interfaces = Socket::connect()?.get_interfaces_info();
    ///     for wifi_interface in wifi_interfaces? {
    ///         println!("{}", wifi_interface.pretty_format());
    ///     }
    /// #   Ok(())
    /// # }
    ///```
    pub fn get_interfaces_info(&mut self) -> Result<Vec<Interface>, neli::err::NlError> {
        let mut interfaces = Vec::new();
        let nl80211sock = &mut self.sock;

        let attrs: Vec<Nlattr<Nl80211Attr, Vec<u8>>> = vec![];
        let genlhdr = Genlmsghdr::new(Nl80211Cmd::CmdGetInterface, NL_80211_GENL_VERSION, attrs)?;

        let nlhdr = {
            let len = None;
            let nl_type = self.family_id;
            let flags = vec![NlmF::Request, NlmF::Dump];
            let seq = None;
            let pid = None;
            let payload = genlhdr;
            Nlmsghdr::new(len, nl_type, flags, seq, pid, payload)
        };

        nl80211sock.send_nl(nlhdr)?;

        let mut iter = nl80211sock.iter::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>();

        while let Some(Ok(response)) = iter.next() {
            match response.nl_type {
                Nlmsg::Error => panic!("Error"),
                Nlmsg::Done => break,
                _ => {
                    let handle = response.nl_payload.get_attr_handle();
                    interfaces.push(Interface::default().parse(handle));
                },
            };
        }

        Ok(interfaces)
    }

    /// Get access point information for a specific interface
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use nl80211::{Socket, PrettyFormat};
    ///
    /// # fn main() -> Result<(), neli::err::NlError>{
    ///   // First of all we need to get wifi interface information to get more data
    ///   let wifi_interfaces = Socket::connect()?.get_interfaces_info();
    ///   for wifi_interface in wifi_interfaces? {
    ///     if let Some(netlink_index) = wifi_interface.index {
    ///
    ///       // Then for each wifi interface we can fetch station information
    ///       let station_info = Socket::connect()?.get_station_info(&netlink_index.clone())?;
    ///           println!("{}", station_info.pretty_format());
    ///       }
    ///     }
    /// #   Ok(())
    /// # }
    ///```
    pub fn get_station_info(&mut self, interface_attr_if_index: &Vec<u8>) -> Result<Station, neli::err::NlError>  {
        let nl80211sock = &mut self.sock;

        let mut attrs: Vec<Nlattr<Nl80211Attr, Vec<u8>>> = vec![];
        let new_attr= Nlattr::new(None, Nl80211Attr::AttrIfindex, interface_attr_if_index.to_owned())?;
        attrs.push(new_attr);

        let genlhdr = Genlmsghdr::new(Nl80211Cmd::CmdGetStation, NL_80211_GENL_VERSION, attrs)?;
        let nlhdr = {
            let len = None;
            let nl_type = self.family_id;
            let flags = vec![NlmF::Request, NlmF::Dump];
            let seq = None;
            let pid = None;
            let payload = genlhdr;
            Nlmsghdr::new(len, nl_type, flags, seq, pid, payload)
        };

        nl80211sock.send_nl(nlhdr)?;

        let mut iter = nl80211sock.iter::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>();

        while let Some(Ok(response)) = iter.next() {
            match response.nl_type {
                    Nlmsg::Error => panic!("Error"),
                    Nlmsg::Done => break,
                    _ => {
                        let  handle = response.nl_payload.get_attr_handle();
                        return Ok(Station::default().parse(handle));
                    },
            };
        }
        return Ok(Station::default())
    }

    pub fn get_bss_info(&mut self, interface_attr_if_index: &Vec<u8>) -> Result<Bss, neli::err::NlError> {
        let nl80211sock = &mut self.sock;

        let mut attrs: Vec<Nlattr<Nl80211Attr, Vec<u8>>> = vec![];

        let new_attr= Nlattr::new(None, Nl80211Attr::AttrIfindex, interface_attr_if_index.to_owned())?;
        attrs.push(new_attr);

        let genlhdr = Genlmsghdr::new(Nl80211Cmd::CmdGetScan, NL_80211_GENL_VERSION, attrs)?;
        let nlhdr = {
            let len = None;
            let nl_type = self.family_id;
            let flags = vec![NlmF::Request, NlmF::Dump];
            let seq = None;
            let pid = None;
            let payload = genlhdr;
            Nlmsghdr::new(len, nl_type, flags, seq, pid, payload)
        };

        nl80211sock.send_nl(nlhdr)?;

        let mut iter = nl80211sock.iter::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>();

        while let Some(Ok(response)) = iter.next() {
            match response.nl_type {
                    Nlmsg::Error => panic!("Error"),
                    Nlmsg::Done => break,
                    _ => {
                        let  handle = response.nl_payload.get_attr_handle();
                        return Ok(Bss::default().parse(handle))
                    }
                }
            }
        Ok(Bss::default())
    }

    // pub fn scan(&mut self) -> Result<(), neli::err::NlError> {
    //     Ok(())
    // }
}
