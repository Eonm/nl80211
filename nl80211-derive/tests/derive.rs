use nl80211_derive::NlType;

#[derive(NlType)]
pub struct Cast(u32);

#[test]
fn test_before() {
    #[derive(NlType)]
    #[fmt(before = "before: ")]
    pub struct Before(u32);

    assert_eq!(Before(10).to_string(), "before: 10");
}

#[test]
fn test_after() {
    #[derive(NlType)]
    #[fmt(after = " after")]
    pub struct After(u32);

    assert_eq!(After(10).to_string(), "10 after");
}

#[test]
fn test_before_and_after() {
    #[derive(NlType)]
    #[fmt(before = "before: ", after = " after")]
    pub struct BeforeAndAfter(u32);

    assert_eq!(BeforeAndAfter(10).to_string(), "before: 10 after");
}

#[test]
fn test_from_vec_for_u8() {
    #[derive(NlType)]
    pub struct FromVecU8(u8);

    let vec = vec![0];
    FromVecU8::try_from(&vec[..]).unwrap();
}

#[test]
fn test_from_vec_for_u32() {
    #[derive(NlType)]
    pub struct FromVecU32(u32);

    let vec = vec![0, 0, 0, 0];

    FromVecU32::try_from(&vec[..]).unwrap();
}

#[test]
fn test_from_vec_for_u64() {
    #[derive(NlType)]
    pub struct FromVecU64(u64);

    let vec = vec![0, 0, 0, 0, 0, 0, 0, 0];

    FromVecU64::try_from(&vec[..]).unwrap();
}

#[test]
fn test_from_string() {
    #[derive(NlType)]
    pub struct FromString(String);

    FromString::from("e");
    FromString::from("e".to_string());
}

#[test]
fn test_cast() {
    fn double(i: u32) -> u32 {
        i * 2
    }

    #[derive(NlType)]
    #[fmt(cast = "double")]
    pub struct Double(u32);

    assert_eq!(Double(2).to_string(), "4")
}

#[test]
fn test_formatter() {
    #[derive(NlType)]
    #[fmt(fmt = "{:.2}")]
    pub struct Float(f32);

    assert_eq!("0.20", Float(0.2).to_string())
}
