#![no_std]

use core::convert::TryInto;

use riot_wrappers::riot_sys::libc;
use riot_wrappers::riot_sys;

use riot_wrappers::riot_main;

use riot_wrappers::shell;

use riot_wrappers::stdio::println;

use riot_wrappers::cstr::cstr;

extern crate rust_riotmodules;

riot_main!(main);

// nimble_scanner

extern "C" {
    pub static mut nimble_riot_own_addr_type: u8;
}


pub struct NimpleScannerConfig {
    pub itvl_ms : u16, 
    pub win_ms : u16,
    pub flags : u8 
}

pub struct NimpleScannerInfo {
    pub status : u8, 
    pub phy_pri : u8,
    pub phy_sec : u8,
    pub rssi : i8
}

pub struct NimbleScanner {
    pub _disc_cb: NimbleScannerCb,
    pub _scan_params : riot_sys::ble_gap_disc_params,
    pub _scan_flags : u8,
    /* duration of the scanning procedure */ 
    pub _scan_duration : i32 // BLE_HS_FOREVER
}





pub type NimbleScannerCb = Option<fn(typ :u8, addr : *const riot_sys::ble_addr_t, info : &NimpleScannerInfo , ad : *const u8, ad_len : usize )>;

unsafe extern "C" fn _on_scan_evt(event: *mut riot_sys::ble_gap_event, _arg: *mut libc::c_void) -> libc::c_int {
    
    match (*event).type_ { 
      7 => {
        let info : NimpleScannerInfo = NimpleScannerInfo { status : NimbleScanner::NIMBLE_SCANNER_COMPLETE, phy_pri : 1, phy_sec : 1, rssi :  (*event).__bindgen_anon_1.disc.rssi};
        println!("[scanner] {} {} {} {}", info.status, info.phy_pri, info.phy_sec, info.rssi);
        
        let data = core::slice::from_raw_parts((*event).__bindgen_anon_1.disc.data, (*event).__bindgen_anon_1.disc.length_data.into());
        println!("=> {:02X?}", data);
        // *const u8
        
      },
      8 => { // BLE_GAP_EVENT_DISC_COMPLETE
        println!("[scanner] scan cycle completed");
      },
      _=> {
        println!("[scanner] unknown event triggered ({})", (*event).type_);
      }
    };
    0
}


impl NimbleScanner {
    
    
        
    pub const NIMBLE_SCANNER_EXT_ADV : u8 = 0x80;
    
    pub const NIMBLE_SCANNER_PASSIVE : u8 = 0x01;
    pub const NIMBLE_SCANNER_LIMITED : u8 = 0x02;
    pub const NIMBLE_SCANNER_FILTER_DUPS : u8 = 0x04;
    pub const NIMBLE_SCANNER_PHY_1M : u8 = 0x10;
    pub const NIMBLE_SCANNER_PHY_CODED : u8 = 0x20;
    
    pub const NIMBLE_SCANNER_COMPLETE : u8 = 0; //riot_sys::BLE_HCI_ADV_DATA_STATUS_COMPLETE;
    pub const NIMBLE_SCANNER_INCOMPLETE  : u8 = 32; // riot_sys::BLE_HCI_ADV_DATA_STATUS_INCOMPLETE;
    pub const NIMBLE_SCANNER_TRUNCATED : u8 = 64; // riot_sys::BLE_HCI_ADV_DATA_STATUS_TRUNCATED;
    
    pub fn new() -> Self {
        NimbleScanner { _disc_cb: None, _scan_params: Default::default(), _scan_flags: 0, _scan_duration: i32::MAX  }
    }
    
    pub fn nimble_scanner_start(&mut self) {
        
        if (unsafe { riot_sys::ble_gap_disc_active() } == 0) {
            let res : libc::c_int;
            
            unsafe { 
                res = riot_sys::ble_gap_disc(nimble_riot_own_addr_type, self._scan_duration,
                                   &self._scan_params, Some(_on_scan_evt), core::ptr::null_mut());
            }
            
            if res != 0 {
                println!("[scanner] err: start failed ({})", res);
            }
        }
        
    }
    
    pub fn nimble_scanner_stop(&mut self) {
        if (unsafe { riot_sys::ble_gap_disc_active() } == 1) { 
            unsafe { 
                riot_sys::ble_gap_disc_cancel();
            }
        }
    }
    
    pub fn nimble_scanner_set_scan_duration(&mut self, duration_ms: i32) {
        self._scan_duration = duration_ms;
        if (unsafe { riot_sys::ble_gap_disc_active() } != 0) { 
            self.nimble_scanner_stop();
            self.nimble_scanner_start();
        }
    }
    
    pub fn nimble_scanner_is_active(&mut self) -> bool {
        unsafe { return riot_sys::ble_gap_disc_active() != 0 }
    }
    
    pub fn nimble_scanner_init(&mut self, params: &NimpleScannerConfig, disc_cb: NimbleScannerCb) {
        self._scan_params.itvl = (params.itvl_ms) * 1000 / 625;
        self._scan_params.window = (params.win_ms) * 1000 / 625;
        self._scan_params.set_passive(if (params.flags & NimbleScanner::NIMBLE_SCANNER_PASSIVE) != 0 { 1 } else { 0 });
        self._scan_params.set_limited(if (params.flags & NimbleScanner::NIMBLE_SCANNER_LIMITED) != 0 { 1 } else { 0 });
        self._scan_params.set_filter_duplicates(if (params.flags & NimbleScanner::NIMBLE_SCANNER_FILTER_DUPS) != 0 { 1 } else { 0 });
        

        self._disc_cb = disc_cb;
    }


    
}

fn do_scan(
         _stdio: &mut riot_wrappers::stdio::Stdio,
         args: riot_wrappers::shell::Args<'_>,
 ) -> i32 {

    if args.len() == 2 && args.get(1).unwrap_or("") == "help" {
        println!("usage: {} [timeout in ms]", args.get(0).unwrap_or(""));
        return 0;
    }
    let timeout = args.get(1).unwrap_or("").parse::<i32>().unwrap_or(1000);

    
    let mut scanner = NimbleScanner::new();
    
    
    let nimble_config = NimpleScannerConfig {
        itvl_ms: 30,
        win_ms: 30,
        flags: 0x10 | 0x20
    };
    
    
    scanner.nimble_scanner_init(&nimble_config, None);

    println!("Scanning for {} ms now ...", timeout);
    scanner.nimble_scanner_start();
    riot_wrappers::ztimer::Clock::msec().sleep(core::time::Duration::from_millis(timeout.try_into().unwrap()));
    scanner.nimble_scanner_stop();
    println!("Done, results:");
    
    
    return 0;
}

fn main() {
    
    println!("NimBLE Scanner Example Application written in Rust!");
    println!("Type `scan help` for more information");

    use riot_wrappers::shell::CommandList;
    let mut commands = shell::new().and(cstr!("scan"), cstr!("trigger a BLE scan"),
            | _stdio: &mut riot_wrappers::stdio::Stdio, _args: riot_wrappers::shell::Args<'_> | {do_scan(_stdio, _args)}
    );
    
    CommandList::<128>::run_forever_providing_buf(&mut commands);
}
