#[derive(Debug)]
pub enum MeterfeederErr {
    GenericError(String),
    ErrorCreatingDeviceInfoList(String),
    ErrorGettingDeviceInfoList,
    NoGeneratorConnected,
    FailedToConnectToDevice(String),
    FailedToSetLatencyTime(String),
    FailedToSetInOutPacketSize(String),
    FailedToSetTimeoutTime(String),
    UnableToFindDeviceByHandle(String),
    DeviceFailedToStopTransmitting(String),
    DeviceFailedToStartTransmitting(String),
    ErrorReadingEntropy(String),
}