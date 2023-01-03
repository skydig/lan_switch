#![allow(dead_code,unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use bit_struct::*; 
use std::string::*;
bit_struct! {
    pub struct phy_mv_16d(u16){
        DisableLinkPulses:u1,
        DownshiftCounter:u3,
        DownshiftEnable:u1,
        ForceCooperLinkGood:u1,
        EnergyDetect:u3,
        MDICrossoverMode:u2,
        EnergyDetectWakupControl:u1,
        CooperTransmitterDisable:u1,
        PowerDown:u1,
        PolarityReversalDisable:u1,
        DisableJabber:u1,
    }
    pub struct phy_mv_17d(u16){
        Speed:u2,
        Duplex:u1,
        PageReceived:u1,
        SpeedAndDuplexResolved:u1,
        CopperLinkReal:u1,
        TransmitPauseEnabled:u1,
        ReceivePauseEnabled:u1,
        Reserved:u1,
        MDICrossoverStatus:u1,
        DownshiftStatus:u1,
        CopperEnergyDetectStatus:u1,
        GlobalLinkStatus:u1,
        DTEPowerStatus:u1,
        PolarityReal:u1,
        JabberReal:u1,
    }
}
