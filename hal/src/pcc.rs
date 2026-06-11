use crate::pac;

pub fn enable_port_clock(pcc: &pac::Pcc0, port: u8) {
    match port {
        0 => { pcc.pcc_porta().write(|w| w.cgc().cgc_1()); }
        1 => { pcc.pcc_portb().write(|w| w.cgc().cgc_1()); }
        2 => { pcc.pcc_portc().write(|w| w.cgc().cgc_1()); }
        3 => { pcc.pcc_portd().write(|w| w.cgc().cgc_1()); }
        _ => {}
    }
}

pub fn enable_lpspi_clock(pcc: &pac::Pcc0, instance: u8) {
    match instance {
        0 => { pcc.pcc_lpspi0().write(|w| w.pcs().pcs_3().cgc().cgc_1()); }
        1 => { pcc.pcc_lpspi1().write(|w| w.pcs().pcs_3().cgc().cgc_1()); }
        2 => { pcc.pcc_lpspi2().write(|w| w.pcs().pcs_3().cgc().cgc_1()); }
        _ => {}
    }
}

pub fn enable_lpi2c_clock(pcc: &pac::Pcc0, instance: u8) {
    match instance {
        0 => { pcc.pcc_lpi2c0().write(|w| w.pcs().pcs_3().cgc().cgc_1()); }
        1 => { pcc.pcc_lpi2c1().write(|w| w.pcs().pcs_3().cgc().cgc_1()); }
        2 => { pcc.pcc_lpi2c2().write(|w| w.pcs().pcs_3().cgc().cgc_1()); }
        _ => {}
    }
}

pub fn enable_lpit0_clock(pcc: &pac::Pcc0) {
    pcc.pcc_lpit0().write(|w| w.pcs().pcs_3().cgc().cgc_1());
}

pub fn enable_lpuart_clock(pcc: &pac::Pcc0, instance: u8) {
    match instance {
        0 => { pcc.pcc_lpuart0().write(|w| w.pcs().pcs_3().cgc().cgc_1()); }
        1 => { pcc.pcc_lpuart1().write(|w| w.pcs().pcs_3().cgc().cgc_1()); }
        2 => { pcc.pcc_lpuart2().write(|w| w.pcs().pcs_3().cgc().cgc_1()); }
        _ => {}
    }
}

pub fn enable_tpm_clock(pcc: &pac::Pcc0, instance: u8) {
    match instance {
        0 => { pcc.pcc_tpm0().write(|w| w.pcs().pcs_3().cgc().cgc_1()); }
        1 => { pcc.pcc_tpm1().write(|w| w.pcs().pcs_3().cgc().cgc_1()); }
        2 => { pcc.pcc_tpm2().write(|w| w.pcs().pcs_3().cgc().cgc_1()); }
        _ => {}
    }
}

pub fn enable_porte_clock(pcc: &pac::Pcc1) {
    pcc.pcc_porte().write(|w| w.cgc().cgc_1());
}

pub fn enable_cau3_clock(pcc: &pac::Pcc1) {
    pcc.pcc_cau3().write(|w| w.cgc().cgc_1());
}

pub fn enable_trng_clock(pcc: &pac::Pcc1) {
    pcc.pcc_trng().write(|w| w.cgc().cgc_1());
}

pub fn enable_lpit1_clock(pcc: &pac::Pcc1) {
    pcc.pcc_lpit1().write(|w| w.pcs().pcs_3().cgc().cgc_1());
}

pub fn enable_tpm3_clock(pcc: &pac::Pcc1) {
    pcc.pcc_tpm3().write(|w| w.pcs().pcs_3().cgc().cgc_1());
}

pub fn enable_lpi2c3_clock(pcc: &pac::Pcc1) {
    pcc.pcc_lpi2c3().write(|w| w.pcs().pcs_3().cgc().cgc_1());
}

pub fn enable_lpspi3_clock(pcc: &pac::Pcc1) {
    pcc.pcc_lpspi3().write(|w| w.pcs().pcs_3().cgc().cgc_1());
}

pub fn enable_lpuart3_clock(pcc: &pac::Pcc1) {
    pcc.pcc_lpuart3().write(|w| w.pcs().pcs_3().cgc().cgc_1());
}

pub fn enable_adc0_clock(pcc: &pac::Pcc0) {
    pcc.pcc_adc0().write(|w| w.pcs().pcs_3().cgc().cgc_1());
}

pub fn enable_crc0_clock(pcc: &pac::Pcc0) {
    pcc.pcc_crc0().write(|w| w.cgc().cgc_1());
}

pub fn enable_sema42_0_clock(pcc: &pac::Pcc0) {
    pcc.pcc_sema42_0().write(|w| w.cgc().cgc_1());
}

pub fn enable_sema42_1_clock(pcc: &pac::Pcc1) {
    pcc.pcc_sema42_1().write(|w| w.cgc().cgc_1());
}

pub fn enable_ewm_clock(pcc: &pac::Pcc0) {
    pcc.pcc_ewm().write(|w| w.cgc().cgc_1());
}

pub fn enable_mua_clock(pcc: &pac::Pcc0) {
    pcc.pcc_mua().write(|w| w.cgc().cgc_1());
}

pub fn enable_lpdac0_clock(pcc: &pac::Pcc0) {
    pcc.pcc_lpdac0().write(|w| w.cgc().cgc_1());
}
