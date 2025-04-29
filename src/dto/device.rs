#[derive(Default,Debug)]
pub struct Device {
    pub name: String,
    pub device_type: String,
    pub mac: String,
    pub rssi: i16,
    pub percent: u8,
    pub signal_text: String,
    pub signal_color: String, // 'from-blue-400 to-blue-600'
    pub status: String,
    pub last: String,
}