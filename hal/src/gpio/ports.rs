macro_rules! gpio_port {
    ($port_num:literal: $pac_port:ty => $mod_name:ident, Parts { $($pin_name:ident: $pin_num:literal),* $(,)? }) => {
        pub mod $mod_name {
            use crate::gpio::{GpioExt, Input, Pin};

            pub struct Parts {
                $(pub $pin_name: Pin<$port_num, $pin_num, Input>),*
            }

            impl GpioExt for $pac_port {
                type Parts = Parts;
                fn split(self) -> Parts {
                    Parts {
                        $($pin_name: Pin::new()),*
                    }
                }
            }
        }
    };
}

gpio_port!(0: crate::pac::Gpioa => gpioa, Parts {
    p0: 0, p1: 1, p2: 2, p3: 3, p4: 4,
    p9: 9, p10: 10,
    p14: 14, p15: 15,
    p17: 17, p18: 18, p19: 19, p20: 20, p21: 21, p22: 22, p23: 23,
    p24: 24, p25: 25, p26: 26, p27: 27, p28: 28,
    p30: 30, p31: 31,
});

gpio_port!(1: crate::pac::Gpiob => gpiob, Parts {
    p0: 0, p1: 1, p2: 2, p3: 3, p4: 4,
    p5: 5, p6: 6, p7: 7, p8: 8, p9: 9,
    p10: 10, p11: 11, p12: 12, p13: 13, p14: 14, p15: 15,
    p16: 16, p17: 17, p18: 18, p19: 19, p20: 20, p21: 21, p22: 22, p23: 23,
    p24: 24, p25: 25, p26: 26, p27: 27, p28: 28, p29: 29, p30: 30, p31: 31,
});

gpio_port!(2: crate::pac::Gpioc => gpioc, Parts {
    p0: 0, p1: 1, p2: 2, p3: 3, p4: 4,
    p5: 5, p6: 6, p7: 7, p8: 8, p9: 9,
    p10: 10, p11: 11, p12: 12, p13: 13, p14: 14, p15: 15,
    p16: 16, p17: 17, p18: 18, p19: 19, p20: 20, p21: 21, p22: 22, p23: 23,
    p24: 24, p25: 25, p26: 26, p27: 27, p28: 28, p29: 29, p30: 30, p31: 31,
});

gpio_port!(3: crate::pac::Gpiod => gpiod, Parts {
    p0: 0, p1: 1, p2: 2, p3: 3, p4: 4,
    p5: 5, p6: 6, p7: 7, p8: 8, p9: 9,
    p10: 10, p11: 11, p12: 12, p13: 13, p14: 14, p15: 15,
    p16: 16, p17: 17, p18: 18, p19: 19, p20: 20, p21: 21, p22: 22, p23: 23,
    p24: 24, p25: 25, p26: 26, p27: 27, p28: 28, p29: 29, p30: 30, p31: 31,
});

gpio_port!(4: crate::pac::Gpioe => gpioe, Parts {
    p0: 0, p1: 1, p2: 2, p3: 3, p4: 4,
    p5: 5, p6: 6, p7: 7,
});
