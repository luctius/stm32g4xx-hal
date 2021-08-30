#![no_main]
#![no_std]

use crate::hal::{
    fdcan::{
        config::NominalBitTiming,
        filter::{StandardFilter, StandardFilterSlot},
        frame::{FrameFormat, TxFrameHeader},
        id::StandardId,
        FdCan,
    },
    gpio::GpioExt,
    nb::block,
    rcc::{Config, RccExt},
    stm32::Peripherals,
};
use stm32g4xx_hal as hal;

use cortex_m_rt::entry;

use log::info;

#[macro_use]
mod utils;

#[entry]
fn main() -> ! {
    utils::logger::init();

    info!("start");

    // APB1 (PCLK1): 8MHz, Bit rate: 125kBit/s, Sample Point 87.5%
    // Value was calculated with http://www.bittiming.can-wiki.info/
    // TODO: use the can_bit_timings crate
    let btr = NominalBitTiming {
        prescaler: 4,
        seg1: 13,
        seg2: 2,
        ..Default::default()
    };

    let dp = Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");
    let rcc = dp.RCC.constrain();
    let mut rcc = rcc.freeze(Config::hsi());

    let mut can1 = {
        let gpioa = dp.GPIOA.split(&mut rcc);
        let rx = gpioa.pa11.into_alternate();
        let tx = gpioa.pa12.into_alternate();

        let can = crate::hal::can::FdCan::new(dp.FDCAN1, (tx, rx), &rcc);
        let mut can = FdCan::new(can).into_config_mode();

        can.set_nominal_bit_timing(btr);
        can.set_standard_filter(
            StandardFilterSlot::_0,
            StandardFilter::accept_all_into_fifo0(),
        );
        can.into_normal()
    };

    let mut can2 = {
        let gpiob = dp.GPIOB.split(&mut rcc);
        let rx = gpiob.pb12.into_alternate();
        let tx = gpiob.pb13.into_alternate();

        let can = crate::hal::can::FdCan::new(dp.FDCAN2, (tx, rx), &rcc);
        let mut can = FdCan::new(can).into_config_mode();

        can.set_nominal_bit_timing(btr);
        can.set_standard_filter(
            StandardFilterSlot::_0,
            StandardFilter::accept_all_into_fifo0(),
        );

        can.into_normal()
    };

    let mut buffer = [0_u32; 2];
    let header = TxFrameHeader {
        len: buffer.len() as u8 * 4,
        id: StandardId::new(0x1).unwrap().into(),
        frame_format: FrameFormat::Standard,
        bit_rate_switching: false,
        marker: None,
    };
    block!(can1.transmit(header, &mut |b| b.clone_from_slice(&buffer))).unwrap();

    loop {
        if let Ok(rxheader) = block!(can2.receive0(&mut |h, b| {
            buffer.clone_from_slice(b);
            h
        })) {
            block!(
                can2.transmit(rxheader.unwrap().to_tx_header(None), &mut |b| b
                    .clone_from_slice(&buffer))
            )
            .unwrap();
        }
        if let Ok(rxheader) = block!(can1.receive0(&mut |h, b| {
            buffer.clone_from_slice(b);
            h
        })) {
            block!(
                can1.transmit(rxheader.unwrap().to_tx_header(None), &mut |b| b
                    .clone_from_slice(&buffer))
            )
            .unwrap();
        }
    }
}
