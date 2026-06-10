## HAL Modules (33/33 Peripherals)

| Module | Capabilities Provided |
|--------|-----------------------|
| **scg** | `configure()` switches clock sources (FIRC/SIRC/SOSC/LPFLL); `clock_hz()`/`slow_hz()` are used for peripheral baud rate calculation. |
| **pcc** | `enable_*_clock()` clock gating functions (port, lpspi, lpi2c, lpuart, lpit, lpdac, mua). |
| **port** | `set_mux()`/`set_pull()` configure pin function and pull-up/pull-down. |
| **gpio** | `Pin<PORT, PIN, MODE>` type-state machine (Input/Output/Alternate); macros generate ports A/B/C/D/E. |
| **lpit** | Implements the `embedded-hal 1.0` `DelayNs` trait (LPIT channel 0, one-shot timer). |
| **lpspi** | Implements `embedded-hal 1.0` `SpiBus<u8>` (auto baud rate calculation; AUTOPCS, CPOL, CPHA configurable). |
| **lpi2c** | Implements `embedded-hal 1.0` `I2c<u8>` (automatic START/STOP control; NACK detection). |
| **lpuart** | Blocking `putc`/`getc` interface + `core::fmt::Write` trait (auto baud rate calculation using OSR + SBR). |
| **wdog** | Hardware watchdog: `new()`/`unlock()`/`refresh()`/`disable()`/`configure()`. |
| **smc** | Power mode awareness: `PowerMode` (RUN/STOP/VLPR/HSRUN), `configure(PMPROT)`, `set_mode(RUNM)`, `reset_cause(SRS)`. |
| **crc** | Hardware CRC: `CrcConfig` (16/32-bit, polynomial, seed, transpose); convenience methods `crc16_ccitt()`/`crc32()`. |
| **tpm** | Three instances (TPM0/1/2): edge-aligned PWM, input capture, output compare; prescaler 1–128; polarity control. |
| **adc** | Command-buffer architecture: `configure()` (power/reference), `set_channel()` + `trigger_conversion()` + `read_result()`. |
| **dma** | 16-channel eDMA + DMAMUX: `DmaTransferConfig` (address/offset/adjust/size), `memcpy()`, 16-channel TCD. |
| **ftfe** | Flash erase/program: `erase_sector()`/`program_phrase()`/`program_check()`; FCCOB command sequence. |
| **usdhc** | SD card driver: `send_command()` + `read_block()`/`write_block()`; full CMD/RSP/DMA support, 4/8-bit bus. |
| **usb** | USB device controller: BDT architecture, `configure_endpoint()`/`prepare_rx()`/`prepare_tx()`, interrupt/error handling. |
| **i2s** | I2S/SAI audio interface: `configure_tx()`/`configure_rx()` master/slave mode, blocking read/write, configurable frame/word width and clocks. |
| **lpcmp** | Low-power comparator: `select_inputs()`/`set_hysteresis()`/`set_filter()`/`configure_dac()`, interrupt or polling. |
| **lpdac** | 12-bit DAC: `configure()` buffer/FIFO/return-to-zero mode, `write_and_trigger()`, DMA enable. |
| **mua** | Inter-core communication: 4 channels `send()`/`receive()` (blocking/non-blocking), flag passing, NMI/GP interrupts. |
| **trgmux** | Trigger routing: `set_sel0()`/`set_sel0_locked()` across 25 peripherals, 64 trigger source selections. |
| **sim** | System integration: `unique_id()`/`revision()`/`mac_address()`, Flash/SRAM capacity, FlexBus security level, SysTick clock gating. |
| **vref** | Voltage reference: `enable(mode)`/`disable()`, `set_trim()`, `set_chop()`, compensation/regulator enable, 2.1 V output. |
| **usbvreg** | USB regulator: `enable()`/`disable()`, STOP/VLPR standby modes, unlock sequence. |
| **llwu** | Low-leakage wakeup: 32 wakeup pins (rising/falling/either edge), 7 module wakeup sources, 2 digital filters (independent edge/DMA mode), pin/filter flags. |
| **spm** | System power management: `core_power_mode()`/`regulator_sel()`, Core/USB/RTC LDO status, LVD/HVD flags. |
| **ewm** | External watchdog: `enable()`/`configure()`, windowed comparison (CMPL/CMPH), clock prescaler, refresh sequence (0xB4 → 0x4B). |
| **emvsim** | Smart card interface: clock prescaler/divisor, VCC enable/deactivate, card reset/presence detection, blocking byte transmit/receive. |
| **tstmr** | Test timer: 56-bit free-running counter, `read()` returns a `u64`, low/high half-words read independently. |

## Development Notes

### PAC Register Access Pattern

```rust
p.scg().rccr().write(|w| unsafe { w.divcore().bits(0).divslow().bits(1).scs().bits(3) });
p.pcc0().pcc_porta().write(|w| w.cgc().cgc_1());
```

### GPIO Typical Usage

```rust
let gpioa = p.gpioa.split();
let mut led = gpioa.p24.into_output();
led.set_high().ok();
```

### DMA Typical Usage (Memory Copy)

```rust
let dma = Dma::new();
dma.enable();
let mut buf = [0u8; 64];
dma.memcpy(buf.as_mut_ptr(), src_ptr, 64);
```

### Build Commands

```powershell
cargo build --workspace
cargo build --release --workspace
```

### Known Issues

- PORTE does not physically exist on the RV32M1 (address 0x4004_A000 is occupied by ADC0). GPIOE pins have no PORT module; `set_mux()`/`set_pull()` are automatically skipped. GPIO input/output works normally.
- LPIT1, TPM3, LPI2C3, LPSPI3, LPUART3, and LPCMP1 are not defined in this SVD variant. It has been confirmed that NXP’s official website offers no newer SVD – this is a fixed chip limitation.
