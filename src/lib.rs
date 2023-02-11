//! # Nl80211
//!
//! [![Coverage Status](https://coveralls.io/repos/github/Eonm/nl80211/badge.svg?branch=master)](https://coveralls.io/github/Eonm/nl80211?branch=master)
//! [![Build Status](https://travis-ci.org/Eonm/nl80211.svg?branch=master)](https://travis-ci.org/Eonm/nl80211)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/Eonm/markdown-packager/issues)
//!
//! [API documentation](https://docs.rs/nl80211/)
//!
//! This crate aims to provide a low level access to nl80211.
//!
//! This crate is inspired by the go [wifi](https://github.com/mdlayher/wifi) package made by @mdlayher.
//!
//! __This library only works on Linux.__
//!
//! __This crate is a work in progress__
//!
//! ## Built-in functions
//!
//! This crate has some built-in functions to fetch information and metrics of wifi interfaces and wireless networks.
//!
//! ### Get interface information
//!
//! ```no_run
//! extern crate nl80211;
//! extern crate neli;
//!
//! use nl80211::{Socket, Nl80211Error};
//!
//! fn main() -> Result<(), Nl80211Error> {
//!   let interfaces = Socket::connect()?.get_interfaces_info()?;
//!
//!   for interface in interfaces {
//!       println!("{:#?}", interface);
//!
//!       // Interface {
//!       //   index: Some([3, 0, 0, 0]),
//!       //   ssid: Some([101, 100, 117, 114, 111, 97, 109]),
//!       //   mac: Some([255, 255, 255, 255, 255, 255]),
//!       //   name: Some([119, 108, 112, 53, 115, 48]),
//!       //   frequency: Some([108, 9, 0, 0]),
//!       //   channel: Some([1, 0, 0, 0]),
//!       //   power: Some([164, 6, 0, 0]),
//!       //   phy: Some([0, 0, 0, 0]),
//!       //   device: Some([1, 0, 0, 0, 0, 0, 0, 0])
//!       // }
//!
//!       println!("{}", interface);
//!
//!       // essid : eduroam
//!       // mac : FF:FF:FF:FF:FF:FF
//!       // interface : wlp5s0
//!       // frequency : 2.412 Ghz
//!       // channel : 1
//!       // power : 17 dBm
//!       // phy : 0
//!       // device : 1
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Get Access Point information and metrics
//!
//! ```no_run
//! extern crate nl80211;
//! extern crate neli;
//!
//! use nl80211::{Socket, Nl80211Error};
//!
//! fn main() -> Result<(), Nl80211Error> {
//!   let interfaces = Socket::connect()?.get_interfaces_info()?;
//!   for interface in interfaces {
//!       let station = interface.get_station_info();
//!       println!("{}", station?);
//!
//!       // bssid : FF:FF:FF:FF:FF:FF
//!       // connected time : 35.816666 minutes
//!       // beacon loss : 0
//!       // signal : -60 dBm
//!       // average signal : -61 dBm
//!       // rx packets : 148983
//!       // tx packets : 46335
//!       // rx bitrate : 60 Mb/s
//!       // tx bitrate : 140 Mb/s
//!       // tx retries : 12578
//!       // tx failed : 2
//!   }
//!
//!   Ok(())
//! }
//! ```

mod cmd;
pub use cmd::*;
mod attr;
pub use attr::*;
mod socket;
pub use socket::Socket;
mod consts;
pub use consts::*;
mod interface;
pub use interface::*;
mod station;
pub use station::*;
mod bss;
pub use bss::*;
mod error;
mod types;
pub use error::*;

  
