#![no_std]

use core::convert::TryInto;

use riot_wrappers::riot_sys::libc;
use riot_wrappers::riot_sys;

use riot_wrappers::riot_main;


use riot_wrappers::stdio::println;

extern crate rust_riotmodules;

riot_main!(main);

// nimble_advertiser

extern "C" {
    pub static mut nimble_riot_own_addr_type: u8;
}

unsafe extern "C" fn _on_gap_evt(_event: *mut riot_sys::ble_gap_event, _arg: *mut libc::c_void) -> libc::c_int {
    println!("GAP Event!");
    0
}

fn main() {
    
    println!("NimBLE Advertiser Example Application written in Rust!");
    
    // BLE_UUID_TYPE_16 = 16
    let uuid = riot_sys::ble_uuid16_t {
        u: riot_sys::ble_uuid_t {type_: 16},
        value: 0x1337
    };
    
    let devicename = "potato";
    
    let mut bledata : riot_sys::ble_hs_adv_fields = Default::default();

    // BLE_HS_ADV_F_DISC_GEN  = 2 
    // BLE_HS_ADV_F_BREDR_UNSUP = 4
    bledata.flags = 2 | 4;
    
    // BLE_HS_ADV_TX_PWR_LVL_AUTO =  -128
    bledata.tx_pwr_lvl = -128;
    bledata.name = devicename.as_bytes().as_ptr() as *const u8;
    bledata.name_len = devicename.len().try_into().unwrap();
    bledata.uuids16 = &uuid;
    bledata.num_uuids16 = 1;
    bledata.set_tx_pwr_lvl_is_present(1);
    bledata.set_name_is_complete(1);
    bledata.set_uuids16_is_complete(1);
    
    // pub const BLE_GAP_CONN_MODE_NON: u32 = 0;
    // pub const BLE_GAP_CONN_MODE_DIR: u32 = 1;
    // pub const BLE_GAP_CONN_MODE_UND: u32 = 2;
    // pub const BLE_GAP_DISC_MODE_NON: u32 = 0;
    // pub const BLE_GAP_DISC_MODE_LTD: u32 = 1;
    // pub const BLE_GAP_DISC_MODE_GEN: u32 = 2;
    let mut parameters : riot_sys::ble_gap_adv_params = Default::default();

    parameters.conn_mode = 2;
    parameters.disc_mode = 2;
    parameters.itvl_min = 160;
    parameters.itvl_max = 160;
    parameters.channel_map = 0;
    parameters.filter_policy = 0;
    parameters.set_high_duty_cycle(0);
    
    unsafe {
        let _rc = riot_sys::ble_gap_adv_set_fields(&bledata);
        let _rc2 = riot_sys::ble_gap_adv_start(nimble_riot_own_addr_type, core::ptr::null(), i32::MAX ,
                               &parameters, Some(_on_gap_evt), core::ptr::null_mut());
    };
}
