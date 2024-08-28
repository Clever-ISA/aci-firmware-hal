use crate::{connection::DevicePort, device::Device};

/// Abstraction for a ACI Device. An implementation represents the physical device for which the firmware is being provided.
///
/// There may be several physical designs for the device represented by the HAL:
/// * A physical device that only provides a single connector back to the host
/// * A physical bridge that provides several physical device connections to the host
/// * A software implementation of a bridge that provides several device connections to the host
pub trait Hal: 'static {
    /// Returns the maximum total number of devices that can be connected at once.
    /// Note that the Hal consumer is responsible for keeping track of which "ports" have been attached.
    ///
    /// The Value of `max_devices` is permitted to be lower than accurate - that is, a [`Hal`] can ultimately support attaching more devices than it reports as valid
    fn max_devices(&self) -> usize;
    /// Registers a device and attaches it to the specified port returning a [`DevicePort`] implementation that refers to it
    ///
    /// If `port` is above `max_devices` then the result is not specified (but the result is not undefined behaviour). It may:
    /// * Succesfully attach the device notwithstanding the invalid value for `port`
    /// * Silently fail to attach the device and return an non-existant [`DevicePort`] (One that may do nothing)
    /// * Panic
    fn attach(&self, port: usize, dev: &'static dyn Device) -> &dyn DevicePort;

    /// Detaches the device connected to `port`. If no device is attached the operation does nothing.
    ///
    /// If `port` exceeds `max_devices`:
    /// * If a device was previously succesfully attached to `port`, it is guaranteed to detach that device
    /// * Otherwise the function may panic or do nothing.
    fn deattach(&self, port: usize);
}
