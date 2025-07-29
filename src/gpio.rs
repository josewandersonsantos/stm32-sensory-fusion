use crate::utils;
use crate::mcu;
pub const MODE_OUTPUT_10MHZ_PP: u32 = 0b0001;
pub const MODE_INPUT_OP: u32 = 0b1000;

pub fn enable_gpio_clock(port_base: u32)
{
    // Habilita o clock do GPIO (assumindo que o clock está no endereço 0x40021000)
    let rcc_apb2enr = mcu::RCC_APB2ENR as *mut u32;
    unsafe
    {
        let current = utils::read_register(rcc_apb2enr);
        let new_value = current | (1 << port_base);
        utils::write_register(rcc_apb2enr, new_value);
    }
}

// Configura o modo do pino (velocidade e direção)
pub fn set_mode_gpio(port_base: u32, pin: u32, mode_bits: u32)
{
    let shift = (pin % 8) * 4;

    let config_reg = if pin < 8
    {
        (port_base + 0x00) as *mut u32 // CRL
    }
    else
    {
        (port_base + 0x04) as *mut u32 // CRH
    };

    unsafe
    {
        utils::write_bits(config_reg, shift, mode_bits);
    }
}

pub fn get_gpio_pin_state(port_base: u32, pin: u32) -> bool
{
    let gpio_idr = (port_base + 0x08) as *const u32; // IDR
    unsafe
    {
        let value = utils::read_register(gpio_idr);
        (value & (1 << pin)) != 0 // Retorna true se o bit do pino estiver setado
    }
}

// Configura o pino com modo de saída push-pull 10MHz (LED, por exemplo)
pub fn set_cfg_gpio(port_base: u32, pin: u32)
{
    set_mode_gpio(port_base, pin, MODE_OUTPUT_10MHZ_PP);
}