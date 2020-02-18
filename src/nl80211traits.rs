use crate::attr::Nl80211Attr;
use neli::nlattr::AttrHandle;

/// Parse netlink messages attributes returned by a nl80211 command
pub trait ParseNlAttr {
    fn parse(&mut self, handle: AttrHandle<Nl80211Attr>) -> Self;
}

/// Decode netlink payloads (Vec\<u8\>) to appropriate types
pub trait NlPayloadDecode {
    fn decode(&mut self) -> Self;
}
