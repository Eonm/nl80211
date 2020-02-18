//! Pretty print host interfaces.

use nl80211::Socket;
use nl80211::PrettyFormat;

fn main() -> Result<(), neli::err::NlError> {
    let interfaces = Socket::connect()?.get_interfaces_info()?;

    for interface in interfaces {
        println!("{}\n", interface.pretty_format());
    }

    Ok(())
}
