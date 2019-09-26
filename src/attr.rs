use neli::consts::NlAttrType;
use neli::{impl_var, impl_var_base, impl_var_trait};
use std::fmt;

impl_var_trait!(
    NlaNested, u16, NlAttrType,
    Unspec => 0
);

impl_var_trait!(
    /// nl80211Attrs
    ///
    /// Enumeration from nl80211/nl80211.h:1929
    Nl80211Attr, u16, NlAttrType,
    AttrUnspec                       => 0,
    AttrWiphy                        => 1,
    AttrWiphyName                    => 2,
    AttrIfindex                      => 3,
    AttrIfname                       => 4,
    AttrIftype                       => 5,
    AttrMac                          => 6,
    AttrKeyData                      => 7,
    AttrKeyIdx                       => 8,
    AttrKeyCipher                    => 9,
    AttrKeySeq                       => 10,
    AttrKeyDefault                   => 11,
    AttrBeaconInterval               => 12,
    AttrDtimPeriod                   => 13,
    AttrBeaconHead                   => 14,
    AttrBeaconTail                   => 15,
    AttrStaAid                       => 16,
    AttrStaFlags                     => 17,
    AttrStaListenInterval            => 18,
    AttrStaSupportedRates            => 19,
    AttrStaVlan                      => 20,
    AttrStaInfo                      => 21,
    AttrWiphyBands                   => 22,
    AttrMntrFlags                    => 23,
    AttrMeshId                       => 24,
    AttrStaPlinkAction               => 25,
    AttrMpathNextHop                 => 26,
    AttrMpathInfo                    => 27,
    AttrBssCtsProt                   => 28,
    AttrBssShortPreamble             => 29,
    AttrBssShortSlotTime             => 30,
    AttrHtCapability                 => 31,
    AttrSupportedIftypes             => 32,
    AttrRegAlpha2                    => 33,
    AttrRegRules                     => 34,
    AttrMeshConfig                   => 35,
    AttrBssBasicRates                => 36,
    AttrWiphyTxqParams               => 37,
    AttrWiphyFreq                    => 38,
    AttrWiphyChannelType             => 39,
    AttrKeyDefaultMgmt               => 40,
    AttrMgmtSubtype                  => 41,
    AttrIe                           => 42,
    AttrMaxNumScanSsids              => 43,
    AttrScanFrequencies              => 44,
    AttrScanSsids                    => 45,
    AttrGeneration                   => 46,
    AttrBss                          => 47,
    AttrRegInitiator                 => 48,
    AttrRegType                      => 49,
    AttrSupportedCommands            => 50,
    AttrFrame                        => 51,
    AttrSsid                         => 52,
    AttrAuthType                     => 53,
    AttrReasonCode                   => 54,
    AttrKeyType                      => 55,
    AttrMaxScanIeLen                 => 56,
    AttrCipherSuites                 => 57,
    AttrFreqBefore                   => 58,
    AttrFreqAfter                    => 59,
    AttrFreqFixed                    => 60,
    AttrWiphyRetryShort              => 61,
    AttrWiphyRetryLong               => 62,
    AttrWiphyFragThreshold           => 63,
    AttrWiphyRtsThreshold            => 64,
    AttrTimedOut                     => 65,
    AttrUseMfp                       => 66,
    AttrStaFlags2                    => 67,
    AttrControlPort                  => 68,
    AttrTestdata                     => 69,
    AttrPrivacy                      => 70,
    AttrDisconnectedByAp             => 71,
    AttrStatusCode                   => 72,
    AttrCipherSuitesPairwise         => 73,
    AttrCipherSuiteGroup             => 74,
    AttrWpaVersions                  => 75,
    AttrAkmSuites                    => 76,
    AttrReqIe                        => 77,
    AttrRespIe                       => 78,
    AttrPrevBssid                    => 79,
    AttrKey                          => 80,
    AttrKeys                         => 81,
    AttrPid                          => 82,
    Attr4addr                        => 83,
    AttrSurveyInfo                   => 84,
    AttrPmkid                        => 85,
    AttrMaxNumPmkids                 => 86,
    AttrDuration                     => 87,
    AttrCookie                       => 88,
    AttrWiphyCoverageClass           => 89,
    AttrTxRates                      => 90,
    AttrFrameMatch                   => 91,
    AttrAck                          => 92,
    AttrPsState                      => 93,
    AttrCqm                          => 94,
    AttrLocalStateChange             => 95,
    AttrApIsolate                    => 96,
    AttrWiphyTxPowerSetting          => 97,
    AttrWiphyTxPowerLevel            => 98,
    AttrTxFrameTypes                 => 99,
    AttrRxFrameTypes                 => 100,
    AttrFrameType                    => 101,
    AttrControlPortEthertype         => 102,
    AttrControlPortNoEncrypt         => 103,
    AttrSupportIbssRsn               => 104,
    AttrWiphyAntennaTx               => 105,
    AttrWiphyAntennaRx               => 106,
    AttrMcastRate                    => 107,
    AttrOffchannelTxOk               => 108,
    AttrBssHtOpmode                  => 109,
    AttrKeyDefaultTypes              => 110,
    AttrMaxRemainOnChannelDuration   => 111,
    AttrMeshSetup                    => 112,
    AttrWiphyAntennaAvailTx          => 113,
    AttrWiphyAntennaAvailRx          => 114,
    AttrSupportMeshAuth              => 115,
    AttrStaPlinkState                => 116,
    AttrWowlanTriggers               => 117,
    AttrWowlanTriggersSupported      => 118,
    AttrSchedScanInterval            => 119,
    AttrInterfaceCombinations        => 120,
    AttrSoftwareIftypes              => 121,
    AttrRekeyData                    => 122,
    AttrMaxNumSchedScanSsids         => 123,
    AttrMaxSchedScanIeLen            => 124,
    AttrScanSuppRates                => 125,
    AttrHiddenSsid                   => 126,
    AttrIeProbeResp                  => 127,
    AttrIeAssocResp                  => 128,
    AttrStaWme                       => 129,
    AttrSupportApUapsd               => 130,
    AttrRoamSupport                  => 131,
    AttrSchedScanMatch               => 132,
    AttrMaxMatchSets                 => 133,
    AttrPmksaCandidate               => 134,
    AttrTxNoCckRate                  => 135,
    AttrTdlsAction                   => 136,
    AttrTdlsDialogToken              => 137,
    AttrTdlsOperation                => 138,
    AttrTdlsSupport                  => 139,
    AttrTdlsExternalSetup            => 140,
    AttrDeviceApSme                  => 141,
    AttrDontWaitForAck               => 142,
    AttrFeatureFlags                 => 143,
    AttrProbeRespOffload             => 144,
    AttrProbeResp                    => 145,
    AttrDfsRegion                    => 146,
    AttrDisableHt                    => 147,
    AttrHtCapabilityMask             => 148,
    AttrNoackMap                     => 149,
    AttrInactivityTimeout            => 150,
    AttrRxSignalDbm                  => 151,
    AttrBgScanPeriod                 => 152,
    AttrWdev                         => 153,
    AttrUserRegHintType              => 154,
    AttrConnFailedReason             => 155,
    AttrSaeData                      => 156,
    AttrVhtCapability                => 157,
    AttrScanFlags                    => 158,
    AttrChannelWidth                 => 159,
    AttrCenterFreq1                  => 160,
    AttrCenterFreq2                  => 161,
    AttrP2pCtwindow                  => 162,
    AttrP2pOppps                     => 163,
    AttrLocalMeshPowerMode           => 164,
    AttrAclPolicy                    => 165,
    AttrMacAddrs                     => 166,
    AttrMacAclMax                    => 167,
    AttrRadarEvent                   => 168,
    AttrExtCapa                      => 169,
    AttrExtCapaMask                  => 170,
    AttrStaCapability                => 171,
    AttrStaExtCapability             => 172,
    AttrProtocolFeatures             => 173,
    AttrSplitWiphyDump               => 174,
    AttrDisableVht                   => 175,
    AttrVhtCapabilityMask            => 176,
    AttrMdid                         => 177,
    AttrIeRic                        => 178,
    AttrCritProtId                   => 179,
    AttrMaxCritProtDuration          => 180,
    AttrPeerAid                      => 181,
    AttrCoalesceRule                 => 182,
    AttrChSwitchCount                => 183,
    AttrChSwitchBlockTx              => 184,
    AttrCsaIes                       => 185,
    AttrCsaCOffBeacon                => 186,
    AttrCsaCOffPresp                 => 187,
    AttrRxmgmtFlags                  => 188,
    AttrStaSupportedChannels         => 189,
    AttrStaSupportedOperClasses      => 190,
    AttrHandleDfs                    => 191,
    AttrSupport5Mhz                  => 192,
    AttrSupport10Mhz                 => 193,
    AttrOpmodeNotif                  => 194,
    AttrVendorId                     => 195,
    AttrVendorSubcmd                 => 196,
    AttrVendorData                   => 197,
    AttrVendorEvents                 => 198,
    AttrQosMap                       => 199,
    AttrMacHint                      => 200,
    AttrWiphyFreqHint                => 201,
    AttrMaxApAssocSta                => 202,
    AttrTdlsPeerCapability           => 203,
    AttrSocketOwner                  => 204,
    AttrCsaCOffsetsTx                => 205,
    AttrMaxCsaCounters               => 206,
    AttrTdlsInitiator                => 207,
    AttrUseRrm                       => 208,
    AttrWiphyDynAck                  => 209,
    AttrTsid                         => 210,
    AttrUserPrio                     => 211,
    AttrAdmittedTime                 => 212,
    AttrSmpsMode                     => 213,
    AttrOperClass                    => 214,
    AttrMacMask                      => 215,
    AttrWiphySelfManagedReg          => 216,
    AttrExtFeatures                  => 217,
    AttrSurveyRadioStats             => 218,
    AttrNetnsFd                      => 219,
    AttrSchedScanDelay               => 220,
    AttrRegIndoor                    => 221,
    AttrMaxNumSchedScanPlans         => 222,
    AttrMaxScanPlanInterval          => 223,
    AttrMaxScanPlanIterations        => 224,
    AttrSchedScanPlans               => 225,
    AttrPbss                         => 226,
    AttrBssSelect                    => 227,
    AttrStaSupportP2pPs              => 228,
    AttrPad                          => 229,
    AttrIftypeExtCapa                => 230,
    AttrMuMimoGroupData              => 231,
    AttrMuMimoFollowMacAddr          => 232,
    AttrScanStartTimeTsf             => 233,
    AttrScanStartTimeTsfBssid        => 234,
    AttrMeasurementDuration          => 235,
    AttrMeasurementDurationMandatory => 236,
    AttrMeshPeerAid                  => 237,
    AttrNanMasterPref                => 238,
    AttrNanDual                      => 239,
    AttrNanFunc                      => 240,
    AttrNanMatch                     => 241,
    AttrAfterLast                    => 242,
    NumAttr                          => 242,//__AttrAfterLast,
    AttrMax                          => 241//__AttrAfterLast - 1
);

impl fmt::Display for Nl80211Attr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl_var_trait!(
    /// nl80211Iftype
    ///
    /// Enumeration from nl80211/nl80211.h:2384
    Nl80211Iftype, u16, NlAttrType,
    IftypeUnspecified => 0,
    IftypeAdhoc       => 1,
    IftypeStation     => 2,
    IftypeAp          => 3,
    IftypeApVlan      => 4,
    IftypeWds         => 5,
    IftypeMonitor     => 6,
    IftypeMeshPoint   => 7,
    IftypeP2pClient   => 8,
    IftypeP2pGo       => 9,
    IftypeP2pDevice   => 10,
    IftypeOcb         => 11,
    IftypeNan         => 12,
    NumIftypes        => 13,
    IftypeMax         => 12
);

impl_var_trait!(
    // nl80211StaFlags as declared in nl80211/nl80211.h:2428
    Nl80211StaFlags, u16, NlAttrType,
    StaFlagInvalid       => 0,
    StaFlagAuthorized    => 1,
    StaFlagShortPreamble => 2,
    StaFlagWme           => 3,
    StaFlagMfp           => 4,
    StaFlagAuthenticated => 5,
    StaFlagTdlsPeer      => 6,
    StaFlagAssociated    => 7,
    StaFlagAfterLast     => 8,
    StaFlagMax           => 7
);

impl_var_trait!(
    ///nl80211StaP2pPsStatus
    ///
    /// Enumeration from nl80211/nl80211.h:2450
    Nl80211StaP2pPsStatus, u16, NlAttrType,
    P2pPsUnsupported => 0,
    P2pPsSupported   => 1,
    NumP2pPsStatus   => 2
);

impl_var_trait!(
    /// nl80211RateInfo
    ///
    /// Enumeration from nl80211/nl80211.h:2505
    Nl80211RateInfo, u16, NlAttrType,
    RateInfoInvalid       => 0,
    RateInfoBitrate       => 1,
    RateInfoMcs           => 2,
    RateInfo40MhzWidth    => 3,
    RateInfoShortGi       => 4,
    RateInfoBitrate32     => 5,
    RateInfoVhtMcs        => 6,
    RateInfoVhtNss        => 7,
    RateInfo80MhzWidth    => 8,
    RateInfo80p80MhzWidth => 9,
    RateInfo160MhzWidth   => 10,
    RateInfo10MhzWidth    => 11,
    RateInfo5MhzWidth     => 12,
    RateInfoAfterLast     => 13,
    RateInfoMax           => 12
);

impl_var_trait!(
    /// nl80211StaBssParam
    ///
    /// Enumeration from nl80211/nl80211.h:2542
    Nl80211StaBssParam, u16, NlAttrType,
    StaBssParamInvalid        => 0,
    StaBssParamCtsProt        => 1,
    StaBssParamShortPreamble  => 2,
    StaBssParamShortSlotTime  => 3,
    StaBssParamDtimPeriod     => 4,
    StaBssParamBeaconInterval => 5,
    StaBssParamAfterLast      => 6,
    StaBssParamMax            => 5
);

impl_var_trait!(
    /// nl80211StaInfo
    ///
    /// Enumeration from nl80211/nl80211.h:2620
    Nl80211StaInfo, u16, NlAttrType,
    StaInfoInvalid            => 0,
    StaInfoInactiveTime       => 1,
    StaInfoRxBytes            => 2,
    StaInfoTxBytes            => 3,
    StaInfoLlid               => 4,
    StaInfoPlid               => 5,
    StaInfoPlinkState         => 6,
    StaInfoSignal             => 7,
    StaInfoTxBitrate          => 8,
    StaInfoRxPackets          => 9,
    StaInfoTxPackets          => 10,
    StaInfoTxRetries          => 11,
    StaInfoTxFailed           => 12,
    StaInfoSignalAvg          => 13,
    StaInfoRxBitrate          => 14,
    StaInfoBssParam           => 15,
    StaInfoConnectedTime      => 16,
    StaInfoStaFlags           => 17,
    StaInfoBeaconLoss         => 18,
    StaInfoTOffset            => 19,
    StaInfoLocalPm            => 20,
    StaInfoPeerPm             => 21,
    StaInfoNonpeerPm          => 22,
    StaInfoRxBytes64          => 23,
    StaInfoTxBytes64          => 24,
    StaInfoChainSignal        => 25,
    StaInfoChainSignalAvg     => 26,
    StaInfoExpectedThroughput => 27,
    StaInfoRxDropMisc         => 28,
    StaInfoBeaconRx           => 29,
    StaInfoBeaconSignalAvg    => 30,
    StaInfoTidStats           => 31,
    StaInfoRxDuration         => 32,
    StaInfoPad                => 33,
    StaInfoAfterLast          => 34,
    StaInfoMax                => 33
);

impl_var_trait!(
    /// nl80211TidStats
    ///
    /// Enumeration from nl80211/nl80211.h:2675
    Nl80211TidStats, u16, NlAttrType,
    TidStatsInvalid       => 0,
    TidStatsRxMsdu        => 1,
    TidStatsTxMsdu        => 2,
    TidStatsTxMsduRetries => 3,
    TidStatsTxMsduFailed  => 4,
    TidStatsPad           => 5,
    NumTidStats           => 6,
    TidStatsMax           => 5
);

impl_var_trait!(
    /// nl80211MpathFlags
    ///
    /// Enumeration from nl80211/nl80211.h:2697
    Nl80211MpathFlags, u16, NlAttrType,
    MpathFlagActive    => 1 << 0,
    MpathFlagResolving => 1 << 1,
    MpathFlagSnValid   => 1 << 2,
    MpathFlagFixed     => 1 << 3,
    MpathFlagResolved  => 1 << 4
);

impl_var_trait!(
    /// nl80211MpathFlags
    ///
    /// Enumeration from nl80211/nl80211.h:2697
    Nl80211MpathInfo, u16, NlAttrType,
    MpathInfoInvalid          => 0,
    MpathInfoFrameQlen        => 1,
    MpathInfoSn               => 2,
    MpathInfoMetric           => 3,
    MpathInfoExptime          => 4,
    MpathInfoFlags            => 5,
    MpathInfoDiscoveryTimeout => 6,
    MpathInfoDiscoveryRetries => 7,
    MpathInfoAfterLast        => 8,
    MpathInfoMax              => 7
);

impl_var_trait!(
    /// nl80211BandAttr
    ///
    /// Enumeration from nl80211/nl80211.h:2757
    Nl80211BandAttr, u16, NlAttrType,
    BandAttrInvalid        => 0,
    BandAttrFreqs          => 1,
    BandAttrRates          => 2,
    BandAttrHtMcsSet       => 3,
    BandAttrHtCapa         => 4,
    BandAttrHtAmpduFactor  => 5,
    BandAttrHtAmpduDensity => 6,
    BandAttrVhtMcsSet      => 7,
    BandAttrVhtCapa        => 8,
    BandAttrAfterLast      => 9,
    BandAttrMax            => 8
);

impl_var_trait!(
    /// nl80211FrequencyAttr
    ///
    /// Enumeration from nl80211/nl80211.h:2833
    Nl80211FrequencyAttr, u16, NlAttrType,
    FrequencyAttrInvalid      => 0,
    FrequencyAttrFreq         => 1,
    FrequencyAttrDisabled     => 2,
    FrequencyAttrNoIr         => 3,
    FrequencyAttrNoIbss       => 4,
    FrequencyAttrRadar        => 5,
    FrequencyAttrMaxTxPower   => 6,
    FrequencyAttrDfsState     => 7,
    FrequencyAttrDfsTime      => 8,
    FrequencyAttrNoHt40Minus  => 9,
    FrequencyAttrNoHt40Plus   => 10,
    FrequencyAttrNo80mhz      => 11,
    FrequencyAttrNo160mhz     => 12,
    FrequencyAttrDfsCacTime   => 13,
    FrequencyAttrIndoorOnly   => 14,
    FrequencyAttrIrConcurrent => 15,
    FrequencyAttrNo20mhz      => 16,
    FrequencyAttrNo10mhz      => 17,
    FrequencyAttrAfterLast    => 18,
    FrequencyAttrMax          => 17
);


impl_var_trait!(
    /// nl80211BitrateAttr
    ///
    /// Enumeration from nl80211/nl80211.h:2873
    Nl80211BitrateAttr, u16, NlAttrType,
    BitrateAttrInvalid           => 0,
    BitrateAttrRate              => 1,
    BitrateAttr2ghzShortpreamble => 2,
    BitrateAttrAfterLast         => 3,
    BitrateAttrMax               => 2
);

impl_var_trait!(
    /// nl80211RegInitiator
    ///
    /// Enumeration from nl80211/nl80211.h:2899
    Nl80211RegInitiator, u16, NlAttrType,
    RegdomSetByCore      => 0,
    RegdomSetByUser      => 1,
    RegdomSetByDriver    => 2,
    RegdomSetByCountryIe => 3
);

impl_var_trait!(
    /// nl80211RegType
    ///
    /// Enumeration from nl80211/nl80211.h:2922
    Nl80211RegType, u16, NlAttrType,
    RegdomTypeCountry      => 0,
    RegdomTypeWorld        => 1,
    RegdomTypeCustomWorld  => 2,
    RegdomTypeIntersection => 3
);

impl_var_trait!(
    /// nl80211RegRuleAttr
    ///
    /// Enumeration from nl80211/nl80211.h:2954
    Nl80211RegRuleAttr, u16, NlAttrType,
    RegRuleAttrInvalid      => 0,
    AttrRegRuleFlags        => 1,
    AttrFreqRangeStart      => 2,
    AttrFreqRangeEnd        => 3,
    AttrFreqRangeMaxBw      => 4,
    AttrPowerRuleMaxAntGain => 5,
    AttrPowerRuleMaxEirp    => 6,
    AttrDfsCacTime          => 7,
    RegRuleAttrAfterLast    => 8,
    RegRuleAttrMax          => 7
);

impl_var_trait!(
    /// nl80211SchedScanMatchAttr
    ///
    /// Enumeration from nl80211/nl80211.h:2989
    Nl80211SchedScanMatchAttr, u16, NlAttrType,
    SchedScanMatchAttrInvalid   => 0,
    SchedScanMatchAttrSsid      => 1,
    SchedScanMatchAttrRssi      => 2,
    SchedScanMatchAttrAfterLast => 3,
    SchedScanMatchAttrMax       => 2
);

impl_var_trait!(
    /// nl80211RegRuleFlags
    ///
    /// Enumeration from nl80211/nl80211.h:3026
    Nl80211RegRuleFlags, u16, NlAttrType,
    RrfNoOfdm       => 1 << 0,
    RrfNoCck        => 1 << 1,
    RrfNoIndoor     => 1 << 2,
    RrfNoOutdoor    => 1 << 3,
    RrfDfs          => 1 << 4,
    RrfPtpOnly      => 1 << 5,
    RrfPtmpOnly     => 1 << 6,
    RrfNoIr         => 1 << 7,
    RrfNoIbss       => 1 << 8,
    RrfAutoBw       => 1 << 11,
    RrfIrConcurrent => 1 << 12,
    RrfNoHt40minus  => 1 << 13,
    RrfNoHt40plus   => 1 << 14,
    RrfNo80mhz      => 1 << 15
    // RrfNo160mhz   => 1 << 16
);

impl_var_trait!(
    /// nl80211DfsRegions
    ///
    /// Enumeration from nl80211/nl80211.h:3061
    Nl80211DfsRegions, u16, NlAttrType,
    DfsUnset => 0,
    DfsFcc   => 1,
    DfsEtsi  => 2,
    DfsJp    => 3
);

impl_var_trait!(
    /// nl80211UserRegHintType
    ///
    /// Enumeration from nl80211/nl80211.h:3085
    Nl80211UserRegHintType, u16, NlAttrType,
    UserRegHintUser     => 0,
    UserRegHintCellBase => 1,
    UserRegHintIndoor   => 2
);


impl_var_trait!(
    /// nl80211SurveyInfo
    ///
    /// Enumeration from nl80211/nl80211.h:3118
    Nl80211SurveyInfo, u16, NlAttrType,
    SurveyInfoInvalid     => 0,
    SurveyInfoFrequency   => 1,
    SurveyInfoNoise       => 2,
    SurveyInfoInUse       => 3,
    SurveyInfoTime        => 4,
    SurveyInfoTimeBusy    => 5,
    SurveyInfoTimeExtBusy => 6,
    SurveyInfoTimeRx      => 7,
    SurveyInfoTimeTx      => 8,
    SurveyInfoTimeScan    => 9,
    SurveyInfoPad         => 10,
    SurveyInfoAfterLast   => 11,
    SurveyInfoMax         => 10
);

impl_var_trait!(
    /// nl80211MntrFlags
    ///
    /// Enumeration from nl80211/nl80211.h:3162
    Nl80211MntrFlags, u16, NlAttrType,
    MntrFlagInvalid    => 0,
    MntrFlagFcsfail    => 1,
    MntrFlagPlcpfail   => 2,
    MntrFlagControl    => 3,
    MntrFlagOtherBss   => 4,
    MntrFlagCookFrames => 5,
    MntrFlagActive     => 6,
    MntrFlagAfterLast  => 7,
    MntrFlagMax        => 6
);

impl_var_trait!(
    /// nl80211MeshPowerMode
    ///
    /// Enumeration from nl80211/nl80211.h:3194
    Nl80211MeshPowerMode, u16, NlAttrType,
    MeshPowerUnknown    => 0,
    MeshPowerActive     => 1,
    MeshPowerLightSleep => 2,
    MeshPowerDeepSleep  => 3,
    MeshPowerAfterLast  => 4,
    MeshPowerMax        => 3
);

impl_var_trait!(
    /// nl80211MeshconfParams
    ///
    /// Enumeration from nl80211/nl80211.h:3312
    Nl80211MeshconfParams, u16, NlAttrType,
    MeshconfInvalid                  => 0,
    MeshconfRetryTimeout             => 1,
    MeshconfConfirmTimeout           => 2,
    MeshconfHoldingTimeout           => 3,
    MeshconfMaxPeerLinks             => 4,
    MeshconfMaxRetries               => 5,
    MeshconfTtl                      => 6,
    MeshconfAutoOpenPlinks           => 7,
    MeshconfHwmpMaxPreqRetries       => 8,
    MeshconfPathRefreshTime          => 9,
    MeshconfMinDiscoveryTimeout      => 10,
    MeshconfHwmpActivePathTimeout    => 11,
    MeshconfHwmpPreqMinInterval      => 12,
    MeshconfHwmpNetDiamTrvsTime      => 13,
    MeshconfHwmpRootmode             => 14,
    MeshconfElementTtl               => 15,
    MeshconfHwmpRannInterval         => 16,
    MeshconfGateAnnouncements        => 17,
    MeshconfHwmpPerrMinInterval      => 18,
    MeshconfForwarding               => 19,
    MeshconfRssiThreshold            => 20,
    MeshconfSyncOffsetMaxNeighbor    => 21,
    MeshconfHtOpmode                 => 22,
    MeshconfHwmpPathToRootTimeout    => 23,
    MeshconfHwmpRootInterval         => 24,
    MeshconfHwmpConfirmationInterval => 25,
    MeshconfPowerMode                => 26,
    MeshconfAwakeWindow              => 27,
    MeshconfPlinkTimeout             => 28,
    MeshconfAttrAfterLast            => 29,
    MeshconfAttrMax                  => 28
);

impl_var_trait!(
    /// nl80211MeshSetupParams
    ///
    /// Enumeration from nl80211/nl80211.h:3397
    Nl80211MeshSetupParams, u16, NlAttrType,
    MeshSetupInvalid             => 0,
    MeshSetupEnableVendorPathSel => 1,
    MeshSetupEnableVendorMetric  => 2,
    MeshSetupIe                  => 3,
    MeshSetupUserspaceAuth       => 4,
    MeshSetupUserspaceAmpe       => 5,
    MeshSetupEnableVendorSync    => 6,
    MeshSetupUserspaceMpm        => 7,
    MeshSetupAuthProtocol        => 8,
    MeshSetupAttrAfterLast       => 9,
    MeshSetupAttrMax             => 8
);

impl_var_trait!(
    /// nl80211TxqAttr
    ///
    /// Enumeration from nl80211/nl80211.h:3427
    Nl80211TxqAttr, u16, NlAttrType,
    TxqAttrInvalid   => 0,
    TxqAttrAc        => 1,
    TxqAttrTxop      => 2,
    TxqAttrCwmin     => 3,
    TxqAttrCwmax     => 4,
    TxqAttrAifs      => 5,
    TxqAttrAfterLast => 6,
    TxqAttrMax       => 5
);

impl_var_trait!(
    /// nl80211Ac
    ///
    /// Enumeration from nl80211/nl80211.h:3440
    Nl80211Ac, u16, NlAttrType,
    AcVo   => 0,
    AcVi   => 1,
    AcBe   => 2,
    AcBk   => 3,
    NumAcs => 4
);

impl_var_trait!(
    /// nl80211ChannelType
    ///
    /// Enumeration from nl80211/nl80211.h:3464
    Nl80211ChannelType, u16, NlAttrType,
    ChanNoHt      => 0,
    ChanHt20      => 1,
    ChanHt40minus => 2,
    ChanHt40plus  => 3
);

impl_var_trait!(
    /// nl80211ChanWidth
    ///
    /// Enumeration from nl80211/nl80211.h:3490
    Nl80211ChanWidth, u16, NlAttrType,
    ChanWidth20Noht => 0,
    ChanWidth20     => 1,
    ChanWidth40     => 2,
    ChanWidth80     => 3,
    ChanWidth80p80  => 4,
    ChanWidth160    => 5,
    ChanWidth5      => 6,
    ChanWidth10     => 7
);

impl_var_trait!(
    /// nl80211BssScanWidth
    ///
    /// Enumeration from nl80211/nl80211.h:3510
    Nl80211BssScanWidth, u16, NlAttrType,
    BssChanWidth20 => 0,
    BssChanWidth10 => 1,
    BssChanWidth5  => 2
);

impl_var_trait!(
    /// nl80211Bss
    ///
    /// Enumeration from nl80211/nl80211.h:3565
    Nl80211Bss, u16, NlAttrType,
    BssInvalid             => 0,
    BssBssid               => 1,
    BssFrequency           => 2,
    BssTsf                 => 3,
    BssBeaconInterval      => 4,
    BssCapability          => 5,
    BssInformationElements => 6,
    BssSignalMbm           => 7,
    BssSignalUnspec        => 8,
    BssStatus              => 9,
    BssSeenMsAgo           => 10,
    BssBeaconIes           => 11,
    BssChanWidth           => 12,
    BssBeaconTsf           => 13,
    BssPrespData           => 14,
    BssLastSeenBoottime    => 15,
    BssPad                 => 16,
    BssParentTsf           => 17,
    BssParentBssid         => 18,
    BssAfterLast           => 19,
    BssMax => 18
);

impl_var_trait!(
    /// nl80211BssStatus
    ///
    /// Enumeration from nl80211/nl80211.h:3603
    Nl80211BssStatus, u16, NlAttrType,
    BssStatusAuthenticated => 0,
    BssStatusAssociated    => 1,
    BssStatusIbssJoined    => 2
);

impl_var_trait!(
    /// nl80211AuthType
    ///
    /// Enumeration from nl80211/nl80211.h:3623
    Nl80211AuthType, u16, NlAttrType,
    AuthtypeOpenSystem => 0,
    AuthtypeSharedKey  => 1,
    AuthtypeFt         => 2,
    AuthtypeNetworkEap => 3,
    AuthtypeSae        => 4,
    AuthtypeNum        => 5,
    AuthtypeMax        => 4,
    AuthtypeAutomatic  => 5
);

impl_var_trait!(
    /// nl80211KeyType
    ///
    /// Enumeration from nl80211/nl80211.h:3643
    Nl80211KeyType, u16, NlAttrType,
    KeytypeGroup    => 0,
    KeytypePairwise => 1,
    KeytypePeerkey  => 2,
    NumKeytypes     => 3
);

impl_var_trait!(
    /// nl80211Mfp
    ///
    /// Enumeration from nl80211/nl80211.h:3656
    Nl80211Mfp, u16, NlAttrType,
    MfpNo       => 0,
    MfpRequired => 1
);

impl_var_trait!(
    /// nl80211WpaVersions
    ///
    /// Enumeration from nl80211/nl80211.h:3661
    Nl80211WpaVersions, u16, NlAttrType,
    WpaVersion1 => 1 << 0,
    WpaVersion2 => 1 << 1
);

impl_var_trait!(
    /// nl80211KeyDefaultTypes
    ///
    /// Enumeration from nl80211/nl80211.h:3675
    Nl80211KeyDefaultTypes, u16, NlAttrType,
    KeyDefaultTypeInvalid   => 0,
    KeyDefaultTypeUnicast   => 1,
    KeyDefaultTypeMulticast => 2,
    NumKeyDefaultTypes      => 3
);

impl_var_trait!(
    /// nl80211KeyAttributes
    ///
    /// Enumeration from nl80211/nl80211.h:3705
    Nl80211KeyAttributes, u16, NlAttrType,
    KeyInvalid      => 0,
    KeyData         => 1,
    KeyIdx          => 2,
    KeyCipher       => 3,
    KeySeq          => 4,
    KeyDefault      => 5,
    KeyDefaultMgmt  => 6,
    KeyType         => 7,
    KeyDefaultTypes => 8,
    KeyAfterLast    => 9,
    KeyMax          => 8
);

impl_var_trait!(
    /// nl80211TxRateAttributes
    ///
    /// Enumeration from nl80211/nl80211.h:3736
    Nl80211TxRateAttributes, u16, NlAttrType,
    TxrateInvalid   => 0,
    TxrateLegacy    => 1,
    TxrateHt        => 2,
    TxrateVht       => 3,
    TxrateGi        => 4,
    TxrateAfterLast => 5,
    TxrateMax       => 4
);

impl_var_trait!(
    /// nl80211TxrateGi
    ///
    /// Enumeration from nl80211/nl80211.h:3759
    Nl80211TxrateGi, u16, NlAttrType,
    TxrateDefaultGi => 0,
    TxrateForceSgi  => 1,
    TxrateForceLgi  => 2
);

impl_var_trait!(
    /// nl80211Band
    ///
    /// Enumeration from nl80211/nl80211.h:3773
    Nl80211Bandc, u16, NlAttrType,
    Band2ghz  => 0,
    Band5ghz  => 1,
    Band60ghz => 2,
    NumBands  => 3
);

impl_var_trait!(
    /// nl80211PsState
    ///
    /// Enumeration from nl80211/nl80211.h:3786
    Nl80211PsState, u16, NlAttrType,
    PsDisabled => 0,
    PsEnabled  => 1
);

impl_var_trait!(
    /// nl80211AttrCqm
    ///
    /// Enumeration from nl80211/nl80211.h:3819
    Nl80211AttrCqm, u16, NlAttrType,
    AttrCqmInvalid            => 0,
    AttrCqmRssiThold          => 1,
    AttrCqmRssiHyst           => 2,
    AttrCqmRssiThresholdEvent => 3,
    AttrCqmPktLossEvent       => 4,
    AttrCqmTxeRate            => 5,
    AttrCqmTxePkts            => 6,
    AttrCqmTxeIntvl           => 7,
    AttrCqmBeaconLossEvent    => 8,
    AttrCqmAfterLast          => 9,
    AttrCqmMax                => 8
);

impl_var_trait!(
    /// nl80211CqmRssiThresholdEvent
    ///
    /// Enumeration from nl80211/nl80211.h:3843
    Nl80211CqmRssiThresholdEvent, u16, NlAttrType,
    CqmRssiThresholdEventLow  => 0,
    CqmRssiThresholdEventHigh => 1,
    CqmRssiBeaconLossEvent    => 2
);

impl_var_trait!(
    /// nl80211TxPowerSetting
    ///
    /// Enumeration from nl80211/nl80211.h:3856
    Nl80211TxPowerSetting, u16, NlAttrType,
    TxPowerAutomatic => 0,
    TxPowerLimited   => 1,
    TxPowerFixed     => 2
);

impl_var_trait!(
    /// nl80211PacketPatternAttr
    ///
    /// Enumeration from nl80211/nl80211.h:3883
    Nl80211PacketPatternAttr, u16, NlAttrType,
    PktpatInvalid => 0,
    PktpatMask    => 1,
    PktpatPattern => 2,
    PktpatOffset  => 3,
    NumPktpat     => 4,
    MaxPktpat     => 3
);

impl_var_trait!(
    /// nl80211WowlanTriggers
    ///
    /// Enumeration from nl80211/nl80211.h:4011
    Nl80211WowlanTriggers, u16, NlAttrType,
    WowlanTrigInvalid               => 0,
    WowlanTrigAny                   => 1,
    WowlanTrigDisconnect            => 2,
    WowlanTrigMagicPkt              => 3,
    WowlanTrigPktPattern            => 4,
    WowlanTrigGtkRekeySupported     => 5,
    WowlanTrigGtkRekeyFailure       => 6,
    WowlanTrigEapIdentRequest       => 7,
    WowlanTrig4wayHandshake         => 8,
    WowlanTrigRfkillRelease         => 9,
    WowlanTrigWakeupPkt80211        => 10,
    WowlanTrigWakeupPkt80211Len     => 11,
    WowlanTrigWakeupPkt8023         => 12,
    WowlanTrigWakeupPkt8023Len      => 13,
    WowlanTrigTcpConnection         => 14,
    WowlanTrigWakeupTcpMatch        => 15,
    WowlanTrigWakeupTcpConnlost     => 16,
    WowlanTrigWakeupTcpNomoretokens => 17,
    WowlanTrigNetDetect             => 18,
    WowlanTrigNetDetectResults      => 19,
    NumWowlanTrig                   => 20,
    MaxWowlanTrig                   => 19
);

impl_var_trait!(
    /// nl80211WowlanTcpAttrs
    ///
    /// Enumeration from nl80211/nl80211.h:4129
    Nl80211WowlanTcpAttrs, u16, NlAttrType,
    WowlanTcpInvalid          => 0,
    WowlanTcpSrcIpv4          => 1,
    WowlanTcpDstIpv4          => 2,
    WowlanTcpDstMac           => 3,
    WowlanTcpSrcPort          => 4,
    WowlanTcpDstPort          => 5,
    WowlanTcpDataPayload      => 6,
    WowlanTcpDataPayloadSeq   => 7,
    WowlanTcpDataPayloadToken => 8,
    WowlanTcpDataInterval     => 9,
    WowlanTcpWakePayload      => 10,
    WowlanTcpWakeMask         => 11,
    NumWowlanTcp              => 12,
    MaxWowlanTcp              => 11
);

impl_var_trait!(
    /// nl80211AttrCoalesceRule
    ///
    /// Enumeration from nl80211/nl80211.h:4174
    Nl80211AttrCoalesceRule, u16, NlAttrType,
    CoalesceRuleInvalid        => 0,
    AttrCoalesceRuleDelay      => 1,
    AttrCoalesceRuleCondition  => 2,
    AttrCoalesceRulePktPattern => 3,
    NumAttrCoalesceRule        => 4,
    AttrCoalesceRuleMax        => 3
);

impl_var_trait!(
    /// nl80211CoalesceCondition
    ///
    /// Enumeration from nl80211/nl80211.h:4192
    Nl80211CoalesceCondition, u16, NlAttrType,
    CoalesceConditionMatch   => 0,
    CoalesceConditionNoMatch => 1
);

impl_var_trait!(
    /// nl80211IfaceLimitAttrs
    ///
    /// Enumeration from nl80211/nl80211.h:4207
    Nl80211IfaceLimitAttrs, u16, NlAttrType,
    IfaceLimitUnspec => 0,
    IfaceLimitMax    => 1,
    IfaceLimitTypes  => 2,
    NumIfaceLimit    => 3,
    MaxIfaceLimit    => 2
);

impl_var_trait!(
    /// nl80211IfCombinationAttrs
    ///
    /// Enumeration from nl80211/nl80211.h:4263
    Nl80211IfCombinationAttrs, u16, NlAttrType,
    IfaceCombUnspec             => 0,
    IfaceCombLimits             => 1,
    IfaceCombMaxnum             => 2,
    IfaceCombStaApBiMatch       => 3,
    IfaceCombNumChannels        => 4,
    IfaceCombRadarDetectWidths  => 5,
    IfaceCombRadarDetectRegions => 6,
    NumIfaceComb                => 7,
    MaxIfaceComb                => 6
);

impl_var_trait!(
    /// nl80211PlinkState
    ///
    /// Enumeration from nl80211/nl80211.h:4296
    Nl80211PlinkState, u16, NlAttrType,
    PlinkListen    => 0,
    PlinkOpnSnt    => 1,
    PlinkOpnRcvd   => 2,
    PlinkCnfRcvd   => 3,
    PlinkEstab     => 4,
    PlinkHolding   => 5,
    PlinkBlocked   => 6,
    NumPlinkStates => 7,
    MaxPlinkStates => 6
);

impl_var_trait!(
    /// plinkActions
    ///
    /// Enumeration from nl80211/nl80211.h:4318
    PlinkActions, u16, NlAttrType,
    PlinkActionNoAction => 0,
    PlinkActionOpen     => 1,
    PlinkActionBlock    => 2,
    NumPlinkActions     => 3
);

impl_var_trait!(
    /// nl80211RekeyData
    ///
    /// Enumeration from nl80211/nl80211.h:4340
    Nl80211RekeyData, u16, NlAttrType,
    RekeyDataInvalid   => 0,
    RekeyDataKek       => 1,
    RekeyDataKck       => 2,
    RekeyDataReplayCtr => 3,
    NumRekeyData       => 4,
    MaxRekeyData       => 3
);

impl_var_trait!(
    /// nl80211HiddenSsid
    ///
    /// Enumeration from nl80211/nl80211.h:4360
    Nl80211HiddenSsid, u16, NlAttrType,
    HiddenSsidNotInUse     => 0,
    HiddenSsidZeroLen      => 1,
    HiddenSsidZeroContents => 2
);

impl_var_trait!(
    /// nl80211StaWmeAttr
    ///
    /// Enumeration from nl80211/nl80211.h:4376
    Nl80211StaWmeAttr, u16, NlAttrType,
    StaWmeInvalid     => 0,
    StaWmeUapsdQueues => 1,
    StaWmeMaxSp       => 2,
    StaWmeAfterLast   => 3,
    StaWmeMax         => 2
);

impl_var_trait!(
    /// nl80211PmksaCandidateAttr
    ///
    /// Enumeration from nl80211/nl80211.h:4398
    Nl80211PmksaCandidateAttr, u16, NlAttrType,
    PmksaCandidateInvalid => 0,
    PmksaCandidateIndex   => 1,
    PmksaCandidateBssid   => 2,
    PmksaCandidatePreauth => 3,
    NumPmksaCandidate     => 4,
    MaxPmksaCandidate     => 3
);

impl_var_trait!(
    /// nl80211TdlsOperation
    ///
    /// Enumeration from nl80211/nl80211.h:4417
    Nl80211TdlsOperation, u16, NlAttrType,
    TdlsDiscoveryReq => 0,
    TdlsSetup        => 1,
    TdlsTeardown     => 2,
    TdlsEnableLink   => 3,
    TdlsDisableLink  => 4
);

impl_var_trait!(
    /// nl80211FeatureFlags
    ///
    /// Enumeration from nl80211/nl80211.h:4526
    Nl80211FeatureFlags, u16, NlAttrType,
    FeatureSkTxStatus                => 1 << 0,
    FeatureHtIbss                    => 1 << 1,
    FeatureInactivityTimer           => 1 << 2,
    FeatureCellBaseRegHints          => 1 << 3,
    FeatureP2pDeviceNeedsChannel     => 1 << 4,
    FeatureSae                       => 1 << 5,
    FeatureLowPriorityScan           => 1 << 6,
    FeatureScanFlush                 => 1 << 7,
    FeatureApScan                    => 1 << 8,
    FeatureVifTxpower                => 1 << 9,
    FeatureNeedObssScan              => 1 << 10,
    FeatureP2pGoCtwin                => 1 << 11,
    FeatureP2pGoOppps                => 1 << 12,
    FeatureAdvertiseChanLimits       => 1 << 14,
    FeatureFullApClientState         => 1 << 15
    // FeatureUserspaceMpm           => 1 << 16,
    // FeatureActiveMonitor          => 1 << 17,
    // FeatureApModeChanWidthChange  => 1 << 18,
    // FeatureDsParamSetIeInProbes   => 1 << 19,
    // FeatureWfaTpcIeInProbes       => 1 << 20,
    // FeatureQuiet                  => 1 << 21,
    // FeatureTxPowerInsertion       => 1 << 22,
    // FeatureAcktoEstimation        => 1 << 23,
    // FeatureStaticSmps             => 1 << 24,
    // FeatureDynamicSmps            => 1 << 25,
    // FeatureSupportsWmmAdmission   => 1 << 26,
    // FeatureMacOnCreate            => 1 << 27,
    // FeatureTdlsChannelSwitch      => 1 << 28,
    // FeatureScanRandomMacAddr      => 1 << 29,
    // FeatureSchedScanRandomMacAddr => 1 << 30,
    // FeatureNdRandomMacAddr        => 1 << 31
);

impl_var_trait!(
    /// nl80211ExtFeatureIndex
    ///
    /// Enumeration from nl80211/nl80211.h:4595
    Nl80211ExtFeatureIndex, u16, NlAttrType,
    ExtFeatureVhtIbss          => 0,
    ExtFeatureRrm              => 1,
    ExtFeatureMuMimoAirSniffer => 2,
    ExtFeatureScanStartTime    => 3,
    ExtFeatureBssParentTsf     => 4,
    ExtFeatureSetScanDwell     => 5,
    ExtFeatureBeaconRateLegacy => 6,
    ExtFeatureBeaconRateHt     => 7,
    ExtFeatureBeaconRateVht    => 8,
    NumExtFeatures             => 9,
    MaxExtFeatures             => 8
);

impl_var_trait!(
    /// nl80211ProbeRespOffloadSupportAttr
    ///
    /// Enumeration from nl80211/nl80211.h:4625
    Nl80211ProbeRespOffloadSupportAttr, u16, NlAttrType,
    ProbeRespOffloadSupportWps    => 1 << 0,
    ProbeRespOffloadSupportWps2   => 1 << 1,
    ProbeRespOffloadSupportP2p    => 1 << 2,
    ProbeRespOffloadSupport80211u => 1 << 3
);

impl_var_trait!(
    /// nl80211ConnectFailedReason
    ///
    /// Enumeration from nl80211/nl80211.h:4638
    Nl80211ConnectFailedReason, u16, NlAttrType,
    ConnFailMaxClients    => 0,
    ConnFailBlockedClient => 1
);

impl_var_trait!(
    /// nl80211ScanFlags
    ///
    /// Enumeration from nl80211/nl80211.h:4667
    Nl80211ScanFlags, u16, NlAttrType,
    ScanFlagLowPriority => 1 << 0,
    ScanFlagFlush       => 1 << 1,
    ScanFlagAp          => 1 << 2,
    ScanFlagRandomAddr  => 1 << 3
);

impl_var_trait!(
    /// nl80211AclPolicy
    ///
    /// Enumeration from nl80211/nl80211.h:4687
    Nl80211AclPolicy, u16, NlAttrType,
    AclPolicyAcceptUnlessListed => 0,
    AclPolicyDenyUnlessListed   => 1
);

impl_var_trait!(
    /// nl80211SmpsMode
    ///
    /// Enumeration from nl80211/nl80211.h:4702
    Nl80211SmpsMode, u16, NlAttrType,
    SmpsOff       => 0,
    SmpsStatic    => 1,
    SmpsDynamic   => 2,
    SmpsAfterLast => 3,
    SmpsMax       => 2
);

impl_var_trait!(
    /// nl80211RadarEvent
    ///
    /// Enumeration from nl80211/nl80211.h:4726
    Nl80211RadarEvent, u16, NlAttrType,
    RadarDetected    => 0,
    RadarCacFinished => 1,
    RadarCacAborted  => 2,
    RadarNopFinished => 3
);

impl_var_trait!(
    /// nl80211DfsState
    ///
    /// Enumeration from nl80211/nl80211.h:4744
    Nl80211DfsState, u16, NlAttrType,
    DfsUsable      => 0,
    DfsUnavailable => 1,
    DfsAvailable   => 2
);

impl_var_trait!(
    /// nl80211ProtocolFeatures
    ///
    /// Enumeration from nl80211/nl80211.h:4758
    Nl80211ProtocolFeatures, u16, NlAttrType,
    ProtocolFeatureSplitWiphyDump => 1 << 0
);

impl_var_trait!(
    /// nl80211CritProtoId
    ///
    /// Enumeration from nl80211/nl80211.h:4771
    Nl80211CritProtoId, u16, NlAttrType,
    CritProtoUnspec => 0,
    CritProtoDhcp   => 1,
    CritProtoEapol  => 2,
    CritProtoApipa  => 3,
    NumCritProto    => 4
);

impl_var_trait!(
    /// nl80211RxmgmtFlags
    ///
    /// Enumeration from nl80211/nl80211.h:4790
    Nl80211RxmgmtFlags, u16, NlAttrType,
    RxmgmtFlagAnswered => 1 << 0
);

impl_var_trait!(
    /// nl80211TdlsPeerCapability
    ///
    /// Enumeration from nl80211/nl80211.h:4824
    Nl80211TdlsPeerCapability, u16, NlAttrType,
    TdlsPeerHt  => 1 << 0,
    TdlsPeerVht => 1 << 1,
    TdlsPeerWmm => 1 << 2
);

impl_var_trait!(
    /// nl80211SchedScanPlan
    ///
    /// Enumeration from nl80211/nl80211.h:4843
    Nl80211SchedScanPlan, u16, NlAttrType,
    SchedScanPlanInvalid    => 0,
    SchedScanPlanInterval   => 1,
    SchedScanPlanIterations => 2,
    SchedScanPlanAfterLast  => 3,
    SchedScanPlanMax        => 2
);

impl_var_trait!(
    /// nl80211BssSelectAttr
    ///
    /// Enumeration from nl80211/nl80211.h:4887
    Nl80211BssSelectAttr, u16, NlAttrType,
    BssSelectAttrInvalid    => 0,
    BssSelectAttrRssi       => 1,
    BssSelectAttrBandPref   => 2,
    BssSelectAttrRssiAdjust => 3,
    BssSelectAttrAfterLast  => 4,
    BssSelectAttrMax        => 3
);

impl_var_trait!(
    /// nl80211NanDualBandConf
    ///
    /// Enumeration from nl80211/nl80211.h:4907
    Nl80211NanDualBandConf, u16, NlAttrType,
    NanBandDefault => 1 << 0,
    NanBand2ghz    => 1 << 1,
    NanBand5ghz    => 1 << 2
);

impl_var_trait!(
    /// nl80211NanFunctionType
    ///
    /// Enumeration from nl80211/nl80211.h:4922
    Nl80211NanFunctionType, u16, NlAttrType,
    NanFuncPublish       => 0,
    NanFuncSubscribe     => 1,
    NanFuncFollowUp      => 2,
    NanFuncTypeAfterLast => 3,
    NanFuncMaxType       => 2
);

impl_var_trait!(
    /// nl80211NanPublishType
    ///
    /// Enumeration from nl80211/nl80211.h:4940
    Nl80211NanPublishType, u16, NlAttrType,
    NanSolicitedPublish   => 1 << 0,
    NanUnsolicitedPublish => 1 << 1
);

impl_var_trait!(
    /// nl80211NanFuncTermReason
    ///
    /// Enumeration from nl80211/nl80211.h:4954
    Nl80211NanFuncTermReason, u16, NlAttrType,
    NanFuncTermReasonUserRequest => 0,
    NanFuncTermReasonTtlExpired  => 1,
    NanFuncTermReasonError       => 2
);

impl_var_trait!(
    /// nl80211NanFuncAttributes
    ///
    /// Enumeration from nl80211/nl80211.h:5006
    Nl80211NanFuncAttributes, u16, NlAttrType,
    NanFuncInvalid         => 0,
    NanFuncType            => 1,
    NanFuncServiceId       => 2,
    NanFuncPublishType     => 3,
    NanFuncPublishBcast    => 4,
    NanFuncSubscribeActive => 5,
    NanFuncFollowUpId      => 6,
    NanFuncFollowUpReqId   => 7,
    NanFuncFollowUpDest    => 8,
    NanFuncCloseRange      => 9,
    NanFuncTtl             => 10,
    NanFuncServiceInfo     => 11,
    NanFuncSrf             => 12,
    NanFuncRxMatchFilter   => 13,
    NanFuncTxMatchFilter   => 14,
    NanFuncInstanceId      => 15,
    NanFuncTermReason      => 16,
    NumNanFuncAttr         => 17,
    NanFuncAttrMax         => 16
);

impl_var_trait!(
    /// nl80211NanSrfAttributes
    ///
    /// Enumeration from nl80211/nl80211.h:5045
    Nl80211NanSrfAttributes, u16, NlAttrType,
    NanSrfInvalid   => 0,
    NanSrfInclude   => 1,
    NanSrfBf        => 2,
    NanSrfBfIdx     => 3,
    NanSrfMacAddrs  => 4,
    NumNanSrfAttr   => 5,
    NanSrfAttrMax   => 4
);

impl_var_trait!(
    /// nl80211NanMatchAttributes
    ///
    /// Enumeration from nl80211/nl80211.h:5070
    Nl80211NanMatchAttributes, u16, NlAttrType,
    NanMatchInvalid   => 0,
    NanMatchFuncLocal => 1,
    NanMatchFuncPeer  => 2,
    NumNanMatchAttr   => 3,
    NanMatchAttrMax   => 2
);
