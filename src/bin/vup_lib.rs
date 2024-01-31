use anyhow::anyhow;
use std::io;

use hidapi::{DeviceInfo, HidApi};

enum VupCmd {
    GetVersion,
}

impl Bytes for VupCmd {
    fn to_bytes(&self) -> Vec<u8> {
        return vec![1, 2];
    }
}

trait Bytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl PartialEq for dyn Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

trait CmdRsp {
    /// `Rsp` is the response data type
    type Rsp;

    fn get_rsp(rsp_frame: &[u8]) -> Self::Rsp;

    /// return true if it is the last frame
    fn is_last(rsp_frame: &[u8]) -> bool;

    fn need_response(rsp_frame: &[u8]) -> bool;

    fn cmd(rsp_frame: &[u8]) -> impl Bytes;
}

struct CmdGetVersion;

impl CmdRsp for CmdGetVersion {
    type Rsp = bool;

    fn get_rsp(rsp_frame: &[u8]) -> Self::Rsp {
        false
    }

    fn is_last(rsp_frame: &[u8]) -> bool {
        false
    }

    fn need_response(rsp_frame: &[u8]) -> bool {
        false
    }

    fn cmd(rsp_frame: &[u8]) -> impl Bytes {
        VupCmd::GetVersion
    }
}

trait RspParse {
    type RET;
    /// parse response data with a response, return None if there is no need to return a value
    fn parse(data: &Vec<u8>) -> Option<Self::RET>;

    fn need_return() -> bool;
}

trait Transport {
    /// connect to device
    fn connect(&self) -> io::Result<()>;

    /// send command to device, return actual sent length
    fn send(&self, cmd: &[u8]) -> io::Result<i32>;

    /// receive response from device
    fn recv(&self) -> io::Result<Vec<u8>>;

    /// close the transport
    fn close(&self) -> io::Result<()>;
}

/// hid transport with vid and pid
struct HidTransport {
    pid: u16,
    vid: u16,

    connected: bool,
}

impl Transport for HidTransport {
    fn send(&self, cmd: &[u8]) -> io::Result<i32> {
        todo!()
    }

    fn recv(&self) -> io::Result<Vec<u8>> {
        todo!()
    }

    fn close(&self) -> io::Result<()> {
        todo!()
    }

    fn connect(&self) -> io::Result<()> {
        todo!()
    }
}

impl HidTransport {
    fn new(vid: u16, pid: u16) -> Self {
        Self {
            pid,
            vid,
            connected: false,
        }
    }

    fn find_device(&self) -> Result<DeviceInfo, anyhow::Error> {
        let api = HidApi::new()?;
        for device in api.device_list() {
            println!("0x{:04X}:0x{:04X}", device.vendor_id(), device.product_id());

            if device.vendor_id() == self.vid && device.product_id() == self.pid {
                return Ok(device.clone());
            }
        }

        Err(anyhow!("device not found"))
    }
}

const VANCH_VID: u16 = 0x0483;
const VANCH_PID: u16 = 0x5750;

fn main() {
    println!("Printing all available hid devices:");

    let api = HidApi::new().unwrap();
    // get first
    let di = api
        .device_list()
        .take_while(|&x| x.product_id() == VANCH_PID || x.vendor_id() == VANCH_VID)
        .next()
        .expect("no device found");

    let dev = di.open_device(&api).unwrap();

    // get version command
    let _ = dev.write(vec![1, 2, 3, 4].as_ref());
}
