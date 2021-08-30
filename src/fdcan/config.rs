pub use super::interrupt::{Interrupt, InterruptLine, Interrupts};

/// Configures the bit timings.
///
/// You can use <http://www.bittiming.can-wiki.info/> to calculate the `btr` parameter. Enter
/// parameters as follows:
///
/// - *Clock Rate*: The input clock speed to the CAN peripheral (*not* the CPU clock speed).
///   This is the clock rate of the peripheral bus the CAN peripheral is attached to (eg. APB1).
/// - *Sample Point*: Should normally be left at the default value of 87.5%.
/// - *SJW*: Should normally be left at the default value of 1.
///
/// Then copy the `CAN_BUS_TIME` register value from the table and pass it as the `btr`
/// parameter to this method.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "unstable-defmt", derive(defmt::Format))]
pub struct NominalBitTiming {
    /// Value by which the oscillator frequency is divided for generating the bit time quanta. The bit
    /// time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual
    /// interpretation by the hardware of this value is such that one more than the value programmed
    /// here is used.
    pub prescaler: u16,
    /// Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that
    /// one more than the programmed value is used.
    pub seg1: u8,
    /// Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that
    /// one more than the programmed value is used.
    pub seg2: u8,
    /// Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that
    /// the used value is the one programmed incremented by one.
    pub sync_jump_width: u8,
}
impl NominalBitTiming {
    #[inline]
    pub(crate) fn nbrp(&self) -> u16 {
        self.prescaler & 0x1FF
    }
    #[inline]
    pub(crate) fn ntseg1(&self) -> u8 {
        self.seg1 & 0xFF
    }
    #[inline]
    pub(crate) fn ntseg2(&self) -> u8 {
        self.seg2 & 0x7F
    }
    #[inline]
    pub(crate) fn nsjw(&self) -> u8 {
        self.sync_jump_width & 0x7F
    }
}

impl Default for NominalBitTiming {
    #[inline]
    fn default() -> Self {
        Self {
            prescaler: 0,
            seg1: 0xA,
            seg2: 0x3,
            sync_jump_width: 0x3,
        }
    }
}

/// Configures the data bit timings for the FdCan Variable Bitrates.
/// This is not used when frame_transmit is set to anything other than AllowFdCanAndBRS.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "unstable-defmt", derive(defmt::Format))]
pub struct DataBitTiming {
    /// Tranceiver Delay Compensation
    pub transceiver_delay_compensation: bool,
    ///  The value by which the oscillator frequency is divided to generate the bit time quanta. The bit
    ///  time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0
    ///  to 31. The hardware interpreters this value as the value programmed plus 1.
    pub prescaler: u8,
    /// Valid values are 0 to 31. The value used by the hardware is the one programmed,
    /// incremented by 1, i.e. tBS1 = (DTSEG1 + 1) x tq.
    pub seg1: u8,
    /// Valid values are 0 to 15. The value used by the hardware is the one programmed,
    /// incremented by 1, i.e. tBS2 = (DTSEG2 + 1) x tq.
    pub seg2: u8,
    /// Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the
    /// hardware is the one programmed, incremented by 1: tSJW = (DSJW + 1) x tq.
    pub sync_jump_width: u8,
}
impl DataBitTiming {
    // #[inline]
    // fn tdc(&self) -> u8 {
    //     let tsd = self.transceiver_delay_compensation as u8;
    //     //TODO: stm32g4 does not export the TDC field
    //     todo!()
    // }
    #[inline]
    pub(crate) fn dbrp(&self) -> u8 {
        self.prescaler & 0x1F
    }
    #[inline]
    pub(crate) fn dtseg1(&self) -> u8 {
        self.seg1 & 0x1F
    }
    #[inline]
    pub(crate) fn dtseg2(&self) -> u8 {
        self.seg2 & 0x0F
    }
    #[inline]
    pub(crate) fn dsjw(&self) -> u8 {
        self.sync_jump_width & 0x0F
    }
}

impl Default for DataBitTiming {
    #[inline]
    fn default() -> Self {
        Self {
            transceiver_delay_compensation: false,
            prescaler: 0,
            seg1: 0xA,
            seg2: 0x3,
            sync_jump_width: 0x3,
        }
    }
}

/// Configures which modes to use
/// Individual headers can contain a desire to be send via FdCan
/// or use Bit rate switching. But if this general setting does not allow
/// that, only classic CAN is used instead.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "unstable-defmt", derive(defmt::Format))]
pub enum FrameTransmissionConfig {
    /// Only allow Classic CAN message Frames
    ClassicCanOnly,
    /// Allow (non-brs) FdCAN Message Frames
    AllowFdCan,
    /// Allow FdCAN Message Frames and allow Bit Rate Switching
    AllowFdCanAndBRS,
}

///
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "unstable-defmt", derive(defmt::Format))]
pub enum ClockDivider {
    /// Divide by 1
    _1 = 0b0000,
    /// Divide by 2
    _2 = 0b0001,
    /// Divide by 4
    _4 = 0b0010,
    /// Divide by 6
    _6 = 0b0011,
    /// Divide by 8
    _8 = 0b0100,
    /// Divide by 10
    _10 = 0b0101,
    /// Divide by 12
    _12 = 0b0110,
    /// Divide by 14
    _14 = 0b0111,
    /// Divide by 16
    _16 = 0b1000,
    /// Divide by 18
    _18 = 0b1001,
    /// Divide by 20
    _20 = 0b1010,
    /// Divide by 22
    _22 = 0b1011,
    /// Divide by 24
    _24 = 0b1100,
    /// Divide by 26
    _26 = 0b1101,
    /// Divide by 28
    _28 = 0b1110,
    /// Divide by 30
    _30 = 0b1111,
}

/// Prescaler of the Timestamp counter
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "unstable-defmt", derive(defmt::Format))]
pub enum TimestampPrescaler {
    /// 1
    _1 = 1,
    /// 2
    _2 = 2,
    /// 3
    _3 = 3,
    /// 4
    _4 = 4,
    /// 5
    _5 = 5,
    /// 6
    _6 = 6,
    /// 7
    _7 = 7,
    /// 8
    _8 = 8,
    /// 9
    _9 = 9,
    /// 10
    _10 = 10,
    /// 11
    _11 = 11,
    /// 12
    _12 = 12,
    /// 13
    _13 = 13,
    /// 14
    _14 = 14,
    /// 15
    _15 = 15,
    /// 16
    _16 = 16,
}

/// Selects the source of the Timestamp counter
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "unstable-defmt", derive(defmt::Format))]
pub enum TimestampSource {
    /// The Timestamp counter is disabled
    None,
    /// Using the FdCan input clock as the Timstamp counter's source,
    /// and using a specific prescaler
    Prescaler(TimestampPrescaler),
    /// Using TIM3 as a source
    FromTIM3,
}

/// FdCan Config Struct
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "unstable-defmt", derive(defmt::Format))]
pub struct FdCanConfig {
    /// Nominal Bit Timings
    pub nbtr: NominalBitTiming,
    /// (Variable) Data Bit Timings
    pub dbtr: DataBitTiming,
    /// Enables or disables automatic retransmission of messages
    ///
    /// If this is enabled, the CAN peripheral will automatically try to retransmit each frame
    /// util it can be sent. Otherwise, it will try only once to send each frame.
    ///
    /// Automatic retransmission is enabled by default.
    pub automatic_retransmit: bool,
    /// Enabled or disables the pausing between transmissions
    ///
    /// This feature looses up burst transmissions coming from a single node and it protects against
    /// "babbling idiot" scenarios where the application program erroneously requests too many
    /// transmissions.
    pub transmit_pause: bool,
    /// Enabled or disables the pausing between transmissions
    ///
    /// This feature looses up burst transmissions coming from a single node and it protects against
    /// "babbling idiot" scenarios where the application program erroneously requests too many
    /// transmissions.
    pub frame_transmit: FrameTransmissionConfig,
    /// Non Isoe Mode
    /// If this is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN
    /// FD Specification V1.0.
    pub non_iso_mode: bool,
    /// Edge Filtering: Two consecutive dominant tq required to detect an edge for hard synchronization
    pub edge_filtering: bool,
    /// Enables protocol exception handling
    pub protocol_exception_handling: bool,
    /// Sets the general clock divider for this FdCAN instance
    pub clock_divider: ClockDivider,
    /// This sets the interrupts for each interrupt line of the FdCan (FDCAN_INT0/1)
    /// Each interrupt set to 0 is set to line_0, each set to 1 is set to line_1.
    /// NOTE: This does not enable or disable the interrupt, but merely configure
    /// them to which interrupt the WOULD trigger if they are enabled.
    pub interrupt_line_config: Interrupts,
    /// Sets the timestamp source
    pub timestamp_source: TimestampSource,
}

impl FdCanConfig {
    /// Configures the bit timings.
    #[inline]
    pub fn set_nominal_bit_timing(mut self, btr: NominalBitTiming) -> Self {
        self.nbtr = btr;
        self
    }

    /// Configures the bit timings.
    #[inline]
    pub fn set_data_bit_timing(mut self, btr: DataBitTiming) -> Self {
        self.dbtr = btr;
        self
    }

    /// Enables or disables automatic retransmission of messages
    ///
    /// If this is enabled, the CAN peripheral will automatically try to retransmit each frame
    /// util it can be sent. Otherwise, it will try only once to send each frame.
    ///
    /// Automatic retransmission is enabled by default.
    #[inline]
    pub fn set_automatic_retransmit(mut self, enabled: bool) -> Self {
        self.automatic_retransmit = enabled;
        self
    }

    /// Enabled or disables the pausing between transmissions
    ///
    /// This feature looses up burst transmissions coming from a single node and it protects against
    /// "babbling idiot" scenarios where the application program erroneously requests too many
    /// transmissions.
    #[inline]
    pub fn set_transmit_pause(mut self, enabled: bool) -> Self {
        self.transmit_pause = enabled;
        self
    }

    /// If this is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN
    /// FD Specification V1.0.
    #[inline]
    pub fn set_non_iso_mode(mut self, enabled: bool) -> Self {
        self.non_iso_mode = enabled;
        self
    }

    /// Two consecutive dominant tq required to detect an edge for hard synchronization
    #[inline]
    pub fn set_edge_filtering(mut self, enabled: bool) -> Self {
        self.edge_filtering = enabled;
        self
    }

    /// Sets the allowed transmission types for messages.
    #[inline]
    pub fn set_frame_transmit(mut self, fts: FrameTransmissionConfig) -> Self {
        self.frame_transmit = fts;
        self
    }

    /// Enables protocol exception handling
    #[inline]
    pub fn set_protocol_exception_handling(mut self, peh: bool) -> Self {
        self.protocol_exception_handling = peh;
        self
    }

    /// Configures which interrupt go to which interrupt lines
    #[inline]
    pub fn set_interrupt_line_config(mut self, l0int: Interrupts) -> Self {
        self.interrupt_line_config = l0int;
        self
    }

    /// Sets the general clock divider for this FdCAN instance
    #[inline]
    pub fn set_clock_divider(mut self, div: ClockDivider) -> Self {
        self.clock_divider = div;
        self
    }

    /// Sets the timestamp source
    #[inline]
    pub fn set_timestamp_source(mut self, tss: TimestampSource) -> Self {
        self.timestamp_source = tss;
        self
    }
}

impl Default for FdCanConfig {
    #[inline]
    fn default() -> Self {
        Self {
            nbtr: NominalBitTiming::default(),
            dbtr: DataBitTiming::default(),
            automatic_retransmit: false,
            transmit_pause: false,
            frame_transmit: FrameTransmissionConfig::ClassicCanOnly,
            non_iso_mode: false,
            edge_filtering: false,
            interrupt_line_config: Interrupts::none(),
            protocol_exception_handling: true,
            clock_divider: ClockDivider::_1,
            timestamp_source: TimestampSource::None,
        }
    }
}