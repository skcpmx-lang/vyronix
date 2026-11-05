//! Hardware collection layer. Windows uses WMI/CIM + sysinfo + battery/storage
//! interfaces; other platforms return honest "unavailable" data so the backend
//! compiles everywhere without fabricating hardware values.

pub mod battery;
pub mod monitoring;
pub mod network;
pub mod storage;
pub mod system;
pub mod windows;

#[cfg(windows)]
pub mod wmi;
