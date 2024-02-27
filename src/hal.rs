use crate::{connection::DevicePort, device::Device};

pub trait Hal: 'static {
    fn max_devices(&self) -> usize;
    fn attach(&self, port: usize, dev: &'static dyn Device) -> &dyn DevicePort;
    fn deattach(&self, port: usize);
}
