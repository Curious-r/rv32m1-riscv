## HAL Modules (drivers: 40, remaining peripheral types: 33)

All drivers use a consistent pointer-based register access pattern: `regs: &'static pac::module::RegisterBlock` initialized in `new()`.

| Module | Instance Selection | Capabilities Provided |
|--------|--------------------|-----------------------|
| **scg** | `Scg::new()` | `configure()` switches clock sources (FIRC/SIRC/SOSC/LPFLL); `clock_hz()`/`slow_hz()` for peripheral baud rate calculation. |
| **pcc** | PCC0 + PCC1 | `enable_*_clock()` clock gating for M4F (PCC0) and M0+ (PCC1). |
| **port** | `set_mux(PORT, PIN, mux)` / `set_pull(PORT, PIN, pull)` | Pin function and pull-up/pull-down (all 5 ports via PAC base addresses). |
| **gpio** | `Pin<PORT, PIN, MODE>` | Type-state Input/Output/Alternate (ports A–E). Traits: `OutputPin`, `InputPin`, `StatefulOutputPin`, `ErrorType`. |
| **trng** | `Trng::new()` | Entropy generator: `read_u32()`/`read_bytes()`/`read_words()`, error recovery, Von Neumann mode. |
| **sema42** | `Sema42::<0>::new()` (M4F), `Sema42::<1>::new()` (M0+) | 16-gate hardware semaphore with `SemaStatus` (LockedBy/Free) and `Processor` types; `try_lock()`, `unlock()`, `status()`, `reset_gate()`. |
| **rsim** | `Rsim::new()` | BLE/XCVR/ZIG/GEN clock gating, RF oscillator control, radio version ID. |
| **cau3** | `Cau3::new()` | CryptoCore + PKHA: 32 GPRs, semaphore lock, task command interface. |
| **lpit** | `Lpit::new_lpit0(pcc0)` / `new_lpit1(pcc1)` | `DelayNs` trait (channel 0, one-shot timer). |
| **lpspi** | `Lpspi::new_lpspi0–3(pcc, scg, pins)` | `SpiBus<u8>` (auto baud, AUTOPCS, CPOL/CPHA). |
| **lpi2c** | `Lpi2c::new_lpi2c0–3(pcc, scg, pins)` | `I2c<u8>` (START/STOP control, NACK detection). |
| **lpuart** | `Lpuart::new_lpuart0–3(pcc, scg, pins)` | Blocking putc/getc + `core::fmt::Write` + `serial::Read`/`Write` (embedded-hal 1.0). Auto baud (OSR + SBR). |
| **wdog** | `Wdog::new_0()` / `new_1()` | `unlock()`/`refresh()`/`disable()`/`configure()`. |
| **smc** | `Smc::new_0()` / `new_1()` | `PowerMode`, `configure(PMPROT)`, `set_mode(RUNM)`, `reset_cause(SRS)`. |
| **crc** | `Crc::new()` | 16/32-bit CRC with configurable polynomial, seed, transpose; `crc16_ccitt()`/`crc32()`. |
| **tpm** | `Tpm::new(ch, base)` | Edge-aligned PWM, input capture, output compare; prescaler 1–128. `TpmPwmPin<CH>` with `SetDutyCycle`. |
| **adc** | `Adc::new()` | Command-buffer: `configure()`/`set_channel()`/`trigger_conversion()`/`read_result()`. |
| **dma** | `Dma::new()` | 16-channel eDMA + DMAMUX: `memcpy()`, TCD entries. |
| **ftfe** | `Ftfe::new()` | Flash `erase_sector()`/`program_phrase()`/`program_check()`. |
| **usdhc** | `Usdhc::new()` | SD card CMD/RSP/DMA, 4/8-bit bus. |
| **usb** | `Usb::new()` | USB device BDT: `configure_endpoint()`/`prepare_rx()`/`prepare_tx()`. |
| **i2s** | `I2s::new()` | I2S/SAI audio: master/slave, configurable frame/word width. |
| **lpcmp** | `Lpcmp::new_0()` / `new_1()` | Low-power comparator: `select_inputs()`/`set_hysteresis()`/`set_filter()`/`configure_dac()`. |
| **lpdac** | `Lpdac::new()` | 12-bit DAC with buffer/FIFO/return-to-zero, `write_and_trigger()`. |
| **mua** | `Mua::new()` | Inter-core communication: 4 channels `send()`/`receive()` (blocking and non-blocking). |
| **trgmux** | `Trgmux::new()` | Trigger routing: `set_sel0()` with locked variant, 64 trigger sources. |
| **sim** | `Sim::new()` | `family_id()`/`subfamily_id()`/`unique_id()`/`mac_address()`. |
| **vref** | `Vref::new()` | Voltage reference: `enable(mode)`/`set_trim()`/`set_chop()`, 2.1 V output. |
| **usbvreg** | `Usbvreg::new()` | USB regulator `enable()`/`disable()`, standby modes, unlock. |
| **llwu** | `Llwu0::new()` / `Llwu1::new()` | 32 wakeup pins, 7 module sources, 2 digital filters. |
| **spm** | `Spm::new()` | `core_power_mode()`/`regulator_sel()`, LDO status, LVD/HVD. |
| **ewm** | `Ewm::new()` | External watchdog: windowed CMPL/CMPH, refresh `0xB4 → 0x4B`. |
| **emvsim** | `Emvsim::new()` | Smart card clock/VCC/card reset/presence, blocking byte I/O. |
| **lptmr** | `Lptmr::new_0()` / `new_1()` / `new_2()` | 16-bit low-power timer: pulse count, time compare, periodic interrupt. |
| **tstmr** | `Tstmr::new()` | 56-bit free-running counter: `read() -> u64`. |

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

- ✅ **Driver refactoring**: All modules converted from unit structs to pointer-based `regs` pattern
- ✅ **critical-section**: Custom RISC-V single-hart impl (MIE via csrrci/csrsi) — no external dependency
- ✅ **Embedded HAL traits**: `DelayNs`, `SpiBus<u8>`, `I2c<u8>`, `serial::Read`/`Write`, `SetDutyCycle`, `StatefulOutputPin`
- ✅ **Edition 2024**: PAC rt feature compiles with `unsafe extern "C"` and `#[unsafe(no_mangle)]`
- ⬜ **AXBS, FB, MCM, MSCM, XRDC**: Drivers not yet implemented (33 remaining peripheral types)
