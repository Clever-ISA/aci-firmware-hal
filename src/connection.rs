/// Abstraction type for a device port. This may refer to the physical connector to the host, a physical connector to a hardware implemented bridge, or a virtual connector to a software implemented bridge.
pub trait DevicePort {
    /// Raises an interrupt to the host
    fn interrupt_host(&self);
}
