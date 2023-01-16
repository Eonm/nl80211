//! Pretty print host interfaces.

use nl80211::Nl80211Error;
use nl80211::Socket;

fn main() -> Result<(), Nl80211Error> {
    let interfaces = Socket::connect()?.get_interfaces_info()?;

    for interface in interfaces {
        println!("{}\n", interface);
    }

    Ok(())
}
