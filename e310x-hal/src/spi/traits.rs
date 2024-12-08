/// Helper traits for SPI pins
use core::ops::Deref;
use e310x::{qspi0, Qspi0, Qspi1, Qspi2};

#[doc(hidden)]
pub trait SpiX: Deref<Target = qspi0::RegisterBlock> + private::Sealed {}

/// SPI pins
///
/// This trait is implemented for pin tuples (), (MOSI, MISO, SCK) and (MOSI, MISO, SCK, CS)
/// and combinations without MOSI/MISO
pub trait Pins<SPI>: private::Sealed {
    /// Chip select index associated with this set of pins (if any)
    const CS_INDEX: Option<u32>;
}

/// Full SPI pins
///
/// This trait is implemented for pin tuples (MOSI, MISO, SCK) and (MOSI, MISO, SCK, CS).
/// All variants must include MOSI, MISO, and SCK pins.
pub trait PinsFull<SPI>: Pins<SPI> {}

/// SPI pins without CS
///
/// This trait is implemented for pin tuples (), (MOSI, MISO, SCK) only without CS pin
/// and combinations without MOSI/MISO (i.e., Pins::CS_INDEX is None)
pub trait PinsNoCS<SPI>: Pins<SPI> {}

/// SPI Chip Select pin
///
/// This trait is implemented for chip select pins only
pub trait PinCS<SPI>: private::Sealed {
    #[doc(hidden)]
    const CS_INDEX: u32;
}

/* SPI0 pins */
mod spi0_impl {
    use super::{Pins, Qspi0, SpiX};

    impl SpiX for Qspi0 {}

    impl Pins<Qspi0> for () {
        const CS_INDEX: Option<u32> = Some(0);
    }
}

/* SPI1 pins */
mod spi1_impl {
    use super::{PinCS, Pins, PinsFull, PinsNoCS, Qspi1, SpiX};
    use crate::gpio::{gpio0, NoInvert, IOF0};

    type Mosi = gpio0::Pin3<IOF0<NoInvert>>;
    type Miso = gpio0::Pin4<IOF0<NoInvert>>;
    type Sck = gpio0::Pin5<IOF0<NoInvert>>;
    type Cs0 = gpio0::Pin2<IOF0<NoInvert>>;
    type Cs1 = gpio0::Pin8<IOF0<NoInvert>>;
    type Cs2 = gpio0::Pin9<IOF0<NoInvert>>;
    type Cs3 = gpio0::Pin10<IOF0<NoInvert>>;

    impl SpiX for Qspi1 {}

    impl PinCS<Qspi1> for Cs0 {
        const CS_INDEX: u32 = 0;
    }
    impl PinCS<Qspi1> for Cs1 {
        const CS_INDEX: u32 = 1;
    }
    impl PinCS<Qspi1> for Cs2 {
        const CS_INDEX: u32 = 2;
    }
    impl PinCS<Qspi1> for Cs3 {
        const CS_INDEX: u32 = 3;
    }

    impl PinsNoCS<Qspi1> for (Mosi, Miso, Sck) {}
    impl PinsNoCS<Qspi1> for (Mosi, (), Sck) {}
    impl PinsNoCS<Qspi1> for ((), Miso, Sck) {}

    impl PinsFull<Qspi1> for (Mosi, Miso, Sck) {}
    impl PinsFull<Qspi1> for (Mosi, Miso, Sck, Cs0) {}
    impl PinsFull<Qspi1> for (Mosi, Miso, Sck, Cs1) {}
    impl PinsFull<Qspi1> for (Mosi, Miso, Sck, Cs2) {}
    impl PinsFull<Qspi1> for (Mosi, Miso, Sck, Cs3) {}

    impl Pins<Qspi1> for (Mosi, Miso, Sck) {
        const CS_INDEX: Option<u32> = None;
    }
    impl Pins<Qspi1> for (Mosi, (), Sck) {
        const CS_INDEX: Option<u32> = None;
    }
    impl Pins<Qspi1> for ((), Miso, Sck) {
        const CS_INDEX: Option<u32> = None;
    }
    impl Pins<Qspi1> for (Mosi, Miso, Sck, Cs0) {
        const CS_INDEX: Option<u32> = Some(0);
    }
    impl Pins<Qspi1> for (Mosi, (), Sck, Cs0) {
        const CS_INDEX: Option<u32> = Some(0);
    }
    impl Pins<Qspi1> for ((), Miso, Sck, Cs0) {
        const CS_INDEX: Option<u32> = Some(0);
    }
    impl Pins<Qspi1> for (Mosi, Miso, Sck, Cs1) {
        const CS_INDEX: Option<u32> = Some(1);
    }
    impl Pins<Qspi1> for (Mosi, (), Sck, Cs1) {
        const CS_INDEX: Option<u32> = Some(1);
    }
    impl Pins<Qspi1> for ((), Miso, Sck, Cs1) {
        const CS_INDEX: Option<u32> = Some(1);
    }
    impl Pins<Qspi1> for (Mosi, Miso, Sck, Cs2) {
        const CS_INDEX: Option<u32> = Some(2);
    }
    impl Pins<Qspi1> for (Mosi, (), Sck, Cs2) {
        const CS_INDEX: Option<u32> = Some(2);
    }
    impl Pins<Qspi1> for ((), Miso, Sck, Cs2) {
        const CS_INDEX: Option<u32> = Some(2);
    }
    impl Pins<Qspi1> for (Mosi, Miso, Sck, Cs3) {
        const CS_INDEX: Option<u32> = Some(3);
    }
    impl Pins<Qspi1> for (Mosi, (), Sck, Cs3) {
        const CS_INDEX: Option<u32> = Some(3);
    }
    impl Pins<Qspi1> for ((), Miso, Sck, Cs3) {
        const CS_INDEX: Option<u32> = Some(3);
    }

    // seal the "private" traits
    mod spi1_private {
        use super::super::private::Sealed;
        use super::*;

        impl Sealed for Cs0 {}
        impl Sealed for Cs1 {}
        impl Sealed for Cs2 {}
        impl Sealed for Cs3 {}
        impl Sealed for (Mosi, Miso, Sck) {}
        impl Sealed for (Mosi, (), Sck) {}
        impl Sealed for ((), Miso, Sck) {}
        impl Sealed for (Mosi, Miso, Sck, Cs0) {}
        impl Sealed for (Mosi, (), Sck, Cs0) {}
        impl Sealed for ((), Miso, Sck, Cs0) {}
        impl Sealed for (Mosi, Miso, Sck, Cs1) {}
        impl Sealed for (Mosi, (), Sck, Cs1) {}
        impl Sealed for ((), Miso, Sck, Cs1) {}
        impl Sealed for (Mosi, Miso, Sck, Cs2) {}
        impl Sealed for (Mosi, (), Sck, Cs2) {}
        impl Sealed for ((), Miso, Sck, Cs2) {}
        impl Sealed for (Mosi, Miso, Sck, Cs3) {}
        impl Sealed for (Mosi, (), Sck, Cs3) {}
        impl Sealed for ((), Miso, Sck, Cs3) {}
    }
}

/* SPI2 pins */
mod spi2_impl {
    use super::{PinCS, Pins, PinsFull, PinsNoCS, Qspi2, SpiX};
    use crate::gpio::{gpio0, NoInvert, IOF0};

    type Mosi = gpio0::Pin27<IOF0<NoInvert>>;
    type Miso = gpio0::Pin28<IOF0<NoInvert>>;
    type Sck = gpio0::Pin29<IOF0<NoInvert>>;
    type Cs0 = gpio0::Pin26<IOF0<NoInvert>>;

    impl SpiX for Qspi2 {}

    impl PinCS<Qspi2> for Cs0 {
        const CS_INDEX: u32 = 0;
    }

    impl PinsNoCS<Qspi2> for (Mosi, Miso, Sck) {}
    impl PinsNoCS<Qspi2> for (Mosi, (), Sck) {}
    impl PinsNoCS<Qspi2> for ((), Miso, Sck) {}

    impl PinsFull<Qspi2> for (Mosi, Miso, Sck) {}
    impl PinsFull<Qspi2> for (Mosi, Miso, Sck, Cs0) {}

    impl Pins<Qspi2> for (Mosi, Miso, Sck) {
        const CS_INDEX: Option<u32> = None;
    }
    impl Pins<Qspi2> for (Mosi, (), Sck) {
        const CS_INDEX: Option<u32> = None;
    }
    impl Pins<Qspi2> for ((), Miso, Sck) {
        const CS_INDEX: Option<u32> = None;
    }
    impl Pins<Qspi2> for (Mosi, Miso, Sck, Cs0) {
        const CS_INDEX: Option<u32> = Some(0);
    }
    impl Pins<Qspi2> for (Mosi, (), Sck, Cs0) {
        const CS_INDEX: Option<u32> = Some(0);
    }
    impl Pins<Qspi2> for ((), Miso, Sck, Cs0) {
        const CS_INDEX: Option<u32> = Some(0);
    }

    // seal the "private" traits
    mod spi2_private {
        use super::super::private::Sealed;
        use super::*;

        impl Sealed for Cs0 {}
        impl Sealed for (Mosi, Miso, Sck) {}
        impl Sealed for (Mosi, (), Sck) {}
        impl Sealed for ((), Miso, Sck) {}
        impl Sealed for (Mosi, Miso, Sck, Cs0) {}
        impl Sealed for (Mosi, (), Sck, Cs0) {}
        impl Sealed for ((), Miso, Sck, Cs0) {}
    }
}

// seal the "private" traits
mod private {
    pub trait Sealed {}

    impl Sealed for () {}

    impl Sealed for super::Qspi0 {}
    impl Sealed for super::Qspi1 {}
    impl Sealed for super::Qspi2 {}
}
