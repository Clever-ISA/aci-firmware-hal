use aci_registry::{DeviceClass, DeviceVendor, ProductId, SubclassId};

use crate::connection::DevicePort;

pub trait Device {
    /// Specifies the class/subclass of the device
    ///
    /// The result of this function may be cached by the HAL implementation, changing the result may result in unexpected (but not undefined) behaviour
    fn device_class(&self) -> (DeviceClass, SubclassId);
    /// Specifies the class/subclass of the device
    ///
    /// The result of this function may be cached by the HAL implementation, changing the result may result in unexpected (but not undefined) behaviour
    fn device_product(&self) -> (DeviceVendor, ProductId);

    /// Handles a read from the device referring to the register `addr`.
    /// Each read is of 4 bytes at once, and each `addr` refers to discrete addressible 4 byte registers - that is, it increases by 1 to step 4 bytes, rather than increasing by 4.
    ///
    /// Any result other than the read value (such as interrupts) can be communicated via `port`.
    ///
    /// ## Range of Values
    /// The first 16 registers (the identity registers) are reserved by the ACI Specification as "Identity Registers". The HAL implementation is responsible for handling them, and these values are never read.
    /// The maximum addr is `4095` as the ACI Specification defines only a 12-bit register address
    fn read(&self, addr: u16 /* actually a u12 */, port: &dyn DevicePort) -> u32;
    /// Handles a write to the device referring to the register `addr`.
    /// Each write is of 4 bytes at once, and each `addr` refers to discrete addressible 4 byte registers - that is, it increases by 1 to step 4 bytes, rather than increasing by 4.
    ///
    /// Any result other than the write (such as interrupts) can be communicated via `port`.
    ///
    /// ## Range of Values
    /// The first 16 registers (the identity registers) are reserved by the ACI Specification as "Identity Registers". The HAL implementation is responsible for handling them, and these values are never read.
    /// The maximum addr is `4095` as the ACI Specification defines only a 12-bit register address
    fn write(&self, addr: u16 /* actually a u12 */, val: u32, port: &dyn DevicePort);

    /// Polls the device for any interrupts that might be pending to report to the host.
    fn poll_interrupts(&self, port: &dyn DevicePort) {}

    /// Interrupts the devices current operation with an important pending operation.
    fn interrupt_device(&self, port: &dyn DevicePort) {}
}
