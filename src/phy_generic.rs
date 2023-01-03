#![allow(dead_code,unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use bit_struct::*; 
use std::string::*;
bit_struct! {
   struct phy_reg_0x00(u16){
        reset:u1,
        internal_loopback:u1,
        speed_selection_13:u1,
        auto_negotiation_enable:u1,
        power_down:u1,
        isolate:u1,
        restart_auto_neg:u1,
        duplex:u1,
        collision_test:u1,
        speed_selection_6:u1,
        reserved:u6,
    }
    pub struct phy_reg_0x01(u16){
        _100base_t4_cap:u1,
        _100base_x_f_d_cap:u1,
        _100base_x_h_d_cap:u1,
        _10base_t_f_d_cap:u1,
        _10base_t_h_d_cap:u1,
        _100base_t2_f_d_cap:u1,
        _100base_t2_h_d_cap:u1,
        extended_status:u1,
        reserved:u1,
        management_frames_preamble_supression:u1,
        auto_negotiation_complete:u1,
        remote_fault:u1,
        auto_neg_ability:u1,
        link_status:u1,
        jabber_detect:u1,
        extended_cap:u1,
    }
    pub struct phy_reg_0x04(u16){
        next_page:u1,
        reserved:u1, //must be 0?
        remote_fault:u1,
        reserved1:u1,
        asym_pause:u1,
        pause_cap:u1,
        _100base_t4_cap:u1,
        _100base_tx_f_d_cap:u1,
        _100base_tx_h_d_cap:u1,
        _10base_t_f_d_cap:u1,
        _10base_t_h_d_cap:u1,
        selector_field:u5, 
    }
    pub struct phy_reg_0x09(u16){
        test_mode:u3,
        master_slave_configuration_enable:u1,
        master_slave_configuration_value:u1,
        dts:u1,
        adv_1000base_t_full_d:u1,
        adv_1000base_t_half_d:u1,
        reserved:u8,
    }
}
