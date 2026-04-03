#![allow(dead_code)]

pub enum Type
{
    IWDG,
    WWDG,
}

// =========================================================
// IWDG IMPLEMENTATION
// =========================================================
pub mod iwdg
{
    use crate::{mcu, utils};

    pub fn init(timeout_ms: u32)
    {
        let iwdg_kr = mcu::IWDG_KR as *mut u32;
        let iwdg_pr = mcu::IWDG_PR as *mut u32;
        let iwdg_rlr = mcu::IWDG_RLR as *mut u32;

        const KEY_ENABLE: u32  = 0xCCCC;
        const KEY_REFRESH: u32 = 0xAAAA;
        const KEY_WRITE: u32   = 0x5555;

        unsafe
        {
            utils::write_register(iwdg_kr, KEY_WRITE);

            // prescaler = 32
            utils::write_register(iwdg_pr, 0x03);

            let reload = (timeout_ms * 1250) / 1000;
            utils::write_register(iwdg_rlr, reload);

            utils::write_register(iwdg_kr, KEY_REFRESH);
            utils::write_register(iwdg_kr, KEY_ENABLE);
        }
    }
   
    
    pub fn refresh()
    {
        let iwdg_kr = mcu::IWDG_KR as *mut u32;

        const KEY_REFRESH: u32 = 0xAAAA;
        
        unsafe
        {
            utils::write_register(iwdg_kr, KEY_REFRESH);
        }
    }
}

// =========================================================
// WWDG IMPLEMENTATION
// =========================================================
pub mod wwdg
{
    use crate::{mcu, rcc, utils};

    fn init(_timeout_ms: u32)
    {        
        let wwdg_cr = mcu::WWDG_CR as *mut u32;
        let wwdg_cfr = mcu::WWDG_CFR as *mut u32;

        unsafe
        {
            // Enable clock WWDG
            rcc::apb1::enable(rcc::apb1::Apb1Peripheral::Wwdg);
            // Config window + prescaler
            utils::write_register(wwdg_cfr, (0b111 << 7) | (0b011 << 0)); 
            // Start + counter
            utils::write_register(wwdg_cr, 0x7F | (1 << 7));
        }
    }

    fn refresh()
    {
        let wwdg_cr = mcu::WWDG_CR as *mut u32;
        unsafe
        {
            utils::write_register(wwdg_cr, 0x7F);
        }
    }

    fn disable()
    {
        let rcc_apb1enr = mcu::RCC_APB1ENR as *mut u32;
        utils::clear_bit(rcc_apb1enr, 11);
    }
}
