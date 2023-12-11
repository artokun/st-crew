bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImmobileReason: u16 {
        const PAUSED = 0b00000001;
        const NO_ENERGY = 0b00000010;
        const DISREPAIR = 0b00000100;
    }
}
