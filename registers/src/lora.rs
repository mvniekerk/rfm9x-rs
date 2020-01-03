#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FIFO read/write access"]
    pub fifo: FIFO,
    #[doc = "0x01 - Operating mode and LoRa(TM) / FSK selection"]
    pub opmode: OPMODE,
    _reserved2: [u8; 4usize],
    #[doc = "0x06 - MSB of the RF carrier frequency"]
    pub rf_freq_msb: RF_FREQ_MSB,
    #[doc = "0x07 - MID of the RF carrier frequency"]
    pub rf_freq_mid: RF_FREQ_MID,
    #[doc = "0x08 - LSB of the RF carrier frequency"]
    pub rf_freq_lsb: RF_FREQ_LSB,
    #[doc = "0x09 - PA output pin, max output power, output power"]
    pub pa: PA,
    #[doc = "0x0a - Rise/Fall time of ramp up/down in FSK"]
    pub pa_ramp: PA_RAMP,
    _reserved7: [u8; 1usize],
    #[doc = "0x0b - Overload current protection config"]
    pub ocp: OCP,
    #[doc = "0x0c - LNA gain and current adjustment"]
    pub lna: LNA,
    #[doc = "0x0d - SPI interface address pointer in FIFO data buffer"]
    pub fifo_add_ptr: FIFO_ADD_PTR,
    #[doc = "0x0e - Write base addres sin FIFO data buffer for TX modulator"]
    pub fifo_tx_base_addr: FIFO_TX_BASE_ADDR,
    #[doc = "0x0f - Read base address in FIFO data buffer for RX demodulator"]
    pub fifo_rx_base_addr: FIFO_RX_BASE_ADDR,
    #[doc = "0x10 - Start address (in data buffer) of last packet received"]
    pub fifo_rx_current_addr: FIFO_RX_CURRENT_ADDR,
    #[doc = "0x11 - Interrupt flags masks"]
    pub irq_flags_mask: IRQ_FLAGS_MASK,
    #[doc = "0x12 - IRQ flags"]
    pub irq_flags: IRQ_FLAGS,
    #[doc = "0x13 - Number of payload bytes of latest packet received"]
    pub rx_nb_bytes: RX_NB_BYTES,
    #[doc = "0x14 - Number of valid headers received since last transition into Rx mode, MSB (15:8). Header and packet counters are reset in Sleep mode"]
    pub rx_header_count_value_msb: RX_HEADER_COUNT_VALUE_MSB,
    #[doc = "0x15 - Number of valid headers received since last transition into Rx mode, LSB (7:0). Header and packet counters are reset in Sleep mode"]
    pub rx_header_count_value_lsb: RX_HEADER_COUNT_VALUE_LSB,
    #[doc = "0x16 - Number of valid packets received since last transition into Rx mode, MSB (15:8). Header and packet counters are reset in Sleep mode"]
    pub rx_packet_count_value_msb: RX_PACKET_COUNT_VALUE_MSB,
    #[doc = "0x17 - Number of valid packets received since last transition into Rx mode, LSB (7:0). Header and packet counters are reset in Sleep mode"]
    pub rx_packet_count_value_lsb: RX_PACKET_COUNT_VALUE_LSB,
    #[doc = "0x18 - Modem stats"]
    pub modem_stat: MODEM_STAT,
    #[doc = "0x19 - Estimation of SNR on last packet received. In two's compliment format multiplied by 4"]
    pub packet_snr: PACKET_SNR,
    #[doc = "0x1a - RSSI of the latest packet received (dBm)"]
    pub packet_rssi: PACKET_RSSI,
    #[doc = "0x1b - Current RSSI value (dBm)"]
    pub rssi: RSSI,
    #[doc = "0x1c - Current value of frequency hopping channel, CRC info and PLL lock info"]
    pub hop_channel: HOP_CHANNEL,
    #[doc = "0x1d - Modem configuration register 1"]
    pub modem_config_1: MODEM_CONFIG_1,
    #[doc = "0x1e - Modem configuration register 2"]
    pub modem_config_2: MODEM_CONFIG_2,
    #[doc = "0x1f - RX Symbol timeout LSB. RX operation time-out value expressed as number of symbols"]
    pub rx_symb_timeout_lsb: RX_SYMB_TIMEOUT_LSB,
    #[doc = "0x20 - Preamble length MSB = Preamble Length + 4.25 symbols"]
    pub preamble_length_msb: PREAMBLE_LENGTH_MSB,
    #[doc = "0x21 - Preamble length LSB = Preamble Length + 4.25 symbols"]
    pub preamble_length_lsb: PREAMBLE_LENGTH_LSB,
    #[doc = "0x22 - Payload length in bytes. The register needs to be set in implicit header for the expected packet length. A 0 value is not permitted"]
    pub payload_length: PAYLOAD_LENGTH,
    #[doc = "0x23 - Max payload length in bytes. If header payload length exceeds value a header CRC error is generated. Allows filtering of packet with a bad size."]
    pub max_payload_length: MAX_PAYLOAD_LENGTH,
    #[doc = "0x24 - Symbol period between frequency hops. 0 = disabled. 1st hop always happen after 1st header symbol"]
    pub freq_hop_period: FREQ_HOP_PERIOD,
    #[doc = "0x25 - Current value of RX databuffer pointer (address of last byte written by Lora receiver)"]
    pub fifo_rx_byte_addr_ptr: FIFO_RX_BYTE_ADDR_PTR,
    #[doc = "0x26 - Configure LNA gain setter and static/mobile node"]
    pub modem_config_3: MODEM_CONFIG_3,
}
#[doc = "FIFO read/write access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u8, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "FIFO read/write access"]
pub mod fifo;
#[doc = "Operating mode and LoRa(TM) / FSK selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opmode](opmode) module"]
pub type OPMODE = crate::Reg<u8, _OPMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPMODE;
#[doc = "`read()` method returns [opmode::R](opmode::R) reader structure"]
impl crate::Readable for OPMODE {}
#[doc = "`write(|w| ..)` method takes [opmode::W](opmode::W) writer structure"]
impl crate::Writable for OPMODE {}
#[doc = "Operating mode and LoRa(TM) / FSK selection"]
pub mod opmode;
#[doc = "MSB of the RF carrier frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_freq_msb](rf_freq_msb) module"]
pub type RF_FREQ_MSB = crate::Reg<u8, _RF_FREQ_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_FREQ_MSB;
#[doc = "`read()` method returns [rf_freq_msb::R](rf_freq_msb::R) reader structure"]
impl crate::Readable for RF_FREQ_MSB {}
#[doc = "`write(|w| ..)` method takes [rf_freq_msb::W](rf_freq_msb::W) writer structure"]
impl crate::Writable for RF_FREQ_MSB {}
#[doc = "MSB of the RF carrier frequency"]
pub mod rf_freq_msb;
#[doc = "MID of the RF carrier frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_freq_mid](rf_freq_mid) module"]
pub type RF_FREQ_MID = crate::Reg<u8, _RF_FREQ_MID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_FREQ_MID;
#[doc = "`read()` method returns [rf_freq_mid::R](rf_freq_mid::R) reader structure"]
impl crate::Readable for RF_FREQ_MID {}
#[doc = "`write(|w| ..)` method takes [rf_freq_mid::W](rf_freq_mid::W) writer structure"]
impl crate::Writable for RF_FREQ_MID {}
#[doc = "MID of the RF carrier frequency"]
pub mod rf_freq_mid;
#[doc = "LSB of the RF carrier frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_freq_lsb](rf_freq_lsb) module"]
pub type RF_FREQ_LSB = crate::Reg<u8, _RF_FREQ_LSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_FREQ_LSB;
#[doc = "`read()` method returns [rf_freq_lsb::R](rf_freq_lsb::R) reader structure"]
impl crate::Readable for RF_FREQ_LSB {}
#[doc = "`write(|w| ..)` method takes [rf_freq_lsb::W](rf_freq_lsb::W) writer structure"]
impl crate::Writable for RF_FREQ_LSB {}
#[doc = "LSB of the RF carrier frequency"]
pub mod rf_freq_lsb;
#[doc = "PA output pin, max output power, output power\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa](pa) module"]
pub type PA = crate::Reg<u8, _PA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA;
#[doc = "`read()` method returns [pa::R](pa::R) reader structure"]
impl crate::Readable for PA {}
#[doc = "`write(|w| ..)` method takes [pa::W](pa::W) writer structure"]
impl crate::Writable for PA {}
#[doc = "PA output pin, max output power, output power"]
pub mod pa;
#[doc = "Rise/Fall time of ramp up/down in FSK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_ramp](pa_ramp) module"]
pub type PA_RAMP = crate::Reg<u8, _PA_RAMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_RAMP;
#[doc = "`read()` method returns [pa_ramp::R](pa_ramp::R) reader structure"]
impl crate::Readable for PA_RAMP {}
#[doc = "`write(|w| ..)` method takes [pa_ramp::W](pa_ramp::W) writer structure"]
impl crate::Writable for PA_RAMP {}
#[doc = "Rise/Fall time of ramp up/down in FSK"]
pub mod pa_ramp;
#[doc = "Overload current protection config\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocp](ocp) module"]
pub type OCP = crate::Reg<u8, _OCP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCP;
#[doc = "`read()` method returns [ocp::R](ocp::R) reader structure"]
impl crate::Readable for OCP {}
#[doc = "`write(|w| ..)` method takes [ocp::W](ocp::W) writer structure"]
impl crate::Writable for OCP {}
#[doc = "Overload current protection config"]
pub mod ocp;
#[doc = "LNA gain and current adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lna](lna) module"]
pub type LNA = crate::Reg<u8, _LNA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LNA;
#[doc = "`read()` method returns [lna::R](lna::R) reader structure"]
impl crate::Readable for LNA {}
#[doc = "`write(|w| ..)` method takes [lna::W](lna::W) writer structure"]
impl crate::Writable for LNA {}
#[doc = "LNA gain and current adjustment"]
pub mod lna;
#[doc = "SPI interface address pointer in FIFO data buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_add_ptr](fifo_add_ptr) module"]
pub type FIFO_ADD_PTR = crate::Reg<u8, _FIFO_ADD_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_ADD_PTR;
#[doc = "`read()` method returns [fifo_add_ptr::R](fifo_add_ptr::R) reader structure"]
impl crate::Readable for FIFO_ADD_PTR {}
#[doc = "`write(|w| ..)` method takes [fifo_add_ptr::W](fifo_add_ptr::W) writer structure"]
impl crate::Writable for FIFO_ADD_PTR {}
#[doc = "SPI interface address pointer in FIFO data buffer"]
pub mod fifo_add_ptr;
#[doc = "Write base addres sin FIFO data buffer for TX modulator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_tx_base_addr](fifo_tx_base_addr) module"]
pub type FIFO_TX_BASE_ADDR = crate::Reg<u8, _FIFO_TX_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_TX_BASE_ADDR;
#[doc = "`read()` method returns [fifo_tx_base_addr::R](fifo_tx_base_addr::R) reader structure"]
impl crate::Readable for FIFO_TX_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [fifo_tx_base_addr::W](fifo_tx_base_addr::W) writer structure"]
impl crate::Writable for FIFO_TX_BASE_ADDR {}
#[doc = "Write base addres sin FIFO data buffer for TX modulator"]
pub mod fifo_tx_base_addr;
#[doc = "Read base address in FIFO data buffer for RX demodulator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_rx_base_addr](fifo_rx_base_addr) module"]
pub type FIFO_RX_BASE_ADDR = crate::Reg<u8, _FIFO_RX_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_RX_BASE_ADDR;
#[doc = "`read()` method returns [fifo_rx_base_addr::R](fifo_rx_base_addr::R) reader structure"]
impl crate::Readable for FIFO_RX_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [fifo_rx_base_addr::W](fifo_rx_base_addr::W) writer structure"]
impl crate::Writable for FIFO_RX_BASE_ADDR {}
#[doc = "Read base address in FIFO data buffer for RX demodulator"]
pub mod fifo_rx_base_addr;
#[doc = "Start address (in data buffer) of last packet received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_rx_current_addr](fifo_rx_current_addr) module"]
pub type FIFO_RX_CURRENT_ADDR = crate::Reg<u8, _FIFO_RX_CURRENT_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_RX_CURRENT_ADDR;
#[doc = "`read()` method returns [fifo_rx_current_addr::R](fifo_rx_current_addr::R) reader structure"]
impl crate::Readable for FIFO_RX_CURRENT_ADDR {}
#[doc = "`write(|w| ..)` method takes [fifo_rx_current_addr::W](fifo_rx_current_addr::W) writer structure"]
impl crate::Writable for FIFO_RX_CURRENT_ADDR {}
#[doc = "Start address (in data buffer) of last packet received"]
pub mod fifo_rx_current_addr;
#[doc = "Interrupt flags masks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_flags_mask](irq_flags_mask) module"]
pub type IRQ_FLAGS_MASK = crate::Reg<u8, _IRQ_FLAGS_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_FLAGS_MASK;
#[doc = "`read()` method returns [irq_flags_mask::R](irq_flags_mask::R) reader structure"]
impl crate::Readable for IRQ_FLAGS_MASK {}
#[doc = "`write(|w| ..)` method takes [irq_flags_mask::W](irq_flags_mask::W) writer structure"]
impl crate::Writable for IRQ_FLAGS_MASK {}
#[doc = "Interrupt flags masks"]
pub mod irq_flags_mask;
#[doc = "IRQ flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_flags](irq_flags) module"]
pub type IRQ_FLAGS = crate::Reg<u8, _IRQ_FLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_FLAGS;
#[doc = "`read()` method returns [irq_flags::R](irq_flags::R) reader structure"]
impl crate::Readable for IRQ_FLAGS {}
#[doc = "`write(|w| ..)` method takes [irq_flags::W](irq_flags::W) writer structure"]
impl crate::Writable for IRQ_FLAGS {}
#[doc = "IRQ flags"]
pub mod irq_flags;
#[doc = "Number of payload bytes of latest packet received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_nb_bytes](rx_nb_bytes) module"]
pub type RX_NB_BYTES = crate::Reg<u8, _RX_NB_BYTES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_NB_BYTES;
#[doc = "`read()` method returns [rx_nb_bytes::R](rx_nb_bytes::R) reader structure"]
impl crate::Readable for RX_NB_BYTES {}
#[doc = "`write(|w| ..)` method takes [rx_nb_bytes::W](rx_nb_bytes::W) writer structure"]
impl crate::Writable for RX_NB_BYTES {}
#[doc = "Number of payload bytes of latest packet received"]
pub mod rx_nb_bytes;
#[doc = "Number of valid headers received since last transition into Rx mode, MSB (15:8). Header and packet counters are reset in Sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_header_count_value_msb](rx_header_count_value_msb) module"]
pub type RX_HEADER_COUNT_VALUE_MSB = crate::Reg<u8, _RX_HEADER_COUNT_VALUE_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_HEADER_COUNT_VALUE_MSB;
#[doc = "`read()` method returns [rx_header_count_value_msb::R](rx_header_count_value_msb::R) reader structure"]
impl crate::Readable for RX_HEADER_COUNT_VALUE_MSB {}
#[doc = "Number of valid headers received since last transition into Rx mode, MSB (15:8). Header and packet counters are reset in Sleep mode"]
pub mod rx_header_count_value_msb;
#[doc = "Number of valid headers received since last transition into Rx mode, LSB (7:0). Header and packet counters are reset in Sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_header_count_value_lsb](rx_header_count_value_lsb) module"]
pub type RX_HEADER_COUNT_VALUE_LSB = crate::Reg<u8, _RX_HEADER_COUNT_VALUE_LSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_HEADER_COUNT_VALUE_LSB;
#[doc = "`read()` method returns [rx_header_count_value_lsb::R](rx_header_count_value_lsb::R) reader structure"]
impl crate::Readable for RX_HEADER_COUNT_VALUE_LSB {}
#[doc = "Number of valid headers received since last transition into Rx mode, LSB (7:0). Header and packet counters are reset in Sleep mode"]
pub mod rx_header_count_value_lsb;
#[doc = "Number of valid packets received since last transition into Rx mode, MSB (15:8). Header and packet counters are reset in Sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_packet_count_value_msb](rx_packet_count_value_msb) module"]
pub type RX_PACKET_COUNT_VALUE_MSB = crate::Reg<u8, _RX_PACKET_COUNT_VALUE_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_PACKET_COUNT_VALUE_MSB;
#[doc = "`read()` method returns [rx_packet_count_value_msb::R](rx_packet_count_value_msb::R) reader structure"]
impl crate::Readable for RX_PACKET_COUNT_VALUE_MSB {}
#[doc = "Number of valid packets received since last transition into Rx mode, MSB (15:8). Header and packet counters are reset in Sleep mode"]
pub mod rx_packet_count_value_msb;
#[doc = "Number of valid packets received since last transition into Rx mode, LSB (7:0). Header and packet counters are reset in Sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_packet_count_value_lsb](rx_packet_count_value_lsb) module"]
pub type RX_PACKET_COUNT_VALUE_LSB = crate::Reg<u8, _RX_PACKET_COUNT_VALUE_LSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_PACKET_COUNT_VALUE_LSB;
#[doc = "`read()` method returns [rx_packet_count_value_lsb::R](rx_packet_count_value_lsb::R) reader structure"]
impl crate::Readable for RX_PACKET_COUNT_VALUE_LSB {}
#[doc = "Number of valid packets received since last transition into Rx mode, LSB (7:0). Header and packet counters are reset in Sleep mode"]
pub mod rx_packet_count_value_lsb;
#[doc = "Modem stats\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modem_stat](modem_stat) module"]
pub type MODEM_STAT = crate::Reg<u8, _MODEM_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODEM_STAT;
#[doc = "`read()` method returns [modem_stat::R](modem_stat::R) reader structure"]
impl crate::Readable for MODEM_STAT {}
#[doc = "Modem stats"]
pub mod modem_stat;
#[doc = "Estimation of SNR on last packet received. In two's compliment format multiplied by 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet_snr](packet_snr) module"]
pub type PACKET_SNR = crate::Reg<u8, _PACKET_SNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKET_SNR;
#[doc = "`read()` method returns [packet_snr::R](packet_snr::R) reader structure"]
impl crate::Readable for PACKET_SNR {}
#[doc = "Estimation of SNR on last packet received. In two's compliment format multiplied by 4"]
pub mod packet_snr;
#[doc = "RSSI of the latest packet received (dBm)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet_rssi](packet_rssi) module"]
pub type PACKET_RSSI = crate::Reg<u8, _PACKET_RSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKET_RSSI;
#[doc = "`read()` method returns [packet_rssi::R](packet_rssi::R) reader structure"]
impl crate::Readable for PACKET_RSSI {}
#[doc = "RSSI of the latest packet received (dBm)"]
pub mod packet_rssi;
#[doc = "Current RSSI value (dBm)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rssi](rssi) module"]
pub type RSSI = crate::Reg<u8, _RSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSSI;
#[doc = "`read()` method returns [rssi::R](rssi::R) reader structure"]
impl crate::Readable for RSSI {}
#[doc = "Current RSSI value (dBm)"]
pub mod rssi;
#[doc = "Current value of frequency hopping channel, CRC info and PLL lock info\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hop_channel](hop_channel) module"]
pub type HOP_CHANNEL = crate::Reg<u8, _HOP_CHANNEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOP_CHANNEL;
#[doc = "`read()` method returns [hop_channel::R](hop_channel::R) reader structure"]
impl crate::Readable for HOP_CHANNEL {}
#[doc = "Current value of frequency hopping channel, CRC info and PLL lock info"]
pub mod hop_channel;
#[doc = "Modem configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modem_config_1](modem_config_1) module"]
pub type MODEM_CONFIG_1 = crate::Reg<u8, _MODEM_CONFIG_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODEM_CONFIG_1;
#[doc = "`read()` method returns [modem_config_1::R](modem_config_1::R) reader structure"]
impl crate::Readable for MODEM_CONFIG_1 {}
#[doc = "`write(|w| ..)` method takes [modem_config_1::W](modem_config_1::W) writer structure"]
impl crate::Writable for MODEM_CONFIG_1 {}
#[doc = "Modem configuration register 1"]
pub mod modem_config_1;
#[doc = "Modem configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modem_config_2](modem_config_2) module"]
pub type MODEM_CONFIG_2 = crate::Reg<u8, _MODEM_CONFIG_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODEM_CONFIG_2;
#[doc = "`read()` method returns [modem_config_2::R](modem_config_2::R) reader structure"]
impl crate::Readable for MODEM_CONFIG_2 {}
#[doc = "`write(|w| ..)` method takes [modem_config_2::W](modem_config_2::W) writer structure"]
impl crate::Writable for MODEM_CONFIG_2 {}
#[doc = "Modem configuration register 2"]
pub mod modem_config_2;
#[doc = "RX Symbol timeout LSB. RX operation time-out value expressed as number of symbols\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_symb_timeout_lsb](rx_symb_timeout_lsb) module"]
pub type RX_SYMB_TIMEOUT_LSB = crate::Reg<u8, _RX_SYMB_TIMEOUT_LSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_SYMB_TIMEOUT_LSB;
#[doc = "`read()` method returns [rx_symb_timeout_lsb::R](rx_symb_timeout_lsb::R) reader structure"]
impl crate::Readable for RX_SYMB_TIMEOUT_LSB {}
#[doc = "`write(|w| ..)` method takes [rx_symb_timeout_lsb::W](rx_symb_timeout_lsb::W) writer structure"]
impl crate::Writable for RX_SYMB_TIMEOUT_LSB {}
#[doc = "RX Symbol timeout LSB. RX operation time-out value expressed as number of symbols"]
pub mod rx_symb_timeout_lsb;
#[doc = "Preamble length MSB = Preamble Length + 4.25 symbols\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preamble_length_msb](preamble_length_msb) module"]
pub type PREAMBLE_LENGTH_MSB = crate::Reg<u8, _PREAMBLE_LENGTH_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PREAMBLE_LENGTH_MSB;
#[doc = "`read()` method returns [preamble_length_msb::R](preamble_length_msb::R) reader structure"]
impl crate::Readable for PREAMBLE_LENGTH_MSB {}
#[doc = "`write(|w| ..)` method takes [preamble_length_msb::W](preamble_length_msb::W) writer structure"]
impl crate::Writable for PREAMBLE_LENGTH_MSB {}
#[doc = "Preamble length MSB = Preamble Length + 4.25 symbols"]
pub mod preamble_length_msb;
#[doc = "Preamble length LSB = Preamble Length + 4.25 symbols\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preamble_length_lsb](preamble_length_lsb) module"]
pub type PREAMBLE_LENGTH_LSB = crate::Reg<u8, _PREAMBLE_LENGTH_LSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PREAMBLE_LENGTH_LSB;
#[doc = "`read()` method returns [preamble_length_lsb::R](preamble_length_lsb::R) reader structure"]
impl crate::Readable for PREAMBLE_LENGTH_LSB {}
#[doc = "`write(|w| ..)` method takes [preamble_length_lsb::W](preamble_length_lsb::W) writer structure"]
impl crate::Writable for PREAMBLE_LENGTH_LSB {}
#[doc = "Preamble length LSB = Preamble Length + 4.25 symbols"]
pub mod preamble_length_lsb;
#[doc = "Payload length in bytes. The register needs to be set in implicit header for the expected packet length. A 0 value is not permitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [payload_length](payload_length) module"]
pub type PAYLOAD_LENGTH = crate::Reg<u8, _PAYLOAD_LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAYLOAD_LENGTH;
#[doc = "`read()` method returns [payload_length::R](payload_length::R) reader structure"]
impl crate::Readable for PAYLOAD_LENGTH {}
#[doc = "`write(|w| ..)` method takes [payload_length::W](payload_length::W) writer structure"]
impl crate::Writable for PAYLOAD_LENGTH {}
#[doc = "Payload length in bytes. The register needs to be set in implicit header for the expected packet length. A 0 value is not permitted"]
pub mod payload_length;
#[doc = "Max payload length in bytes. If header payload length exceeds value a header CRC error is generated. Allows filtering of packet with a bad size.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [max_payload_length](max_payload_length) module"]
pub type MAX_PAYLOAD_LENGTH = crate::Reg<u8, _MAX_PAYLOAD_LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAX_PAYLOAD_LENGTH;
#[doc = "`read()` method returns [max_payload_length::R](max_payload_length::R) reader structure"]
impl crate::Readable for MAX_PAYLOAD_LENGTH {}
#[doc = "`write(|w| ..)` method takes [max_payload_length::W](max_payload_length::W) writer structure"]
impl crate::Writable for MAX_PAYLOAD_LENGTH {}
#[doc = "Max payload length in bytes. If header payload length exceeds value a header CRC error is generated. Allows filtering of packet with a bad size."]
pub mod max_payload_length;
#[doc = "Symbol period between frequency hops. 0 = disabled. 1st hop always happen after 1st header symbol\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freq_hop_period](freq_hop_period) module"]
pub type FREQ_HOP_PERIOD = crate::Reg<u8, _FREQ_HOP_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQ_HOP_PERIOD;
#[doc = "`read()` method returns [freq_hop_period::R](freq_hop_period::R) reader structure"]
impl crate::Readable for FREQ_HOP_PERIOD {}
#[doc = "`write(|w| ..)` method takes [freq_hop_period::W](freq_hop_period::W) writer structure"]
impl crate::Writable for FREQ_HOP_PERIOD {}
#[doc = "Symbol period between frequency hops. 0 = disabled. 1st hop always happen after 1st header symbol"]
pub mod freq_hop_period;
#[doc = "Current value of RX databuffer pointer (address of last byte written by Lora receiver)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_rx_byte_addr_ptr](fifo_rx_byte_addr_ptr) module"]
pub type FIFO_RX_BYTE_ADDR_PTR = crate::Reg<u8, _FIFO_RX_BYTE_ADDR_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_RX_BYTE_ADDR_PTR;
#[doc = "`read()` method returns [fifo_rx_byte_addr_ptr::R](fifo_rx_byte_addr_ptr::R) reader structure"]
impl crate::Readable for FIFO_RX_BYTE_ADDR_PTR {}
#[doc = "Current value of RX databuffer pointer (address of last byte written by Lora receiver)"]
pub mod fifo_rx_byte_addr_ptr;
#[doc = "Configure LNA gain setter and static/mobile node\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modem_config_3](modem_config_3) module"]
pub type MODEM_CONFIG_3 = crate::Reg<u8, _MODEM_CONFIG_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODEM_CONFIG_3;
#[doc = "`read()` method returns [modem_config_3::R](modem_config_3::R) reader structure"]
impl crate::Readable for MODEM_CONFIG_3 {}
#[doc = "`write(|w| ..)` method takes [modem_config_3::W](modem_config_3::W) writer structure"]
impl crate::Writable for MODEM_CONFIG_3 {}
#[doc = "Configure LNA gain setter and static/mobile node"]
pub mod modem_config_3;
