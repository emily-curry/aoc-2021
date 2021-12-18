pub trait BitPacketVersion {
    fn get_version(&self) -> Option<u8>;

    fn sum_version(&self) -> u64;
}
