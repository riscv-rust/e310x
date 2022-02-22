/// Helper traits for SPI pins
use core::ops::Deref;
use e310x::{qspi0, QSPI0, QSPI1, QSPI2};

#[doc(hidden)]
pub trait SpiX: Deref<Target = qspi0::RegisterBlock> + private::Sealed {}
impl SpiX for QSPI0 {}
impl SpiX for QSPI1 {}
impl SpiX for QSPI2 {}

/// SPI pins - DO NOT IMPLEMENT THIS TRAIT
///
/// This trait is implemented for pin tuples (), (MOSI, MISO, SCK) and (MOSI, MISO, SCK, CS)
/// and combinations without MOSI/MISO
pub trait Pins<SPI>: private::Sealed {
    #[doc(hidden)]
    const CS_INDEX: Option<u32>;
}

/// SPI pins without CS - DO NOT IMPLEMENT THIS TRAIT
///
/// This trait is implemented for pin tuples (), (MOSI, MISO, SCK) only without CS pin
/// and combinations without MOSI/MISO
pub trait PinsNoCS<SPI>: Pins<SPI> {}

/// SPI Chip Select pin - DO NOT IMPLEMENT THIS TRAIT
///
/// This trait is implemented for chip select pins only
pub trait PinCS<SPI>: private::Sealed {
    #[doc(hidden)]
    const CS_INDEX: u32;
}

/* SPI0 pins */
impl Pins<QSPI0> for () {
    const CS_INDEX: Option<u32> = Some(0);
}

/* SPI1 pins */
mod spi1_impl {
    use super::{PinCS, Pins, PinsNoCS, QSPI1};
    use crate::gpio::gpio0;
    use crate::gpio::{NoInvert, IOF0};

    type MOSI = gpio0::Pin3<IOF0<NoInvert>>;
    type MISO = gpio0::Pin4<IOF0<NoInvert>>;
    type SCK = gpio0::Pin5<IOF0<NoInvert>>;
    type CS0 = gpio0::Pin2<IOF0<NoInvert>>;
    type CS1 = gpio0::Pin8<IOF0<NoInvert>>;
    type CS2 = gpio0::Pin9<IOF0<NoInvert>>;
    type CS3 = gpio0::Pin10<IOF0<NoInvert>>;

    // ensure only the correct CS pins can be used to make SpiSharedDevice instances
    impl PinCS<QSPI1> for CS0 {
        const CS_INDEX: u32 = 0;
    }
    impl PinCS<QSPI1> for CS1 {
        const CS_INDEX: u32 = 1;
    }
    impl PinCS<QSPI1> for CS2 {
        const CS_INDEX: u32 = 2;
    }
    impl PinCS<QSPI1> for CS3 {
        const CS_INDEX: u32 = 3;
    }

    impl PinsNoCS<QSPI1> for (MOSI, MISO, SCK) {}
    impl PinsNoCS<QSPI1> for (MOSI, (), SCK) {}
    impl PinsNoCS<QSPI1> for ((), MISO, SCK) {}

    impl Pins<QSPI1> for (MOSI, MISO, SCK) {
        const CS_INDEX: Option<u32> = None;
    }
    impl Pins<QSPI1> for (MOSI, (), SCK) {
        const CS_INDEX: Option<u32> = None;
    }
    impl Pins<QSPI1> for ((), MISO, SCK) {
        const CS_INDEX: Option<u32> = None;
    }
    impl Pins<QSPI1> for (MOSI, MISO, SCK, CS0) {
        const CS_INDEX: Option<u32> = Some(0);
    }
    impl Pins<QSPI1> for (MOSI, (), SCK, CS0) {
        const CS_INDEX: Option<u32> = Some(0);
    }
    impl Pins<QSPI1> for ((), MISO, SCK, CS0) {
        const CS_INDEX: Option<u32> = Some(0);
    }
    impl Pins<QSPI1> for (MOSI, MISO, SCK, CS1) {
        const CS_INDEX: Option<u32> = Some(1);
    }
    impl Pins<QSPI1> for (MOSI, (), SCK, CS1) {
        const CS_INDEX: Option<u32> = Some(1);
    }
    impl Pins<QSPI1> for ((), MISO, SCK, CS1) {
        const CS_INDEX: Option<u32> = Some(1);
    }
    impl Pins<QSPI1> for (MOSI, MISO, SCK, CS2) {
        const CS_INDEX: Option<u32> = Some(2);
    }
    impl Pins<QSPI1> for (MOSI, (), SCK, CS2) {
        const CS_INDEX: Option<u32> = Some(2);
    }
    impl Pins<QSPI1> for ((), MISO, SCK, CS2) {
        const CS_INDEX: Option<u32> = Some(2);
    }
    impl Pins<QSPI1> for (MOSI, MISO, SCK, CS3) {
        const CS_INDEX: Option<u32> = Some(3);
    }
    impl Pins<QSPI1> for (MOSI, (), SCK, CS3) {
        const CS_INDEX: Option<u32> = Some(3);
    }
    impl Pins<QSPI1> for ((), MISO, SCK, CS3) {
        const CS_INDEX: Option<u32> = Some(3);
    }

    // seal the "private" traits
    mod spi1_private {
        use super::super::private::Sealed;
        use super::*;

        impl Sealed for CS0 {}
        impl Sealed for CS1 {}
        impl Sealed for CS2 {}
        impl Sealed for CS3 {}
        impl Sealed for (MOSI, MISO, SCK) {}
        impl Sealed for (MOSI, (), SCK) {}
        impl Sealed for ((), MISO, SCK) {}
        impl Sealed for (MOSI, MISO, SCK, CS0) {}
        impl Sealed for (MOSI, (), SCK, CS0) {}
        impl Sealed for ((), MISO, SCK, CS0) {}
        impl Sealed for (MOSI, MISO, SCK, CS1) {}
        impl Sealed for (MOSI, (), SCK, CS1) {}
        impl Sealed for ((), MISO, SCK, CS1) {}
        impl Sealed for (MOSI, MISO, SCK, CS2) {}
        impl Sealed for (MOSI, (), SCK, CS2) {}
        impl Sealed for ((), MISO, SCK, CS2) {}
        impl Sealed for (MOSI, MISO, SCK, CS3) {}
        impl Sealed for (MOSI, (), SCK, CS3) {}
        impl Sealed for ((), MISO, SCK, CS3) {}
    }
}

/* SPI2 pins */
mod spi2_impl {
    use super::{PinCS, Pins, PinsNoCS, QSPI2};
    use crate::gpio::gpio0;
    use crate::gpio::{NoInvert, IOF0};

    type MOSI = gpio0::Pin27<IOF0<NoInvert>>;
    type MISO = gpio0::Pin28<IOF0<NoInvert>>;
    type SCK = gpio0::Pin29<IOF0<NoInvert>>;
    type CS0 = gpio0::Pin26<IOF0<NoInvert>>;

    impl PinCS<QSPI2> for CS0 {
        const CS_INDEX: u32 = 0;
    }

    impl PinsNoCS<QSPI2> for (MOSI, MISO, SCK) {}
    impl PinsNoCS<QSPI2> for (MOSI, (), SCK) {}
    impl PinsNoCS<QSPI2> for ((), MISO, SCK) {}

    impl Pins<QSPI2> for (MOSI, MISO, SCK) {
        const CS_INDEX: Option<u32> = None;
    }
    impl Pins<QSPI2> for (MOSI, (), SCK) {
        const CS_INDEX: Option<u32> = None;
    }
    impl Pins<QSPI2> for ((), MISO, SCK) {
        const CS_INDEX: Option<u32> = None;
    }
    impl Pins<QSPI2> for (MOSI, MISO, SCK, CS0) {
        const CS_INDEX: Option<u32> = Some(0);
    }
    impl Pins<QSPI2> for (MOSI, (), SCK, CS0) {
        const CS_INDEX: Option<u32> = Some(0);
    }
    impl Pins<QSPI2> for ((), MISO, SCK, CS0) {
        const CS_INDEX: Option<u32> = Some(0);
    }

    // seal the "private" traits
    mod spi2_private {
        use super::super::private::Sealed;
        use super::*;

        impl Sealed for CS0 {}
        impl Sealed for (MOSI, MISO, SCK) {}
        impl Sealed for (MOSI, (), SCK) {}
        impl Sealed for ((), MISO, SCK) {}
        impl Sealed for (MOSI, MISO, SCK, CS0) {}
        impl Sealed for (MOSI, (), SCK, CS0) {}
        impl Sealed for ((), MISO, SCK, CS0) {}
    }
}

// seal the "private" traits
mod private {
    pub trait Sealed {}

    impl Sealed for () {}

    impl Sealed for super::QSPI0 {}
    impl Sealed for super::QSPI1 {}
    impl Sealed for super::QSPI2 {}
}
