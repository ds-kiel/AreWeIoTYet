#![no_std]

use libtock_platform as platform;
//use libtock_platform::allow_ro::AllowRo;
use libtock_platform::share;
use libtock_platform::allow_rw::AllowRw;
use libtock_platform::subscribe::Subscribe;
use libtock_platform::{DefaultConfig, ErrorCode, Syscalls};

use core::cell::Cell;

use core::fmt::Write;
use libtock_console as console;
type Console = console::Console<libtock_runtime::TockSyscalls>;
use libtock_alarm as alarm;
type Alarm = alarm::Alarm<libtock_runtime::TockSyscalls>;
use alarm::Milliseconds;

// simple-ble/include/gap.h

#[non_exhaustive]
pub struct GapConst;

impl GapConst {
    pub const ADV_DATA_MAX_SIZE : usize = 31;
    pub const ADV_A_SIZE : u32 = 6;

    pub const ADV_IND : u32 = 0x00;
    pub const ADV_DIRECT_IND : u32 = 0x01;
    pub const ADV_NON_CONN_IND : u32 = 0x02;
    pub const ADV_SCAN_IND : u32 = 0x06; 
}

#[non_exhaustive]
pub struct GapAdvertisementData;

impl GapAdvertisementData {
    pub const GAP_FLAGS: u8 = 0x01; /* Flags, see enum below */
    pub const GAP_INCOMPLETE_LIST_16BIT_SERVICE_IDS: u8 = 0x02; /* Incomplete list of 16-bit Service IDs. */
    pub const GAP_COMPLETE_LIST_16BIT_SERVICE_IDS: u8 = 0x03; /* Complete list of 16-bit Service IDs. */
    pub const GAP_INCOMPLETE_LIST_32BIT_SERVICE_IDS: u8 = 0x04;  /* Incomplete list of 32-bit Service IDs (not relevant for Bluetooth 4.0). */
    pub const GAP_COMPLETE_LIST_32BIT_SERVICE_IDS: u8 = 0x05; /* Complete list of 32-bit Service IDs (not relevant for Bluetooth 4.0). */
    pub const GAP_INCOMPLETE_LIST_128BIT_SERVICE_IDS: u8 = 0x06; /* Incomplete list of 128-bit Service IDs. */
    pub const GAP_COMPLETE_LIST_128BIT_SERVICE_IDS: u8 = 0x07; /* Complete list of 128-bit Service IDs. */
    pub const GAP_SHORTENED_LOCAL_NAME: u8 = 0x08; /* Shortened Local Name. */
    pub const GAP_COMPLETE_LOCAL_NAME: u8 = 0x09; /* Complete Local Name. */
    pub const GAP_TX_POWER_LEVEL: u8 = 0x0A; /* TX Power Level (in dBm). */
    pub const GAP_DEVICE_ID: u8 = 0x10; /* Device ID. */
    pub const GAP_SLAVE_CONNECTION_INTERVAL_RANGE: u8 = 0x12; /* Slave Connection Interval Range. */
    pub const GAP_LIST_128BIT_SOLICITATION_IDS: u8 = 0x15; /* List of 128 bit service UUIDs the device is looking for. */
    pub const GAP_SERVICE_DATA: u8 = 0x16; /* Service Data. */
    pub const GAP_APPEARANCE: u8 = 0x19; /* Appearance */
    pub const GAP_ADVERTISING_INTERVAL: u8 = 0x1A; /* Advertising Interval. */
    pub const GAP_MANUFACTURER_SPECIFIC_DATA: u8 = 0xFF; /* Manufacturer Specific Data. */
}

#[non_exhaustive]
pub struct GapFlags(u8);

impl GapFlags {
    pub const LE_LIMITED_DISCOVERABLE: u8 = 0x01; /* Peripheral device is discoverable for a limited period of time. */
    pub const LE_GENERAL_DISCOVERABLE: u8 = 0x02; /* Peripheral device is discoverable at any moment. */
    pub const BREDR_NOT_SUPPORTED: u8 = 0x04; /* Peripheral device is LE only. */
    pub const SIMULTANEOUS_LE_BREDR_C: u8 = 0x08; /* Not relevant - central mode only. */
    pub const SIMULTANEOUS_LE_BREDR_H: u8 = 0x10; /* Not relevant - central mode only. */
}

pub struct AdvData<'a> {
    pub buf: &'a mut [u8],
    pub offset: usize, 
    pub capacity: usize
}

// simple-ble/include/gap.c
impl<'a> AdvData<'a> {
    // the original code for some reason creates the structure only on the stack
    pub fn gap_adv_data_new(buf: &mut [u8], len: usize) -> AdvData {
        AdvData { 
                buf: buf,
                offset: 0,
                capacity: len
         }
    }
    
    pub fn gap_add_flags(&mut self, flags : u8) -> i32 {
        let new_len = self.offset + 3;
        if new_len <= self.capacity {
            self.buf[self.offset] = 2;
            self.buf[self.offset+1] = GapAdvertisementData::GAP_FLAGS;
            self.buf[self.offset+2] = flags;
            self.offset = new_len;
            // always writes 3 bytes
            return 3;
        }
        -1
    }

    // internal helper to configure gap data in the advertisement
    //
    // header - gap data header
    // data   - buffer of data configure in the advertisement
    // len    - length of data buffer
    pub fn gap_add_adv_data_field(&mut self, typee : u8, data : &[u8], data_len : u8) -> i32 {
        // make room for gap data header: length and gap_type
        let new_length = 2 + usize::from(data_len) + self.offset;
        if new_length > self.capacity {
            return -1;
        }
        
        self.buf[self.offset] = data_len + 1;
        self.buf[self.offset+1] = typee;
        let dlen = usize::from(data_len);
        //writeln!(Console::writer(), "Writing [{}:{}] from [0:{}] - {} len, {} len", self.offset + 2, self.offset + 2+dlen, dlen, self.buf.len(), data.len()).unwrap();
        self.buf[self.offset + 2..][..dlen].copy_from_slice(&data[..dlen]);
        self.offset = new_length;
        2 + i32::from(data_len)
    }
    
    pub fn gap_add_device_name(&mut self, device_name : Option<&[u8]>, len: u8) -> i32 {
        if let Some(device_name) = device_name {
            return self.gap_add_adv_data_field(GapAdvertisementData::GAP_COMPLETE_LOCAL_NAME, device_name, len);
        }else {
            return -1;
        }
    }
    
    
    pub fn gap_add_service_uuid16(&mut self, uuid16 : Option<&[u16]>, len: u8) -> i32 {
        if let Some(uuid16) = uuid16 {
            // The C code here also does an endian-unsafe cast
            unsafe {
                let (_, a16, _) = uuid16.align_to::<u8>();
                //writeln!(Console::writer(), "Converting {:?} to {:?}", uuid16, a16).unwrap();
                return self.gap_add_adv_data_field(GapAdvertisementData::GAP_COMPLETE_LIST_16BIT_SERVICE_IDS, a16, len);
            }
            
        }else {
            return -1;
        }
    }
    
    pub fn gap_add_manufacturer_specific_data(&mut self, data : Option<&[u8]>, len: u8) -> i32 {
        if let Some(data) = data {
            return self.gap_add_adv_data_field(GapAdvertisementData::GAP_MANUFACTURER_SPECIFIC_DATA, data, len);
        }else {
            return -1;
        }
    }
    
    pub fn gap_add_service_data(&mut self, uuid16 : u16, data : &[u8], len: u8) -> i32 {
        let err = self.gap_add_service_uuid16(Some(&[uuid16]), 2);
        if err < 0 {
            return err;
        }
        return self.gap_add_adv_data_field(GapAdvertisementData::GAP_SERVICE_DATA, data, len);
    }
    
}

//  libtock-c/libtock/ble.c 

#[non_exhaustive]
pub struct BLEConst;

impl BLEConst {
    pub const BLE_DRIVER_NUMBER: u32 = 0x30000;
    
    pub const BLE_ADV_START_CMD: u32 = 0;
    pub const BLE_ADV_STOP_CMD: u32 = 1;
    pub const BLE_CFG_TX_POWER_CMD: u32 = 2;
    pub const BLE_SCAN_CMD: u32 = 5;

    pub const BLE_SCAN_SUB: u32 = 0;
    
    pub const BLE_CFG_ADV_BUF_ALLOWRO: u32 = 0;
    pub const BLE_CFG_SCAN_BUF_ALLOWRW: u32 = 0;
    
    pub const ADV_IND: u32 = 0x00;
    pub const ADV_DIRECT_IND: u32 = 0x01;
    pub const ADV_NONCONN_IND: u32 = 0x02;
    pub const ADV_SCAN_IND: u32 = 0x03;
    
   
    pub const POSITIVE_10_DBM: u8 = 10;
    pub const POSITIVE_9_DBM: u8 = 9;
    pub const POSITIVE_8_DBM: u8 = 8;
    pub const POSITIVE_7_DBM: u8 = 7;
    pub const POSITIVE_6_DBM: u8 = 6;
    pub const POSITIVE_5_DBM: u8 = 5;
    pub const POSITIVE_4_DBM: u8 = 4;
    pub const POSITIVE_3_DBM: u8 = 3;
    pub const POSITIVE_2_DBM: u8 = 2;
    pub const POSITIVE_1_DBM: u8 = 1;
    pub const ZERO_DBM: u8 = 0;
    pub const NEGATIVE_1_DBM: u8 = 0xff;
    pub const NEGATIVE_2_DBM: u8 = 0xfe;
    pub const NEGATIVE_3_DBM: u8 = 0xfd;
    pub const NEGATIVE_4_DBM: u8 = 0xfc;
    pub const NEGATIVE_5_DBM: u8 = 0xfb;
    pub const NEGATIVE_6_DBM: u8 = 0xfa;
    pub const NEGATIVE_7_DBM: u8 = 0xf9;
    pub const NEGATIVE_8_DBM: u8 = 0xf8;
    pub const NEGATIVE_9_DBM: u8 = 0xf7;
    pub const NEGATIVE_10_DBM: u8 = 0xf6;
    pub const NEGATIVE_11_DBM: u8 = 0xf5;
    pub const NEGATIVE_12_DBM: u8 = 0xf4;
    pub const NEGATIVE_13_DBM: u8 = 0xf3;
    pub const NEGATIVE_14_DBM: u8 = 0xf2;
    pub const NEGATIVE_15_DBM: u8 = 0xf1;
    pub const NEGATIVE_16_DBM: u8 = 0xf0;
    pub const NEGATIVE_17_DBM: u8 = 0xef;
    pub const NEGATIVE_18_DBM: u8 = 0xee;
    pub const NEGATIVE_19_DBM: u8 = 0xed;
    pub const NEGATIVE_20_DBM: u8 = 0xec;
}

pub struct BLE_<S: Syscalls, C: Config = DefaultConfig>(S, C);


impl<S: Syscalls, C: Config> BLE_<S, C> {
    
   
    pub fn advertise(pdu_type : u32, advd : &[u8], interval: u16, time: u32) -> Result<(), ErrorCode> {
        
        let r = share::scope(|handle| {
            
            let o = S::allow_ro::<C,{ BLEConst::BLE_DRIVER_NUMBER },{ BLEConst::BLE_CFG_ADV_BUF_ALLOWRO },>(handle, advd);
            writeln!(Console::writer(), "Read Only! {:?}", o).unwrap();
            o?;
            
            let u32interval : u32 = interval.into();
            let o = S::command(BLEConst::BLE_DRIVER_NUMBER, BLEConst::BLE_ADV_START_CMD, pdu_type, u32interval).to_result();
            writeln!(Console::writer(), "Start Advertising! {:?} @ {} {} {} {}", o, BLEConst::BLE_DRIVER_NUMBER, BLEConst::BLE_ADV_START_CMD, pdu_type, u32interval).unwrap();
            o?;
            
            if time == 0 {
                loop {
                    S::yield_wait();
                }
            }else {
                Alarm::sleep_for(Milliseconds(time)).unwrap();
            }
            
            let o = S::command(BLEConst::BLE_DRIVER_NUMBER, BLEConst::BLE_ADV_STOP_CMD, 1, 0).to_result();
            writeln!(Console::writer(), "Done Advertising! {:?}", o).unwrap();
            o?;
            
            return o;
        });
        return r;
    }
    
    pub fn set_tx_power(power_level: u8) -> Result<(), ErrorCode> {
        S::command(BLEConst::BLE_DRIVER_NUMBER, BLEConst::BLE_CFG_TX_POWER_CMD, power_level.into(), 0).to_result()
    }
    
    // uint8_t *data, uint8_t max_len, subscribe_upcall callback
    pub fn passive_scan(f: &dyn Fn(&[u8], u32, u32)) -> Result<(), ErrorCode> {
        
        const BUF_SIZE : usize = 39;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        
        let mut f_status = 0;
        let mut f_length = 0;
        
        let called: Cell<Option<(u32, u32)>> = Cell::new(None);
        let r = share::scope::<
            (
                AllowRw<_, {BLEConst::BLE_DRIVER_NUMBER}, { BLEConst::BLE_CFG_SCAN_BUF_ALLOWRW }>,
                Subscribe<_, {BLEConst::BLE_DRIVER_NUMBER}, { BLEConst::BLE_SCAN_SUB }>,
            ),
            _,
            _,
        >(|handle| {
            let (allow_rw, subscribe) = handle.split();
            let o = S::allow_rw::<C, {BLEConst::BLE_DRIVER_NUMBER}, { BLEConst::BLE_CFG_SCAN_BUF_ALLOWRW }>(allow_rw, &mut buf);
            writeln!(Console::writer(), "Write Only! {:?}", o).unwrap();
            o?;
            let o = S::subscribe::<_, _, C, {BLEConst::BLE_DRIVER_NUMBER}, { BLEConst::BLE_SCAN_SUB }>(subscribe, &called);
            writeln!(Console::writer(), "Subscribe! {:?}", o).unwrap();
            o?;

            let o = S::command(BLEConst::BLE_DRIVER_NUMBER, BLEConst::BLE_SCAN_CMD, 1, 0).to_result();
            writeln!(Console::writer(), "Command! {:?}", o).unwrap();
            o?;

            loop {
                S::yield_wait();
                if let Some((status, length)) = called.get() {
                    f_status = status;
                    f_length = 0;
                    writeln!(Console::writer(), "Received! {} {}", status, length).unwrap();
                    return match status {
                        0 => Ok(()),
                        e_status => Err(e_status.try_into().unwrap_or(ErrorCode::Fail)),
                    };
                }
            }
        });
        r?;
        f(&buf, f_status, f_length);
        r
    }
   
    
 
}

/// System call configuration trait for `BLE`.
pub trait Config:
    platform::allow_ro::Config + platform::allow_rw::Config + platform::subscribe::Config
{
}
impl<T: platform::allow_ro::Config + platform::allow_rw::Config + platform::subscribe::Config>
    Config for T
{
}

// With different files / packages this could have the same name, doesn't really matter though
pub type BLE = BLE_<libtock_runtime::TockSyscalls>;