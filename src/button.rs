use crate::{exti, gpio, proc, mcu, utils};

pub enum ButtonStatus
{
    Pressed,
    Released,
}
pub enum Trigger
{
    FallingEdge,
    RisingEdge,
}
pub enum Mode
{
    Input,
    Interrupt(Trigger),
}

pub fn button_init(port:u32, pin: u32, mode: Mode)
{
    // Enable the GPIO clock for the port // Bit 2 = GPIOA enable
    gpio::enable_gpio_clock(0x02);
    gpio::set_mode_gpio(port, pin, gpio::MODE_INPUT_OP);
    exti::configure_afio(port, pin);

    match mode
    {
        Mode::Input => { /* Nothing to do for input mode */ },
        Mode::Interrupt(trigger) =>
        {
            // Configure the interrupt based on the trigger type
            match trigger
            {
                Trigger::FallingEdge =>
                {
                    // Configure for falling edge interrupt
                    // This would typically involve setting up the EXTI registers
                    exti::gpio::set_edge(pin, exti::gpio::EdgeTrigger::Falling);
                },
                Trigger::RisingEdge =>
                {
                    // Configure for rising edge interrupt
                    // This would typically involve setting up the EXTI registers
                    exti::gpio::set_edge(pin, exti::gpio::EdgeTrigger::Rising);
                },
            }

            // Enable the interrupt for the pin
            exti::enable_interrupt(pin);

            // Enable the NVIC interrupt for the EXTI line
            if let Some(irq_number) = mcu::IRQn::from_pin(pin)
            {
                proc::enable_irq(irq_number);
            }
        },

    }
}

pub fn button_configure_interrupt(port:u32, pin: u32)
{

}

pub fn button_clear_interrupt(pin: u32)
{
    // Clear the interrupt pending bit for the EXTI line
    unsafe
    {
        let exti_pr = mcu::EXTI_PR as *mut u32;
        utils::write_bits(exti_pr, pin, 0x01);
    }
}

pub fn button_read_status(port:u32, pin: u32) -> ButtonStatus
{
    if gpio::get_gpio_pin_state(port, pin)
    {
        ButtonStatus::Pressed
    }
    else
    {
        ButtonStatus::Released
    }
}
