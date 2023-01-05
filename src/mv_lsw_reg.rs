#![allow(dead_code,unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_macros)]

use bit_struct::*; 
use std::string::*;
bit_struct! {
    struct mv_6390_G1_1B_0x00_STATUS(u16){//P187
        IS:u1,
        Reserved:u3,
        IR:u1,
        Reserved1:u2,
        AVBInt:u1,
        DeviceInt:u1,
        StatsDone:u1,
        VTUProb:u1,
        VTUDone:u1,
        ATUProb:u1,
        ATUDone:u1,
        TCAMInt:u1,
        EEInt:u1,
    }
    struct mv_6390_G1_1B_0x01_ATU_FID(u16){
        Reserved:u4,
        FID:u12
    }
    struct mv_6390_G1_1B_0x02_VTU_FID(u16){
        Reserved:u3,
        VP:u1,
        FID:u12,
    }
    struct mv_6390_G1_1B_0x03_VTU_SID(u16){
        DontLearn:u1,
        FilterUC:u1,
        FilterBC:u1,
        FilterMC:u1,
        Reserved:u6,
        SID:u6,
    }
    struct mv_6390_G1_1B_0x05_VTU(u16){
        VB:u1,
        VTUOp:u3,
        VTUMode:u2,
        Reserved:u3,
        MemV:u1,
        MissV:u1,
        SPID:u5,
    }
    struct mv_6390_G1_1B_0x06_VTU(u16){
        Reserved:u2,
        P:u1,
        V:u1,
        VID:u12,
    }

    struct mv_6390_G1_1B_0x07_VTU(u16){
        P7S:u2,
        P6S:u2,
        P5S:u2,
        P4S:u2,
        P3S:u2,
        P2S:u2,
        P1S:u2,
        P0S:u2,
    }

    struct mv_6390_G1_1B_0x08_VTU(u16){
        VQ:u1,
        VID_QPRI:u3,
        VF:u1,
        VID_FPRI:u3,
        Reserved:u2,
        P10S:u2,
        P9S:u2,
        P8S:u2,
    }

    struct mv_6390_G1_1B_0x0A_ATU(u16){
        HM:u1,
        ATUBin:u2,
        Reserved:u1,
        AgeTime:u8,
        L2:u1,
        LA:u1,
        HASHSELECT:u2,
    }

    struct mv_6390_G1_1B_0x0B_ATU(u16){
        AB:u1,
        ATUOp:u3,
        Reserved:u1,
        MacQPri:u3,
        AgeOutV:u1,
        MemberV:u1,
        MissV:u1,
        ATUFullV:u1,
        Reserved1:u1,
        MacFPri:u3,
    }

    struct mv_6390_G1_1B_0x0C_ATU(u16){
        LAG:u1,
        PortVec:u11,
        EntryState:u4,
    }

    struct mv_6390_G1_1B_0x0D_ATU(u16){
        Byte0:u8,
        Byte1:u8
    }

    struct mv_6390_G1_1B_0x0E_ATU(u16){
        Byte2:u8,
        Byte3:u8
    }

    struct mv_6390_G1_1B_0x0F_ATU(u16){
        Byte4:u8,
        Byte5:u8
    }

    struct mv_6390_G1_1B_0x1D_ATU(u16){
        StatsBusy:u1,
        StatsOp:u2,
        Reserved:u1,
        StatsBank:u1,
        StatsPort:u5,
        StatsPointer:u5
    }

    struct mv_6390_G1_1B_0x1E_ATU(u16){
        StatsByte3:u8,
        StatsByte2:u8,
    }

    struct mv_6390_G1_1B_0x1F_ATU(u16){
        StatsByte1:u8,
        StatsByte0:u8,
    }

    struct mv_6390_G2_1C_0x05(u16){
        LoopbackFilter:u1,
        Reserved:u1,
        FlowControlMsg:u1,
        FloodBC:u1,
        Remove1PTag:u1,
        AtuAgeIntEn:u1,
        TagFlowCtl:u1,
        AutoDisable:u1,
        Reserved1:u8,
    }

    struct mv_6390_G2_1C_0x18_PHY_CMD_SETUP(u16){
        SMIBusy:u1,
        SMIFunc:u2,
        Reserved:u1,
        SMIOp:u2,
        Pointer:u10,
    }

    struct mv_6390_G2_1C_0x18_PHY_CMD(u16){
        SMIBusy:u1,
        SMIFunc:u2,
        SMIMode:u1,
        SMIOp:u2,
        DevAddr:u5,
        RegAddr:u5,
    }

    struct mv_PHY_CMD(u16){
        SMIBusy:u1,
        Reserved:u2,
        SMIMode:u1,
        SMIOp:u2,
        DevAddr:u5,
        RegAddr:u5,
    }

    struct mv_PHY_Data(u16){
        SMIData:u16,
    }

    struct mv_6390_PORT_0x00(u16){
        TxPauseEn:u1,
        RxPauseEn:u1,
        Reserved:u1,
        PhyDetect:u1,
        Link:u1,
        Duplex:u1,
        Speed:u2,
        DuplexFixed:u1,
        EEE:u1,
        TxPaused:u1,
        FlowCtl:u1,
        C_Mode:u4,
    }

    struct mv_6390_PORT_0x01(u16){
        RGMII_MODE_RX:u1,
        RGMII_MODE_TX:u1,
        FS:u1,
        AS:u1,
        MIIPM:u1,
        Reserved:u1,
        EEE:u1,
        ForceEEE:u1,
        Reserved1:u2,
        LinkValue:u1,
        ForcedLink:u1,
        DpxValue:u1,
        ForcedDpx:u1,
        SpdValue:u2,
    }

    struct mv_6390_PORT_0x04(u16){
        SA_Filtering:u2,
        EgressMode:u2,
        Header:u1,
        Snoop:u1,
        FrameMode:u2,
        VlanTunnel:u1,
        TagIfBoth:u1,
        InitialPri:u2,
        EgressFlood:u2,
        PortState:u2,
    }

    struct mv_6390_PORT_0x05(u16){
        MessagePort:u1,
        LAGPort:u1,
        VTUPage:u1,
        LAG_ID:u5,
        FID11_4:u8,
    }

    struct mv_6390_PORT_0x06(u16){
        FID0_3:u4,
        ForceMap:u1,
        VLANTable:u11,
    }

    struct mv_6390_PORT_0x07(u16){
        DefFPri:u3,
        ForceDefaultVID:u1,
        DefaultVID:u12,
    }

    struct mv_6390_PORT_0x08(u16){
        ForceGoodFCS:u1,
        AllowBad:u1,
        JumboMode:u2,
        _8021QMode:u2,
        DiscardTagged:u1,
        DiscardUntagged:u1,
        MapDA:u1,
        ARPMirror:u1,
        EgressMonitorSource:u1,
        IngressMonitorSource:u1,
        AllowVID0:u1,
        DefQPri:u3,
    }

    struct mv_6390_TCAMP_PAGE_Op(u16) {
        TCAMBusy:u1,
        TCAMOp:u3,
        TCAMPage:u2,
        Reserved:u2,
        TCAMEntry:u8,
    }

    struct mv_6390_TCAMP_PAGE0_0x02(u16) {
        Mask7_6:u2,
        Reserved:u3,
        Mask2_0:u3,
        FrameType:u2,
        Reserved1:u3,
        SPV10_8:u3,
    }

    struct mv_6390_TCAMP_PAGE0_0x03(u16) {
        Mask7_0:u8,
        SPV7_0:u8,
    }

    struct mv_6390_TCAMP_PAGE0_0x04(u16) {
        Mask:u8,
        Reserved_PPPR:u4,
        Reserved_PVID11_8:u4,
    }

    struct mv_6390_TCAMP_PAGE0_0x05(u16) {
        Mask:u8,
        Index_PVID7_0:u8,
    }

    struct mv_6390_TCAMP_PAGE0_0x06(u16) {
        Mask:u8,
        Octet1_49_DA1:u8,
    }

    struct mv_6390_TCAMP_PAGE0_0x07(u16) {
        Mask:u8,
        Octet2_50_DA2:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x08(u16) {
        Mask:u8,
        Octet3_51_DA3:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x09(u16) {
        Mask:u8,
        Octet4_52_DA4:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x0A(u16) {
        Mask:u8,
        Octet5_53_DA5:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x0B(u16) {
        Mask:u8,
        Octet6_54_DA6:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x0C(u16) {
        Mask:u8,
        Octet7_55_SA1:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x0D(u16) {
        Mask:u8,
        Octet8_56_SA2:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x0E(u16) {
        Mask:u8,
        Octet9_57_SA3:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x0F(u16) {
        Mask:u8,
        Octet10_58_SA4:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x10(u16) {
        Mask:u8,
        Octet11_59_SA5:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x11(u16) {
        Mask:u8,
        Octet12_60_SA6:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x12(u16) {
        Mask:u8,
        Octet13_61_TAG1:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x13(u16) {
        Mask:u8,
        Octet14_62_TAG2:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x14(u16) {
        Mask:u8,
        Octet15_63_PRI_VID:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x15(u16) {
        Mask:u8,
        Octet16_64_VID:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x16(u16) {
        Mask:u8,
        Octet17_65_Type1:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x17(u16) {
        Mask:u8,
        Octet18_66_Type2:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x18(u16) {
        Mask:u8,
        Octet19_67_Data1:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x19(u16) {
        Mask:u8,
        Octet20_68_Data2:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x1A(u16) {
        Mask:u8,
        Octet21_69_Data3:u8,
    }
    struct mv_6390_TCAMP_PAGE0_0x1B(u16) {
        Mask:u8,
        Octet22_70_Data4:u8,
    }
    struct mv_6390_TCAMP_PAGE1_Data(u16) {
        Mask:u8,
        Octet:u8,
    }
    struct mv_6390_TCAMP_PAGE2_0x02(u16) {
        Continue:u1,
        Int:u1,
        IncTcamCtr:u1,
        VidOverride:u1,
        VIDData:u12,
    }
    struct mv_6390_TCAMP_PAGE2_0x03(u16) {
        NextIndex_FlowID:u8,
        QPRI_Override:u1,
        QPRIData:u3,
        FPRIOverride:u1,
        FPRIData:u3,
    }
    struct mv_6390_TCAMP_PAGE2_0x05(u16) {
        Reserved:u4,
        SourcePortFilter:u1,
        DPVData:u11,
    }
    struct mv_6390_TCAMP_PAGE2_0x06(u16) {
        DPVMode:u2,
        ColorMode:u2,
        VTUPageOverride:u1,
        VTUPageData:u1,
        UnknownFiltering:u2,
        Reserved:u2,
        EgressActionPointer:u6,
    }
    struct mv_6390_TCAMP_PAGE2_0x07(u16) {
        LoadBalanceOverride:u1,
        LoadBalanceData:u3,
        Reserved:u5,
        DSCPOverride:u1,
        DSCPData:u6,
    }

    struct mv_6390_TCAMP_PAGE2_0x08(u16) {
        FActionOverride:u1,
        FActionData:u15,
    }

    struct mv_6390_TCAMP_PAGE2_0x1C(u16) {
        Reserved:u11,
        DebugPort:u5,
    }
    struct mv_6390_TCAMP_PAGE2_0x1F(u16) {
        HighHit:u8,
        LowHit:u8,
    }
    struct mv_6390_TCAMP_PAGE3_Op(u16) {
        TCAMBusy:u1,
        TCAMOp:u3,
        TCAMPage:u2,
        Reserved:u2,
        AMEntry:u8,
    }
    struct mv_6390_TCAMP_PAGE3_0x01(u16) {
        Reserved:u11,
        EgressPort:u5,
    }
    struct mv_6390_TCAMP_PAGE3_0x02(u16) {
        Reserved:u1,
        FrameModeOverride:u1,
        FrameModeData:u2,
        Reserved1:u1,
        TagModeOverride:u1,
        TagModeData:u2,
        Reserved2:u2,
        DAMode:u2,
        Reserved3:u1,
        SAMode:u3,
    }
    struct mv_6390_TCAMP_PAGE3_0x03(u16) {
        Reserved:u1,
        EgressVIDOverride:u1,
        EgressVIDMode:u2,
        EgressVIDData:u12,
    }
    struct mv_6390_TCAMP_PAGE3_0x04(u16) {
        DSCPMode:u2,
        EgressDSCP:u6,
        Reserved:u1,
        EgressFPRIOverride:u1,
        EgressFPRIMode:u2,
        EgressCFIData:u1,
        EgressFPRIData:u3,
    }
    struct mv_6390_TCAMP_PAGE3_0x1F(u16) {
        Reserved:u2,
        LastNon_ZeroHit:u6,
        Reserved1:u2,
        LastHit:u6,
    }
}

#[macro_export]
macro_rules! mdio_write {
    ($wfunc:ident, $dev:expr, $reg:expr, $val:expr) => (
        {
            $wfunc($dev as u16,$reg as u16, $val as u16)
        }
        );
}
#[macro_export]
macro_rules! mdio_write_swap {
    ($wfunc:ident, $dev:expr, $reg:expr, $val:expr) => (
        {
            let mut val= $val as u16;
            //swap ret
            if cfg!(target_endian = "big") {
                val = val.to_le()
            } else {
                val = val.to_be()
            }
            $wfunc($dev as u16,$reg as u16, val as u16)
        }
        );
}

#[test]
fn test_write_swap()
{
    fn a_write(dev:u16, reg:u16, val:u16 )->i32{
        (dev+reg+val) as i32
    }
    let ret = mdio_write_swap![a_write, 1, 2, 3];
    println!("ret={:?}",ret);
}
#[test]
fn test_write()
{
    fn a_write(dev:u16, reg:u16, val:u16 )->i32 {
        (dev+reg+val) as i32
    }
    let ret = mdio_write![a_write, 1, 2, 3];
    println!("ret={:?}",ret);
}
#[macro_export]
macro_rules! mdio_read {
    ($t:ty, $func:ident, $dev:expr, $reg:expr, &mut $ret:ident) => (
        {
            let retv = $func($dev as u16,$reg as u16, &mut $ret);
            <$t>::try_from(retv)
        }
    );
}
#[macro_export]
macro_rules! mdio_read_swap {
    ($t:ty,$func:ident, $dev:expr, $reg:expr,&mut $ret:ident) => (
        {
            let mut ret = $func($dev as u16,$reg as u16, &mut $ret);
            //swap ret
            if cfg!(target_endian = "big") {
                ret = ret.to_le()
            } else {
                ret = ret.to_be()
            }
            <$t>::try_from(ret as u16)
        }
    );
}

#[test]
fn test_read_swap()
{
    fn a_read(dev:u16, reg:u16, ret:&mut i32 )->u16 {
        *ret=0;
        dev+reg 
    }
    let mut ret=-1i32;
    let retv = mdio_read_swap![mv_6390_PORT_0x08, a_read, 1, 2, &mut ret];
    println!("ret={:?},{}",retv, ret);
}
#[test]
fn test_read()
{
    fn a_read(dev:u16, reg:u16, ret:& mut i32)->u16 {
        *ret=-1;
        dev+reg 
    }
    let mut ret=0i32;
    let retv = mdio_read![mv_6390_PORT_0x08, a_read, 1, 2, &mut ret];
    println!("ret={:?},{}",retv, ret);
}
