#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn TAMP();
    fn RAMCFG();
    fn FLASH();
    fn FLASH_S();
    fn GTZC();
    fn RCC();
    fn RCC_S();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn EXTI5();
    fn EXTI6();
    fn EXTI7();
    fn EXTI8();
    fn EXTI9();
    fn EXTI10();
    fn EXTI11();
    fn EXTI12();
    fn EXTI13();
    fn EXTI14();
    fn EXTI15();
    fn GPDMA1_CH0();
    fn GPDMA1_CH1();
    fn GPDMA1_CH2();
    fn GPDMA1_CH3();
    fn GPDMA1_CH4();
    fn GPDMA1_CH5();
    fn GPDMA1_CH6();
    fn GPDMA1_CH7();
    fn IWDG();
    fn SAES();
    fn ADC1();
    fn DAC1();
    fn FDCAN1_IT0();
    fn FDCAN1_IT1();
    fn TIM1_BRK_TERR_IERR();
    fn TIM1_UP();
    fn TIM1_TRG_COM_DIR_IDX();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn TIM5();
    fn TIM6();
    fn TIM7();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn SPI3();
    fn USART1();
    fn USART2();
    fn USART3();
    fn UART4();
    fn UART5();
    fn LPUART1();
    fn LPTIM1();
    fn IM8_BRK_TERR_IERR();
    fn TIM8_UP();
    fn TIM8_TRG_COM_DIR_IDX();
    fn TIM8_CC();
    fn ADC2();
    fn LPTIM2();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn USB_FS();
    fn CRS();
    fn UCPD1();
    fn FMC();
    fn OCTOSPI1();
    fn SDMMC1();
    fn I2C3_EV();
    fn I2C3_ER();
    fn SPI4();
    fn SPI5();
    fn SPI6();
    fn USART6();
    fn USART10();
    fn USART11();
    fn SAI1();
    fn SAI2();
    fn GPDMA2_CH0();
    fn GPDMA2_CH1();
    fn GPDMA2_CH2();
    fn GPDMA2_CH3();
    fn GPDMA2_CH4();
    fn GPDMA2_CH5();
    fn GPDMA2_CH6();
    fn GPDMA2_CH7();
    fn UART7();
    fn UART8();
    fn UART9();
    fn UART12();
    fn SDMMC2();
    fn ICACHE();
    fn DCACHE();
    fn ETH();
    fn ETH_WKUP();
    fn DCMI_PSSI();
    fn FDCAN2_IT0();
    fn FDCAN2_IT1();
    fn Cordic();
    fn FMAC();
    fn DTS_WKUP();
    fn RNG();
    fn OTFDEC1();
    fn AES();
    fn HASH();
    fn PKA();
    fn TIM12();
    fn TIM13();
    fn TIM14();
    fn I3C1_EV();
    fn I3C1_ER();
    fn I2C4_EV();
    fn I2C4_ER();
    fn LPTIM3();
    fn LPTIM4();
    fn LPTIM5();
    fn LPTIM6();
}

#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 131] = [
    Vector { _handler: WWDG },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TAMP },
    Vector { _handler: RAMCFG },
    Vector { _handler: FLASH },
    Vector { _handler: FLASH_S },
    Vector { _handler: GTZC },
    Vector { _handler: RCC },
    Vector { _handler: RCC_S },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: EXTI5 },
    Vector { _handler: EXTI6 },
    Vector { _handler: EXTI7 },
    Vector { _handler: EXTI8 },
    Vector { _handler: EXTI9 },
    Vector { _handler: EXTI10 },
    Vector { _handler: EXTI11 },
    Vector { _handler: EXTI12 },
    Vector { _handler: EXTI13 },
    Vector { _handler: EXTI14 },
    Vector { _handler: EXTI15 },
    Vector {
        _handler: GPDMA1_CH0,
    },
    Vector {
        _handler: GPDMA1_CH1,
    },
    Vector {
        _handler: GPDMA1_CH2,
    },
    Vector {
        _handler: GPDMA1_CH3,
    },
    Vector {
        _handler: GPDMA1_CH4,
    },
    Vector {
        _handler: GPDMA1_CH5,
    },
    Vector {
        _handler: GPDMA1_CH6,
    },
    Vector {
        _handler: GPDMA1_CH7,
    },
    Vector { _handler: IWDG },
    Vector { _handler: SAES },
    Vector { _handler: ADC1 },
    Vector { _handler: DAC1 },
    Vector {
        _handler: FDCAN1_IT0,
    },
    Vector {
        _handler: FDCAN1_IT1,
    },
    Vector {
        _handler: TIM1_BRK_TERR_IERR,
    },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM_DIR_IDX,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: TIM5 },
    Vector { _handler: TIM6 },
    Vector { _handler: TIM7 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: SPI3 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPTIM1 },
    Vector {
        _handler: IM8_BRK_TERR_IERR,
    },
    Vector { _handler: TIM8_UP },
    Vector {
        _handler: TIM8_TRG_COM_DIR_IDX,
    },
    Vector { _handler: TIM8_CC },
    Vector { _handler: ADC2 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: USB_FS },
    Vector { _handler: CRS },
    Vector { _handler: UCPD1 },
    Vector { _handler: FMC },
    Vector { _handler: OCTOSPI1 },
    Vector { _handler: SDMMC1 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _handler: SPI4 },
    Vector { _handler: SPI5 },
    Vector { _handler: SPI6 },
    Vector { _handler: USART6 },
    Vector { _handler: USART10 },
    Vector { _handler: USART11 },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector {
        _handler: GPDMA2_CH0,
    },
    Vector {
        _handler: GPDMA2_CH1,
    },
    Vector {
        _handler: GPDMA2_CH2,
    },
    Vector {
        _handler: GPDMA2_CH3,
    },
    Vector {
        _handler: GPDMA2_CH4,
    },
    Vector {
        _handler: GPDMA2_CH5,
    },
    Vector {
        _handler: GPDMA2_CH6,
    },
    Vector {
        _handler: GPDMA2_CH7,
    },
    Vector { _handler: UART7 },
    Vector { _handler: UART8 },
    Vector { _handler: UART9 },
    Vector { _handler: UART12 },
    Vector { _handler: SDMMC2 },
    Vector { _reserved: 0 },
    Vector { _handler: ICACHE },
    Vector { _handler: DCACHE },
    Vector { _handler: ETH },
    Vector { _handler: ETH_WKUP },
    Vector {
        _handler: DCMI_PSSI,
    },
    Vector {
        _handler: FDCAN2_IT0,
    },
    Vector {
        _handler: FDCAN2_IT1,
    },
    Vector { _handler: Cordic },
    Vector { _handler: FMAC },
    Vector { _handler: DTS_WKUP },
    Vector { _handler: RNG },
    Vector { _handler: OTFDEC1 },
    Vector { _handler: AES },
    Vector { _handler: HASH },
    Vector { _handler: PKA },
    Vector { _reserved: 0 },
    Vector { _handler: TIM12 },
    Vector { _handler: TIM13 },
    Vector { _handler: TIM14 },
    Vector { _handler: I3C1_EV },
    Vector { _handler: I3C1_ER },
    Vector { _handler: I2C4_EV },
    Vector { _handler: I2C4_ER },
    Vector { _handler: LPTIM3 },
    Vector { _handler: LPTIM4 },
    Vector { _handler: LPTIM5 },
    Vector { _handler: LPTIM6 },
];

/// Available interrupts for this device
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 4: TAMP global interrupt
    TAMP = 4,
    /// 5: RAM configuration global interrupt
    RAMCFG = 5,
    /// 6: FLASH non-secure global interrupt
    FLASH = 6,
    /// 7: FLASH secure global interrupt
    FLASH_S = 7,
    /// 8: GTZC global interrupt
    GTZC = 8,
    /// 9: RCC non-secure global interrupt
    RCC = 9,
    /// 10: RCC secure global interrupt
    RCC_S = 10,
    /// 11: EXTI Line0 interrupt
    EXTI0 = 11,
    /// 12: EXTI Line1 interrupt
    EXTI1 = 12,
    /// 13: EXTI Line2 interrupt
    EXTI2 = 13,
    /// 14: EXTI Line3 interrupt
    EXTI3 = 14,
    /// 15: EXTI Line4 interrupt
    EXTI4 = 15,
    /// 16: EXTI Line5 interrupt
    EXTI5 = 16,
    /// 17: EXTI Line6 interrupt
    EXTI6 = 17,
    /// 18: EXTI Line7 interrupt
    EXTI7 = 18,
    /// 19: EXTI Line8 interrupt
    EXTI8 = 19,
    /// 20: EXTI Line9 interrupt
    EXTI9 = 20,
    /// 21: EXTI Line10 interrupt
    EXTI10 = 21,
    /// 22: EXTI Line11 interrupt
    EXTI11 = 22,
    /// 23: EXTI Line12 interrupt
    EXTI12 = 23,
    /// 24: EXTI Line13 interrupt
    EXTI13 = 24,
    /// 25: EXTI Line14 interrupt
    EXTI14 = 25,
    /// 26: EXTI Line15 interrupt
    EXTI15 = 26,
    /// 27: GPDMA1 channel 0 global interrupt
    GPDMA1_CH0 = 27,
    /// 28: GPDMA1 channel 1 global interrupt
    GPDMA1_CH1 = 28,
    /// 29: GPDMA1 channel 2 global interrupt
    GPDMA1_CH2 = 29,
    /// 30: GPDMA1 channel 3 global interrupt
    GPDMA1_CH3 = 30,
    /// 31: GPDMA1 channel 4 global interrupt
    GPDMA1_CH4 = 31,
    /// 32: GPDMA1 channel 5 global interrupt
    GPDMA1_CH5 = 32,
    /// 33: GPDMA1 channel 6 global interrupt
    GPDMA1_CH6 = 33,
    /// 34: GPDMA1 channel 7 global interrupt
    GPDMA1_CH7 = 34,
    /// 35: IWDG interrupt
    IWDG = 35,
    /// 36: secure AES interrupt
    SAES = 36,
    /// 37: ADC1 global interrupt
    ADC1 = 37,
    /// 38: DAC1 global interrupt
    DAC1 = 38,
    /// 39: FDCAN1 interrupt 0
    FDCAN1_IT0 = 39,
    /// 40: FDCAN1 interrupt 1
    FDCAN1_IT1 = 40,
    /// 41: TIM1 break/TIM1 transition error/TIM1 index error
    TIM1_BRK_TERR_IERR = 41,
    /// 42: TIM1 Update
    TIM1_UP = 42,
    /// 43: TIM1 trigger and commutation/TIM1 direction change interrupt/TIM1 index
    TIM1_TRG_COM_DIR_IDX = 43,
    /// 44: TIM1 capture compare interrupt
    TIM1_CC = 44,
    /// 45: TIM2 global interrupt
    TIM2 = 45,
    /// 46: TIM3 global interrupt
    TIM3 = 46,
    /// 47: TIM4 global interrupt
    TIM4 = 47,
    /// 48: TIM5 global interrupt
    TIM5 = 48,
    /// 49: TIM6 global interrupt
    TIM6 = 49,
    /// 50: TIM7 global interrupt
    TIM7 = 50,
    /// 51: I2C1 event interrupt
    I2C1_EV = 51,
    /// 52: I2C1 error interrupt
    I2C1_ER = 52,
    /// 53: I2C2 event interrupt
    I2C2_EV = 53,
    /// 54: I2C2 error interrupt
    I2C2_ER = 54,
    /// 55: SPI1 global interrupt
    SPI1 = 55,
    /// 56: SPI2 global interrupt
    SPI2 = 56,
    /// 57: SPI3 global interrupt
    SPI3 = 57,
    /// 58: USART1 global interrupt
    USART1 = 58,
    /// 59: USART2 global interrupt
    USART2 = 59,
    /// 60: USART3 global interrupt
    USART3 = 60,
    /// 61: UART4 global interrupt
    UART4 = 61,
    /// 62: UART5 global interrupt
    UART5 = 62,
    /// 63: LPUART1 global interrupt
    LPUART1 = 63,
    /// 64: LPTIM1 global interrupt
    LPTIM1 = 64,
    /// 65: TIM8 break interrupt/TIM8 transition error/TIM8 index error
    IM8_BRK_TERR_IERR = 65,
    /// 66: TIM8 update interrupt
    TIM8_UP = 66,
    /// 67: TIM8 trigger and commutation interrupt/TIM8 direction change interrupt/TIM8 index
    TIM8_TRG_COM_DIR_IDX = 67,
    /// 68: TIM8 capture compare interrupt
    TIM8_CC = 68,
    /// 69: ADC2 global interrupt
    ADC2 = 69,
    /// 70: LPTIM2 global interrupt
    LPTIM2 = 70,
    /// 71: TIM5 global interrupt
    TIM15 = 71,
    /// 72: TIM6 global interrupt
    TIM16 = 72,
    /// 73: TIM17 global interrupt
    TIM17 = 73,
    /// 74: USB OTG FS global interrupt
    USB_FS = 74,
    /// 75: Clock Recovery System global interrupt
    CRS = 75,
    /// 76: UCPD1 global interrupt
    UCPD1 = 76,
    /// 77: FMC global interrupt
    FMC = 77,
    /// 78: OCTOSPI1 global interrupt
    OCTOSPI1 = 78,
    /// 79: SDMMC1 global interrupt
    SDMMC1 = 79,
    /// 80: I2C3 event interrupt
    I2C3_EV = 80,
    /// 81: I2C3 error interrupt
    I2C3_ER = 81,
    /// 82: SPI4 global interrupt
    SPI4 = 82,
    /// 83: SPI5 global interrupt
    SPI5 = 83,
    /// 84: SPI6 global interrupt
    SPI6 = 84,
    /// 85: USART6 global interrupt
    USART6 = 85,
    /// 86: USART10 global interrupt
    USART10 = 86,
    /// 87: USART11 global interrupt
    USART11 = 87,
    /// 88: SAI1 global interrupt
    SAI1 = 88,
    /// 89: SAI2 global interrupt
    SAI2 = 89,
    /// 90: GPDMA2 channel 0 global interrupt
    GPDMA2_CH0 = 90,
    /// 91: GPDMA2 channel 1 global interrupt
    GPDMA2_CH1 = 91,
    /// 92: GPDMA2 channel 2 global interrupt
    GPDMA2_CH2 = 92,
    /// 93: GPDMA2 channel 3 global interrupt
    GPDMA2_CH3 = 93,
    /// 94: GPDMA2 channel 4 global interrupt
    GPDMA2_CH4 = 94,
    /// 95: GPDMA2 channel 5 global interrupt
    GPDMA2_CH5 = 95,
    /// 96: GPDMA2 channel 6 global interrupt
    GPDMA2_CH6 = 96,
    /// 97: GPDMA2 channel 7 global interrupt
    GPDMA2_CH7 = 97,
    /// 98: UART7 global interrupt
    UART7 = 98,
    /// 99: UART8 global interrupt
    UART8 = 99,
    /// 100: UART9 global interrupt
    UART9 = 100,
    /// 101: UART12 global interrupt
    UART12 = 101,
    /// 102: SDMMC2 global interrupt
    SDMMC2 = 102,
    /// 104: Instruction cache global interrupt
    ICACHE = 104,
    /// 105: Data cache global interrupt
    DCACHE = 105,
    /// 106: ETH interrupt
    ETH = 106,
    /// 107: ETHERNET wakeup interrupt through EXTI line
    ETH_WKUP = 107,
    /// 108: DCMI/PSSI global interrupt
    DCMI_PSSI = 108,
    /// 109: FDCAN2 interrupt 0
    FDCAN2_IT0 = 109,
    /// 110: FDCAN2 interrupt 1
    FDCAN2_IT1 = 110,
    /// 111: Cordic interrupt
    Cordic = 111,
    /// 112: FMAC interrupt
    FMAC = 112,
    /// 113: DTS interrupt or DTS AIT through EXTI line
    DTS_WKUP = 113,
    /// 114: RNG global interrupt
    RNG = 114,
    /// 115: OTFDEC1 secure global interrupt
    OTFDEC1 = 115,
    /// 116: AES global interrupt
    AES = 116,
    /// 117: HASH interrupt
    HASH = 117,
    /// 118: PKA global interrupt
    PKA = 118,
    /// 120: TIM12 global interrupt
    TIM12 = 120,
    /// 121: TIM13 global interrupt
    TIM13 = 121,
    /// 122: TIM14 global interrupt
    TIM14 = 122,
    /// 123: I3C1 event interrupt
    I3C1_EV = 123,
    /// 124: I3C1 error interrupt
    I3C1_ER = 124,
    /// 125: I2C4 event interrupt
    I2C4_EV = 125,
    /// 126: I2C4 error interrupt
    I2C4_ER = 126,
    /// 127: LPTIM3 global interrupt
    LPTIM3 = 127,
    /// 128: LPTIM4 global interrupt
    LPTIM4 = 128,
    /// 129: LPTIM5 global interrupt
    LPTIM5 = 129,
    /// 130: LPTIM6 global interrupt
    LPTIM6 = 130,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
