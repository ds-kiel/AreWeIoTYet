//! An extremely simple libtock-rs example. Just prints out a message
//! using the Console capsule, then terminates.

#![no_main]
#![no_std]



use core::fmt::Write;
use libtock::console::Console;
use libtock::runtime::{set_main, stack_size};

mod ble;
use ble::{GapConst, GapFlags, AdvData, BLEConst, BLE};

set_main! {main}
stack_size! {0x1000}


// example file

const DEVICE_NAME_SIZE : u8 = 6;
const UUIDS_SIZE : u8 = 4;
const MANUFACTURER_DATA_SIZE : u8 = 2;
const FAKE_TEMPERATURE_DATA_SIZE : u8 = 2;


const ADVERTISING_INTERVAL_MS : u16 = 300;
const DEVICE_NAME: [u8; 6]  = [0x54, 0x6f, 0x63, 0x6b, 0x4f ,0x53]; // TockOS
const UUIDS: [u16; 2] = [0x1800, 0x1809];
const MANUFACTURER_DATA: [u8; 2] = [0x13, 0x37];
const FAKE_TEMPERATURE_DATA: [u8; 2] = [0x00, 0x00];





fn main() {
    
    let mut adv_data_buf: [u8; GapConst::ADV_DATA_MAX_SIZE] = [0; GapConst::ADV_DATA_MAX_SIZE];
    
    writeln!(Console::writer(), " - Initializing BLE... {:?}", DEVICE_NAME).unwrap();
    let mut adv_data : AdvData = AdvData::gap_adv_data_new(&mut adv_data_buf, GapConst::ADV_DATA_MAX_SIZE);
    
    adv_data.gap_add_flags(GapFlags::LE_GENERAL_DISCOVERABLE | GapFlags::BREDR_NOT_SUPPORTED);
    
    // configure device name as TockOS
    writeln!(Console::writer(), " - Setting the device name... {:?}", DEVICE_NAME).unwrap();
    let err = adv_data.gap_add_device_name(Some(&DEVICE_NAME), DEVICE_NAME_SIZE);
    if err < 0 {
        writeln!(Console::writer(), "ble_advertise_name, error: {}", err).unwrap();
    }
    
    // configure list of UUIDs
    writeln!(Console::writer(), " - Setting the device UUID...").unwrap();
    let err = adv_data.gap_add_service_uuid16(Some(&UUIDS), UUIDS_SIZE);
    if err < 0 {
        writeln!(Console::writer(), "ble_advertise_uuid16, error: {}", err).unwrap();
    }
    
    // configure manufacturer data
    writeln!(Console::writer(), " - Setting the manufacturer  data...").unwrap();
    let err = adv_data.gap_add_manufacturer_specific_data(Some(&MANUFACTURER_DATA), MANUFACTURER_DATA_SIZE);
    if err < 0 {
        writeln!(Console::writer(), "ble_advertise_manufacturer_specific_data, error: {}", err).unwrap();
    }
    
    // configure service data
    writeln!(Console::writer(), " - Setting the service data...").unwrap();
    let err = adv_data.gap_add_service_data(UUIDS[1], &FAKE_TEMPERATURE_DATA, FAKE_TEMPERATURE_DATA_SIZE);
    if err < 0 {
        writeln!(Console::writer(), "ble_advertise_service_data, error: {}", err).unwrap();
    }
    
    // https://github.com/tock/libtock-c/blob/master/examples/ble_advertising/main.c
    // https://github.com/tock/libtock-c/blob/master/libtock/ble.h
    // https://github.com/tock/libtock-c/blob/master/libtock/ble.c
    
    
    writeln!(Console::writer(), " - Begin advertising! Offset: {} of {:?}", adv_data.offset, adv_data.buf).unwrap();
    
    
    let err = BLE::advertise(BLEConst::ADV_NONCONN_IND, &adv_data.buf[..adv_data.offset], ADVERTISING_INTERVAL_MS, 2500);
    writeln!(Console::writer(), "=> {:?}", err).unwrap();
    
    // BLE Already busy!
    let err = BLE::advertise(BLEConst::ADV_NONCONN_IND, &adv_data.buf[..adv_data.offset], ADVERTISING_INTERVAL_MS, 0);
    writeln!(Console::writer(), "=> {:?}", err).unwrap();
    
    
    
    writeln!(Console::writer(), "Done advertising").unwrap();
    
}