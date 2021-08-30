//! # Controller Area Network (CAN) Interface
//!

use crate::fdcan;
use crate::fdcan::message_ram;
// use crate::stm32::{self, FDCAN1, FDCAN2, FDCAN3};
use crate::rcc::Rcc;
use crate::stm32;

mod sealed {
    pub trait Sealed {}
}

/// A pair of (TX, RX) pins configured for CAN communication
pub trait Pins: sealed::Sealed {
    /// The CAN peripheral that uses these pins
    type Instance;
}

/// Implements sealed::Sealed and Pins for a (TX, RX) pair of pins associated with a CAN peripheral
/// The alternate function number can be specified after each pin name. If not specified, both
/// default to AF9.
macro_rules! pins {
    ($($PER:ident => ($tx:ident<$txaf:ident>, $rx:ident<$rxaf:ident>),)+) => {
        $(
            impl crate::can::sealed::Sealed for ($tx<crate::gpio::Alternate<$txaf>>, $rx<crate::gpio::Alternate<$rxaf>>) {}
            impl crate::can::Pins for ($tx<crate::gpio::Alternate<$txaf>>, $rx<crate::gpio::Alternate<$rxaf>>) {
                type Instance = $PER;
            }
        )+
    };
    ($($PER:ident => ($tx:ident, $rx:ident),)+) => {
        pins! { $($PER => ($tx<crate::gpio::AF9>, $rx<crate::gpio::AF9>),)+ }
    }
}

//TODO: verify correct pins
mod fdcan1 {
    use super::FdCan;
    use crate::fdcan;
    use crate::fdcan::message_ram;
    use crate::gpio::{
        gpioa::{PA11, PA12},
        gpiob::{PB8, PB9},
        gpiod::{PD0, PD1},
        AF1, AF2, AF4, AF6, AF7,
    };
    use crate::rcc::Rcc;
    use crate::stm32;
    use crate::stm32::FDCAN1;

    // All STM32G4 models with CAN support these pins
    pins! {
        FDCAN1 => (PA12<AF4>, PA11<AF4>),
        FDCAN1 => (PB9<AF7>, PB8<AF6>),
        FDCAN1 => (PD1<AF2>, PD0<AF1>),
    }

    unsafe impl fdcan::Instance for FdCan<FDCAN1> {
        const REGISTERS: *mut stm32::fdcan::RegisterBlock = FDCAN1::ptr() as *mut _;
    }

    unsafe impl message_ram::MsgRamExt for FdCan<FDCAN1> {
        const MSG_RAM: *mut message_ram::RegisterBlock = (0x4000_ac00 as *mut _);
    }

    /// Implements sealed::Sealed and Enable for a CAN peripheral (e.g. CAN1)
    impl crate::can::sealed::Sealed for crate::stm32::FDCAN1 {}
    impl crate::can::Enable for crate::stm32::FDCAN1 {
        #[inline(always)]
        fn enable(rcc: &Rcc) {
            // Enable peripheral
            rcc.rb.apb1enr1.modify(|_, w| w.fdcanen().set_bit());
        }
    }
}

#[cfg(any(feature = "stm32g474"))]
mod fdcan2 {
    use super::FdCan;
    use crate::fdcan;
    use crate::fdcan::message_ram;
    use crate::gpio::{
        gpiob::{PB5, PB6, PB12, PB13},
        AF4, AF6, AF8
    };
    use crate::rcc::Rcc;
    use crate::stm32::{self, FDCAN2};

    pins! {
        FDCAN2 => (PB13<AF4>, PB12<AF6>),
        FDCAN2 => (PB6<AF6>, PB5<AF8>),
    }

    unsafe impl fdcan::Instance for FdCan<FDCAN2> {
        const REGISTERS: *mut stm32::fdcan::RegisterBlock = FDCAN2::ptr() as *mut _;
    }

    unsafe impl message_ram::MsgRamExt for FdCan<FDCAN2> {
        const MSG_RAM: *mut message_ram::RegisterBlock = (0x4000_af54 as *mut _);
    }

    impl crate::can::sealed::Sealed for crate::stm32::FDCAN2 {}
    impl crate::can::Enable for crate::stm32::FDCAN2 {
        #[inline(always)]
        fn enable(rcc: &Rcc) {
            // Enable peripheral
            rcc.rb.apb1enr1.modify(|_, w| w.fdcanen().set_bit());
        }
    }
}

#[cfg(any(feature = "stm32g474"))]
mod fdcan3 {
    use super::FdCan;
    use crate::fdcan;
    use crate::fdcan::message_ram;
    use crate::gpio::{
        gpioa::{PA15, PA8},
        gpiob::{PB3, PB4},
        AF8, AF9,
    };
    use crate::rcc::Rcc;
    use crate::stm32::{self, FDCAN3};

    pins! {
        FDCAN3 => (PA15<AF9>, PA8<AF8>),
        FDCAN3 => (PB4<AF8>, PB3<AF9>),
    }

    unsafe impl fdcan::Instance for FdCan<FDCAN3> {
        const REGISTERS: *mut stm32::fdcan::RegisterBlock = FDCAN3::ptr() as *mut _;
    }

    unsafe impl message_ram::MsgRamExt for FdCan<FDCAN3> {
        const MSG_RAM: *mut message_ram::RegisterBlock = (0x4000_b2a4 as *mut _);
    }
}

/*
//TODO: add other types
//TODO: verify correct pins
#[cfg(any(feature = "stm32g474"))]
mod pb9_pb8_af8 {
    use crate::gpio::{
        gpiob::{PB8, PB9},
        AF8,
    };
    use crate::stm32::FDCAN1;
    pins! { FDCAN1 => (PB9<AF8>, PB8<AF8>), }
}
*/
/*
//TODO: add other types
//TODO: verify correct pins
#[cfg(any(feature = "stm32g474"))]
mod pb9_pb8_af9 {
    use crate::gpio::{
        gpiob::{PB8, PB9},
        AF9,
    };
    use crate::stm32::FDCAN1;
    pins! { FDCAN1 => (PB9<AF9>, PB8<AF9>), }
}

//TODO: add other types
//TODO: verify correct pins
#[cfg(any(feature = "stm32g474"))]
mod pg1_pg0 {
    use crate::gpio::{
        gpiog::{PG0, PG1},
        AF9,
    };
    use crate::stm32::FDCAN1;
    pins! { FDCAN1 => (PG1<AF9>, PG0<AF9>), }
}

//TODO: add other types
//TODO: verify correct pins
#[cfg(any(feature = "stm32g474"))]
mod pg12_pg11 {
    use crate::gpio::{
        gpiog::{PG11, PG12},
        AF9,
    };
    use crate::stm32::CAN2;
    pins! { CAN2 => (PG12<AF9>, PG11<AF9>), }
}

//TODO: add other types
//TODO: verify correct pins
#[cfg(any(feature = "stm32g474"))]
mod ph13_pi9 {
    use crate::gpio::{gpioh::PH13, gpioi::PI9, AF9};
    use crate::stm32::CAN1;
    pins! { CAN1 => (PH13<AF9>, PI9<AF9>), }
}
*/
/// Enable/disable peripheral
pub trait Enable: sealed::Sealed {
    /// Enables this peripheral by setting the associated enable bit in an RCC enable register
    fn enable(rcc: &Rcc);
}

/*
/// Pins and definitions for models with a third CAN peripheral
#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
mod can3 {
    use super::Can;
    use crate::gpio::{
        gpioa::{PA15, PA8},
        gpiob::{PB3, PB4},
        AF11,
    };
    use crate::stm32::CAN3;
    pins! {
        CAN3 => (PA15<AF11>, PA8<AF11>),
        CAN3 => (PB4<AF11>, PB3<AF11>),
    }
    bus! { CAN3 => (27), }

    unsafe impl bxcan::Instance for Can<CAN3> {
        const REGISTERS: *mut bxcan::RegisterBlock = CAN3::ptr() as *mut _;
    }

    unsafe impl bxcan::FilterOwner for Can<CAN3> {
        const NUM_FILTER_BANKS: u8 = 14;
    }
}
*/
/// Interface to the CAN peripheral.
pub struct FdCan<Instance> {
    _peripheral: Instance,
}

impl<Instance> FdCan<Instance>
where
    Instance: Enable,
{
    /// Creates a CAN interface.
    pub fn new<P>(can: Instance, _pins: P, rcc: &Rcc) -> FdCan<Instance>
    where
        P: Pins<Instance = Instance>,
    {
        Instance::enable(rcc);
        FdCan { _peripheral: can }
    }

    pub fn new_unchecked(can: Instance, rcc: &Rcc) -> FdCan<Instance> {
        Instance::enable(rcc);
        FdCan { _peripheral: can }
    }
}
