## HAL Modules (37/73 Peripheral Types)

| Module | Instance Support | Capabilities Provided |
|--------|-----------------|-----------------------|
| **scg** | — | `configure()` switches clock sources (FIRC/SIRC/SOSC/LPFLL); `clock_hz()`/`slow_hz()` are used for peripheral baud rate calculation. |
| **pcc** | PCC0 + PCC1 | `enable_*_clock()` clock gating functions for M4F (PCC0: porta-d, lpspi0-2, lpi2c0-2, lpuart0-2, tpm0-2, lpit0) and M0+ (PCC1: porte, cau3, trng, lpit1, tpm3, lpi2c3, lpspi3, lpuart3). |
| **port** | PORTA–E | `set_mux()`/`set_pull()` configure pin function and pull-up/pull-down. PORTE at `0x4103_7000`. |
| **gpio** | A/B/C/D/E | `Pin<PORT, PIN, MODE>` type-state machine (Input/Output/Alternate); macros generate ports A/B/C/D/E. |
| **trng** | — | TRNG entropy generator: blocking `read_u32()`/`read_bytes()`/`read_words()`, entropy valid polling, error recovery, Von Neumann sample mode. PCC1 clock gating. |
| **sema42** | SEMA420 (M4F), SEMA421 (M0+) | 16-gate hardware semaphore: `try_lock(gate, proc)`/`unlock(gate)`/`is_locked()`/`locked_by()`. Pointer-cast from either PAC instance. |
| **rsim** | — | Radio System Interface: BLE/XCVR/ZIG/GEN clock gating, RF oscillator control, power mode management, radio version ID. |
| **cau3** | — | CryptoCore + PKHA accelerator: module enable/reset, 7 interrupt sources, 32 GPRs (R0–R31 + SP + LR), semaphore lock, task command interface. PCC1 clock gating. |
| **lpit** | LPIT0 (PCC0), LPIT1 (PCC1) | Implements `embedded-hal 1.0` `DelayNs` trait (channel 0, one-shot timer). |
| **lpspi** | LPSPI0–3 (PCC0: 0-2, PCC1: 3) | Implements `embedded-hal 1.0` `SpiBus<u8>` (auto baud rate calculation; AUTOPCS, CPOL, CPHA configurable). |
| **lpi2c** | LPI2C0–3 (PCC0: 0-2, PCC1: 3) | Implements `embedded-hal 1.0` `I2c<u8>` (automatic START/STOP control; NACK detection). |
| **lpuart** | LPUART0–3 (PCC0: 0-2, PCC1: 3) | Blocking `putc`/`getc` interface + `core::fmt::Write` trait (auto baud rate calculation using OSR + SBR). |
| **wdog** | WDOG0, WDOG1 | Hardware watchdog: `new()`/`unlock()`/`refresh()`/`disable()`/`configure()`. |
| **smc** | SMC0, SMC1 | Power mode awareness: `PowerMode` (RUN/STOP/VLPR/HSRUN), `configure(PMPROT)`, `set_mode(RUNM)`, `reset_cause(SRS)`. SMC1 via static methods. |
| **crc** | — | Hardware CRC: `CrcConfig` (16/32-bit, polynomial, seed, transpose); convenience methods `crc16_ccitt()`/`crc32()`. |
| **tpm** | TPM0/1/2 (PCC0), TPM3 (PCC1) | Edge-aligned PWM, input capture, output compare; prescaler 1–128; polarity control. |
| **adc** | — | Command-buffer architecture: `configure()` (power/reference), `set_channel()` + `trigger_conversion()` + `read_result()`. |
| **dma** | — | 16-channel eDMA + DMAMUX: `DmaTransferConfig` (address/offset/adjust/size), `memcpy()`, 16-channel TCD. |
| **ftfe** | — | Flash erase/program: `erase_sector()`/`program_phrase()`/`program_check()`; FCCOB command sequence. |
| **usdhc** | — | SD card driver: `send_command()` + `read_block()`/`write_block()`; full CMD/RSP/DMA support, 4/8-bit bus. |
| **usb** | — | USB device controller: BDT architecture, `configure_endpoint()`/`prepare_rx()`/`prepare_tx()`, interrupt/error handling. |
| **i2s** | — | I2S/SAI audio interface: `configure_tx()`/`configure_rx()` master/slave mode, blocking read/write, configurable frame/word width and clocks. |
| **lpcmp** | LPCMP0, LPCMP1 | Low-power comparator: `select_inputs()`/`set_hysteresis()`/`set_filter()`/`configure_dac()`, interrupt or polling. |
| **lpdac** | — | 12-bit DAC: `configure()` buffer/FIFO/return-to-zero mode, `write_and_trigger()`, DMA enable. |
| **mua** | — | Inter-core communication: 4 channels `send()`/`receive()` (blocking/non-blocking), flag passing, NMI/GP interrupts. |
| **trgmux** | — | Trigger routing: `set_sel0()`/`set_sel0_locked()` across 25 peripherals, 64 trigger source selections. |
| **sim** | — | System integration: `unique_id()`/`revision()`/`mac_address()`, Flash/SRAM capacity, FlexBus security level, SysTick clock gating. |
| **vref** | — | Voltage reference: `enable(mode)`/`disable()`, `set_trim()`, `set_chop()`, compensation/regulator enable, 2.1 V output. |
| **usbvreg** | — | USB regulator: `enable()`/`disable()`, STOP/VLPR standby modes, unlock sequence. |
| **llwu** | LLWU0, LLWU1 | Low-leakage wakeup: 32 wakeup pins (rising/falling/either edge), 7 module wakeup sources, 2 digital filters (independent edge/DMA mode), pin/filter flags. |
| **spm** | — | System power management: `core_power_mode()`/`regulator_sel()`, Core/USB/RTC LDO status, LVD/HVD flags. |
| **ewm** | — | External watchdog: `enable()`/`configure()`, windowed comparison (CMPL/CMPH), clock prescaler, refresh sequence (0xB4 → 0x4B). |
| **emvsim** | — | Smart card interface: clock prescaler/divisor, VCC enable/deactivate, card reset/presence detection, blocking byte transmit/receive. |
| **lptmr** | LPTMR0/1/2 | 16-bit low-power timer: pulse counting, time comparison, periodic interrupt. |
| **tstmr** | — | Test timer: 56-bit free-running counter, `read()` returns a `u64`, low/high half-words read independently. |

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

- Several HAL modules (Adc, Crc, Flexio, Ewm, Rtc, etc.) still use unit structs + `unsafe { &*ptr() }` — not yet refactored to the pointer-based pattern used by multi-instance drivers.
- No `critical-section` implementation is provided yet — global state in TRNG and other modules is unsafe without it.
