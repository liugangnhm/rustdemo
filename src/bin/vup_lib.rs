fn main() {
    let cmd = CmdGetVersion::get_version();
    println!("{}", cmd);
}

struct CmdGetVersion;

impl CmdGetVersion {
    fn get_version() -> String {
        "1.0.0".to_string()
    }
}

trait RspParse {
    type RET;
    /// parse response data with a response, return None if there is no need to return a value
    fn parse(data: &Vec<u8>) -> Option<Self::RET>;

    fn need_return() -> bool;
}
