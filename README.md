## HAL Modules (48 modules covering all 73 PAC peripheral types)

All drivers use a consistent pointer-based register access pattern: `regs: &'static pac::module::RegisterBlock` initialized in `new()`.

| Module | Instance Selection | Capabilities Provided |
|--------|--------------------|-----------------------|
| **adc** | `Adc::new()` | Command-buffer: `configure()`/`set_channel()`/`trigger_conversion()`/`read_result()`. |
| **axbs** | `Axbs::new()` | AHB crossbar switch: slave arbitration (fixed/round-robin), master priority, per-master arbitration. |
| **cau3** | `Cau3::new()` | CryptoCore + PKHA: 32 GPRs, semaphore lock, task command interface; `pkha_mod_exp()`/`pkha_mod_mul()`/`pkha_ecc_point_mul()`/`pkha_is_prime()`. |
| **crc** | `Crc::new()` | 16/32-bit CRC with configurable polynomial, seed, transpose; `crc16_ccitt()`/`crc32()`. |
| **dma** | `Dma::new()` | 16-channel eDMA: `memcpy()`, `DmaTcd`, `DmaHandle` with scatter-gather, channel link, minor offset, bandwidth/modulo control. |
| **dmamux** | `Dmamux::new()` | 16-channel DMA mux: `enable_channel()`/`disable_channel()`/`set_source()`. |
| **dual_core** | `DualCore::new()` | Core ID via MSCM; synchronized IPC with SEMA42 gate locking; hold/release other core, boot mode, flag passing. |
| **emvsim** | `Emvsim::new()` | Smart card clock/VCC/card reset/presence, blocking byte I/O. |
| **ewm** | `Ewm::new()` | External watchdog: windowed CMPL/CMPH, refresh `0xB4 → 0x4B`. |
| **fb** | `Fb::new()` | FlexBus external bus: 6 chip-select regions (base/mask/control), port multiplexing. |
| **flexio** | `Flexio::new()` | FlexIO raw: 8 shifters + 8 timers, pin/shifter/timer configuration. |
| **flexio_i2c** | `FlexioI2cMaster::new()` | FlexIO-emulated I2C master with `I2cMasterConfig` (baud rate, doze/debug/fast-access). |
| **flexio_spi** | `FlexioSpiMaster::new()` | FlexIO-emulated SPI master with `SpiMasterConfig` (baud rate, clock phase, 8/16-bit data mode). |
| **flexio_uart** | `FlexioUart::new()` | FlexIO-emulated UART with `UartConfig` (baud rate, 7/8/9-bit word count). |
| **ftfe** | `Ftfe::new()` | Flash `erase_sector()`/`program_phrase()`/`program_check()`; CMD_* constants, MGSTAT0 error detection. |
| **gpio** | `Pin<PORT, PIN, MODE>` | Type-state Input/Output/Alternate (ports A–E). Traits: `OutputPin`, `InputPin`, `StatefulOutputPin`, `ErrorType`. |
| **i2s** | `I2s::new()` | I2S/SAI audio: master/slave, configurable frame/word width. |
| **lpcmp** | `Lpcmp::new_0()` / `new_1()` | Low-power comparator: `select_inputs()`/`set_hysteresis()`/`set_filter()`/`configure_dac()`. |
| **lpdac** | `Lpdac::new()` | 12-bit DAC with buffer/FIFO/return-to-zero, `write_and_trigger()`. |
| **lpi2c** | `Lpi2c::new_lpi2c0–3(pcc, scg)` | `I2c<u8>` (START/STOP control, NACK detection); `I2cConfig` (frequency, glitch filter, high-drive, ignore-NACK). |
| **lpit** | `Lpit::new_lpit0(pcc0)` / `new_lpit1(pcc1)` | `DelayNs` trait (channel 0, one-shot timer). |
| **lpspi** | `Lpspi::new_lpspi0–3(pcc, scg)` | `SpiBus<u8>` (auto baud, AUTOPCS, CPOL/CPHA); `SpiMode` master/slave, configurable `word_size`. |
| **lptmr** | `Lptmr::new_0()` / `new_1()` / `new_2()` | 16-bit low-power timer: pulse count, time compare, periodic interrupt. |
| **lpuart** | `Lpuart::new_lpuart0–3(pcc, scg)` | Blocking putc/getc + `core::fmt::Write` + `serial::Read`/`Write` (embedded-hal 1.0); `UartConfig` (word length, parity, stop bits, TX/RX interrupt enable). |
| **llwu** | `Llwu0::new()` / `Llwu1::new()` | 32 wakeup pins, 7 module sources, 2 digital filters. |
| **mcm** | `Mcm::new()` | Core platform config: PLASC/PLAMC, CPCR/L1 cache control, ISCR interrupt status, CPO compute op control. |
| **mscm** | `Mscm::new()` | Processor type/number/master ID, on-chip memory descriptors (OCMDR0–3). |
| **mua** | `Mua::new()` | Inter-core communication: 4 channels `send()`/`receive()` (blocking/non-blocking), flags, interrupts, reset/hold. |
| **pcc** | PCC0 + PCC1 | `enable_*_clock()` clock gating for M4F (PCC0) and M0+ (PCC1). |
| **port** | `set_mux(PORT, PIN, mux)` / `set_pull(PORT, PIN, pull)` | Pin function and pull-up/pull-down (all 5 ports via PAC base addresses). |
| **rsim** | `Rsim::new()` | BLE/XCVR/ZIG/GEN clock gating, RF oscillator control, radio version ID. |
| **rtc** | `Rtc::new()` | Real-time clock: time-of-day (`RtcTime`), alarm, interrupt enable (`RtcInterrupt`), clock source selection. |
| **scg** | `Scg::new()` | `configure()` switches clock sources (FIRC/SIRC/SOSC/LPFLL); `clock_hz()`/`slow_hz()` for peripheral baud rate calculation. |
| **sema42** | `Sema42::<0>::new()` (M4F), `Sema42::<1>::new()` (M0+) | 16-gate hardware semaphore with `SemaStatus` (LockedBy/Free) and `Processor` types; `try_lock()`, `unlock()`, `status()`, `reset_gate()`. |
| **sim** | `Sim::new()` | `family_id()`/`subfamily_id()`/`unique_id()`/`mac_address()`. |
| **smc** | `Smc::new_0()` / `new_1()` | `PowerMode`, `configure(PMPROT)`, `set_mode(RUNM)`, `reset_cause(SRS)`. |
| **spm** | `Spm::new()` | `core_power_mode()`/`regulator_sel()`, LDO status, LVD/HVD. |
| **tpm** | `Tpm::new(instance)` | Edge/center-aligned PWM, input capture, output compare; prescaler 1–128, `TpmConfig` (trigger, doze/debug/pause); `TpmPwmPin<CH>` with `SetDutyCycle`. |
| **trgmux** | `Trgmux::new()` | Trigger routing: `set_sel0()` with locked variant, 64 trigger sources. |
| **trng** | `Trng::new()` | Entropy generator: `read_u32()`/`read_bytes()`/`read_words()`, error recovery, Von Neumann mode. |
| **tstmr** | `Tstmr::new()` | 56-bit free-running counter: `read() -> u64`. |
| **usb** | `Usb::new()` | USB device BDT: `configure_endpoint()`/`prepare_rx()`/`prepare_tx()`. |
| **usbvreg** | `Usbvreg::new()` | USB regulator `enable()`/`disable()`, standby modes, unlock. |
| **usdhc** | `Usdhc::new()` | SD card CMD/RSP/DMA, 4/8-bit bus; `UsdhcConfig` (data timeout, endian mode, watermarks); `DmaMode`/`BootConfig`/`Adma2Descriptor`. |
| **vref** | `Vref::new()` | Voltage reference: `enable(mode)`/`set_trim()`/`set_chop()`, 2.1 V output. |
| **wdog** | `Wdog::new_0()` / `new_1()` | `unlock()`/`refresh()`/`disable()`/`configure()`. |
| **xrdc** | `Xrdc::new()` | Resource domain control: global enable, HW config, master domain assignment, fault/error handling. |

## Driver Architecture

All HAL drivers follow the same pattern — an owning `regs` field initialized once from the PAC pointer:

```rust
pub struct Tpm {
    regs: &'static pac::tpm0::RegisterBlock,
}

impl Tpm {
    pub fn new(ch: TpmInstance) -> Self {
        let regs = match ch {
            TpmInstance::Tpm0 => unsafe { &*(pac::Tpm0::ptr() as *const pac::tpm0::RegisterBlock) },
            // ...
        };
        Self { regs }
    }
}
```

## Build Commands

```powershell
cargo build --workspace                           # Host libraries only
cargo build -p rv32m1-riscv-board --features rt   # With interrupt vector table
```

## Development Status

All 48 HAL modules are implemented, covering all 73 PAC peripheral types (multi-instance peripherals like GPIO A–E, LPUART 0–3, TPM 0–3, etc. are handled by single modules). No remaining driver gaps.
