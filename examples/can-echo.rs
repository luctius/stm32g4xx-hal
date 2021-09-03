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

    info!("Start");

    // APB1 (PCLK1): 16MHz, Bit rate: 250kBit/s, Sample Point 87.5%
    // Value was calculated with http://www.bittiming.can-wiki.info/
    // TODO: use the can_bit_timings crate
    let btr = NominalBitTiming {
        prescaler: 8,
        seg1: 13,
        seg2: 2,
        sync_jump_width: 1,
    };

    info!("Init Clocks");

    let dp = Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");
    let rcc = dp.RCC.constrain();
    let mut rcc = rcc.freeze(Config::hsi());

    info!("Split GPIO");

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    let mut can1 = {
        info!("Init CAN 1");
        let rx = gpiob.pb8.into_alternate();
        let tx = gpiob.pb9.into_alternate();

        info!("-- Create CAN 1 instance");
        let can = crate::hal::can::FdCan::new(dp.FDCAN1, (tx, rx), &rcc);

        info!("-- Set CAN 1 in Config Mode");
        let mut can = FdCan::new(can).into_config_mode();

        info!("-- Configure nominal timing");
        can.set_nominal_bit_timing(btr);

        info!("-- Configure Filters");
        can.set_standard_filter(
            StandardFilterSlot::_0,
            StandardFilter::accept_all_into_fifo0(),
        );

        info!("-- Set CAN1 in to normal mode");
        can.into_external_loopback()
    };

    info!("Create Message Data");
    let mut buffer = [0xAABBCCDD, 0xEEFF0011];
    info!("Create Message Header");
    let header = TxFrameHeader {
        len: buffer.len() as u8 * 4,
        id: StandardId::new(0x1).unwrap().into(),
        frame_format: FrameFormat::Standard,
        bit_rate_switching: false,
        marker: None,
    };
    info!("Initial Header: {:?}", &header);

    info!("Transmit initial message");
    block!(can1.transmit(header, &mut |b| b.clone_from_slice(&buffer))).unwrap();

    loop {
        if let Ok(rxheader) = block!(can1.receive0(&mut |h, b| {
            info!("CAN 1 Receive");
            info!("Received Header: {:?}", &h);
            buffer.clone_from_slice(b);
            h
        })) {
            block!(
                can1.transmit(rxheader.unwrap().to_tx_header(None), &mut |b| {
                    info!("CAN 1 Transmit");
                    b.clone_from_slice(&buffer)
                })
            )
            .unwrap();
        }
    }
}
