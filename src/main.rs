// https://github.com/niekiran/embedded-rust

// cargo flash --chip STM32F103C8T6

// Diz pro compilador que não vamos usar a biblioteca padrão do Rust (std),
// porque ela depende de um sistema operacional (e no embutido não tem isso).
#![no_std]

// Também estamos dizendo que não vamos usar a função `main` tradicional do Rust.
// Vamos definir nosso próprio ponto de entrada, sem runtime automático.
#![no_main]

// Importa o tipo `PanicInfo`, que o compilador passa pra nossa função de panic handler.
// Ele contém informações sobre onde o panic aconteceu (arquivo, linha, etc).
use core::panic::PanicInfo;

mod startup_stm32f103;
mod led;
mod button;
mod mcu;
mod utils;
mod gpio;
mod exti;
mod proc;

const LED_PC13_PIN: u32 = mcu::PORTC_ADDR + mcu::GPIO13; // LED on PC13
const BTN_PA7_PIN: u32 = mcu::PORTA_ADDR + mcu::GPIO07;  // Button on PA7

// Esta é a função chamada quando algo dá errado (ex: um unwrap em None, panic!(), etc).
// O atributo #[panic_handler] avisa o compilador que essa é a função que ele deve chamar.
#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    // O tipo de retorno `!` significa que essa função nunca retorna (loop infinito).
    // Em sistemas embarcados é comum entrar num loop eterno em caso de erro.
    loop {}
}

// Essa função é o nosso ponto de entrada real. É chamada no boot do micro.
// O atributo #[no_mangle] garante que o nome da função no binário seja exatamente "main",
// do jeito que o linker (ou o sistema de boot) espera.
#[no_mangle]
// pub extern "C" fn main() -> ! {
fn main() -> !
{
    // Aqui vai o seu código principal, setup de periféricos, etc.
    // Neste exemplo, só vamos entrar em loop infinito.

    /*
    // RCC_APB2ENR — Offset 0x18 da base RCC (0x40021000)
    const RCC_APB2ENR: *mut u32 = (0x4002_1000 + 0x18) as *mut u32;
    // GPIOC_CRH — Base GPIOC (0x40011000) + Offset 0x04
    const GPIOC_CRH: *mut u32 = (0x4001_1000 + 0x04) as *mut u32;
    // GPIOC_ODR — GPIOC base + Offset 0x14
    const GPIOC_ODR: *mut u32 = (0x4001_1000 + 0x14) as *mut u32;
    unsafe {
        // 1. Habilita o clock do GPIOC, sem apagar outros bits
        let enr = RCC_APB2ENR.read_volatile();
        RCC_APB2ENR.write_volatile(enr | (1 << 4)); // IOPCEN

        // 2. Configura PC13 como saída push-pull, 2MHz
        let mut crh = GPIOC_CRH.read_volatile();
        crh &= !(0b1111 << 20);         // Limpa os bits do PC13
        crh |=  0b0010 << 20;           // MODE13 = 0b10 (2MHz), CNF13 = 0b00 (Push-pull)
        GPIOC_CRH.write_volatile(crh);

        // 3. Escreve 0 no PC13 = LED aceso
        let odr = GPIOC_ODR.read_volatile();
        GPIOC_ODR.write_volatile(odr & !(1 << 13));
    }
    */

    led::led_init(mcu::PORTC_ADDR, mcu::GPIO13);
    //led::led_off(mcu::PORTC_ADDR, mcu::GPIO13);
    led::led_toggle(mcu::PORTC_ADDR, mcu::GPIO13);
    
    //button::button_init(mcu::PORTA_ADDR, mcu::GPIO00, button::Mode::Interrupt(button::Trigger::FallingEdge));
    button::button_init(mcu::PORTA_ADDR, mcu::GPIO07, button::Mode::Interrupt(button::Trigger::FallingEdge));
    //button::button_configure_interrupt(mcu::PORTA_ADDR, mcu::GPIO07);

    loop
    {
        // led::led_toggle(mcu::PORTC_ADDR, mcu::GPIO13);

        // let mut cont = 0;
        // while cont < 100000
        // {
        //     cont += 1;
        // }

        // led::led_toggle(mcu::PORTC_ADDR, mcu::GPIO13);
        // let mut cont = 0;
        // while cont < 100000
        // {
        //     cont += 1;
        // }
    }
}

//button interrupt handler
#[allow(non_snake_case)]
#[no_mangle]
extern "C" fn EXTI9_5_Handler()
{
    led::led_toggle(mcu::PORTC_ADDR, mcu::GPIO13);
    // Clear the interrupt pending bit for EXTI line 7
    button::button_clear_interrupt(mcu::GPIO07);            
}

#[allow(non_snake_case)]
#[no_mangle]
extern "C" fn EXTI0_Handler()
{
    led::led_toggle(mcu::PORTC_ADDR, mcu::GPIO13);
    // Clear the interrupt pending bit for EXTI line 0
    button::button_clear_interrupt(mcu::GPIO00);
}