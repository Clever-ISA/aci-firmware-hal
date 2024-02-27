use aci_registry::{DeviceClass, DeviceVendor, ProductId, SubclassId};

use crate::connection::DevicePort;

pub trait Device {
    /// Specifies the class/subclass of the device
    ///
    /// The result of this function may be chaced by the HAL implementation, changing the result may result in unexpected (but not undefined) behaviour
    fn device_class(&self) -> (DeviceClass, SubclassId);
    /// Specifies the class/subclass of the device
    ///
    /// The result of this function may be chaced by the HAL implementation, changing the result may result in unexpected (but not undefined) behaviour
    fn device_product(&self) -> (DeviceVendor, ProductId);

    /// Handles a read from the device.
    fn read(&self, addr: u16 /* actually a u12 */, port: &dyn DevicePort) -> u32;
    fn write(&self, addr: u16 /* actually a u12 */, val: u32, port: &dyn DevicePort);
}
