use thiserror::Error;

#[derive(Error, Debug)]
pub enum Nl80211Error {
    #[error("failed to parse netlink output")]
    FailedToReadBytes(#[from] std::array::TryFromSliceError),
    #[error("failed to get netlink nested attributes")]
    NetlinkError(#[from] neli::err::NlError),
    #[error("netlink deserialization error")]
    NetlinkDeserializationError(#[from] neli::err::DeError),
    #[error("netlink serialization error")]
    NetlinkSerializationError(#[from] neli::err::SerError),
}
