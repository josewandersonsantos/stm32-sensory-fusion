# Sensory Fusion 🚀 (In Progress)

Projeto embarcado em Rust para fusão sensorial com **Blue Pill (STM32F103)**, utilizando os sensores:

- 🛰️ **GPS NEO-6M** para localização geográfica (via protocolo NMEA)
- 🧭 **MPU6050** para leitura de aceleração e giroscópio

## Objetivo

Integrar e processar dados de sensores para criar uma base de navegação confiável e de alta precisão, utilizando conceitos de *sensor fusion* em sistemas embarcados.

## Funcionalidades
- Leitura de dados do GPS via UART (com parsing do protocolo NMEA)
- Comunicação com MPU6050 via I2C
- Integração futura com filtro de fusão sensorial (ex: Complementar, Kalman)
- Escrita em **Rust** com `#![no_std]`, focando em eficiência e segurança

## Estrutura do Projeto

```txt
src/
├── main.rs           # Ponto de entrada
├── gps.rs            # Parser NMEA e leitura UART
├── mpu.rs            # Leitura do MPU6050 via I2C
├── fusion.rs         # Lógica de fusão sensorial (em breve)
└── utils.rs          # Funções auxiliares
```

## Como rodar 🛠️

> Requer toolchain de Rust para embarcados, como `thumbv7m-none-eabi`, além do `probe-rs` ou `openocd` para upload.

```bash
rustup target add thumbv7m-none-eabi

cargo build --release --target thumbv7m-none-eabi

# Upload pode variar conforme seu setup:
cargo flash --chip STM32F103C8 --release
```

<!-- ## Dependências

- [`embedded-hal`](https://docs.rs/embedded-hal)
- [`cortex-m`](https://docs.rs/cortex-m)
- [`cortex-m-rt`](https://docs.rs/cortex-m-rt)
- [`stm32f1xx-hal`](https://docs.rs/stm32f1xx-hal)
- [`nb`, `heapless`, etc.] -->

## Licença

MIT © José — Projeto educacional para explorar fusão sensorial em sistemas embarcados.

---
