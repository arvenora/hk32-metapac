#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "2 - EXTI11"]
    EXTI11 = 2,
    #[doc = "3 - FLASH"]
    FLASH = 3,
    #[doc = "4 - RCC"]
    RCC = 4,
    #[doc = "5 - EXTI0"]
    EXTI0 = 5,
    #[doc = "6 - EXTI1"]
    EXTI1 = 6,
    #[doc = "7 - EXTI2"]
    EXTI2 = 7,
    #[doc = "8 - EXTI3"]
    EXTI3 = 8,
    #[doc = "9 - EXTI4"]
    EXTI4 = 9,
    #[doc = "10 - EXTI5"]
    EXTI5 = 10,
    #[doc = "11 - TIM1_BRK"]
    TIM1_BRK = 11,
    #[doc = "12 - ADC1"]
    ADC1 = 12,
    #[doc = "13 - TIM1_UP_TRG_COM"]
    TIM1_UP_TRG_COM = 13,
    #[doc = "14 - TIM1_CC"]
    TIM1_CC = 14,
    #[doc = "15 - TIM2"]
    TIM2 = 15,
    #[doc = "17 - TIM6"]
    TIM6 = 17,
    #[doc = "21 - EXTI6"]
    EXTI6 = 21,
    #[doc = "22 - EXTI7"]
    EXTI7 = 22,
    #[doc = "23 - I2C1"]
    I2C1 = 23,
    #[doc = "25 - SPI1"]
    SPI1 = 25,
    #[doc = "27 - USART1"]
    USART1 = 27,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn WWDG();
        fn EXTI11();
        fn FLASH();
        fn RCC();
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn EXTI5();
        fn TIM1_BRK();
        fn ADC1();
        fn TIM1_UP_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
        fn TIM6();
        fn EXTI6();
        fn EXTI7();
        fn I2C1();
        fn SPI1();
        fn USART1();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 28] = [
        Vector { _handler: WWDG },
        Vector { _reserved: 0 },
        Vector { _handler: EXTI11 },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
        Vector { _handler: EXTI5 },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: ADC1 },
        Vector {
            _handler: TIM1_UP_TRG_COM,
        },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _reserved: 0 },
        Vector { _handler: TIM6 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: EXTI6 },
        Vector { _handler: EXTI7 },
        Vector { _handler: I2C1 },
        Vector { _reserved: 0 },
        Vector { _handler: SPI1 },
        Vector { _reserved: 0 },
        Vector { _handler: USART1 },
    ];
}
pub const USART1: usart_v1::USART =
    unsafe { usart_v1::USART::from_ptr(0x4001_3800usize as _) };
pub const RCC: rcc_f030m::RCC =
    unsafe { rcc_f030m::RCC::from_ptr(0x4002_1000usize as _) };
pub const GPIOA: gpio_v1::GPIO =
    unsafe { gpio_v1::GPIO::from_ptr(0x4800_0000usize as _) };
pub const GPIOB: gpio_v1::GPIO =
    unsafe { gpio_v1::GPIO::from_ptr(0x4800_0400usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[allow(clippy::missing_safety_doc)]
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write(&self, f: impl FnOnce(&mut T)) {
            let mut val = Default::default();
            f(&mut val);
            self.write_value(val);
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify(&self, f: impl FnOnce(&mut T)) {
            let mut val = self.read();
            f(&mut val);
            self.write_value(val);
        }
    }
}
pub mod gpio_v1 {
    #[doc = "General-purpose I/Os."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPIO {
        ptr: *mut u8,
    }
    unsafe impl Send for GPIO {}
    unsafe impl Sync for GPIO {}
    impl GPIO {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "GPIO port mode register."]
        #[inline(always)]
        pub const fn MODER(self) -> crate::common::Reg<MODER, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "GPIO port output type register."]
        #[inline(always)]
        pub const fn OTYPER(self) -> crate::common::Reg<OTYPER, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "GPIO port output speed register."]
        #[inline(always)]
        pub const fn OSPEEDR(self) -> crate::common::Reg<OSPEEDR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "GPIO port pull-up/pull-down register."]
        #[inline(always)]
        pub const fn PUPDR(self) -> crate::common::Reg<PUPDR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "GPIO port input data register."]
        #[inline(always)]
        pub const fn IDR(self) -> crate::common::Reg<IDR, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "GPIO port output data register."]
        #[inline(always)]
        pub const fn ODR(self) -> crate::common::Reg<ODR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "GPIO port bit set/reset register."]
        #[inline(always)]
        pub const fn BSRR(self) -> crate::common::Reg<BSRR, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "GPIO port configuration lock register."]
        #[inline(always)]
        pub const fn LCKR(self) -> crate::common::Reg<LCKR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "GPIO alternate function low register."]
        #[inline(always)]
        pub const fn AFRL(self) -> crate::common::Reg<AFRL, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "GPIO alternate function high register."]
        #[inline(always)]
        pub const fn AFRH(self) -> crate::common::Reg<AFRH, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[doc = "Port bit reset register."]
        #[inline(always)]
        pub const fn BRR(self) -> crate::common::Reg<BRR, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
        #[doc = "IOSR."]
        #[inline(always)]
        pub const fn IOSR(self) -> crate::common::Reg<IOSR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
        }
    }
    #[doc = "GPIO alternate function high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AFRH(pub u32);
    impl AFRH {
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[inline(always)]
        pub const fn set_AFSEL8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL9(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[inline(always)]
        pub const fn set_AFSEL9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[inline(always)]
        pub const fn set_AFSEL10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL11(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[inline(always)]
        pub const fn set_AFSEL11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL12(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[inline(always)]
        pub const fn set_AFSEL12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL13(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[inline(always)]
        pub const fn set_AFSEL13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL14(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[inline(always)]
        pub const fn set_AFSEL14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL15(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 8..15)."]
        #[inline(always)]
        pub const fn set_AFSEL15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for AFRH {
        #[inline(always)]
        fn default() -> AFRH {
            AFRH(0)
        }
    }
    impl core::fmt::Debug for AFRH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AFRH")
                .field("AFSEL8", &self.AFSEL8())
                .field("AFSEL9", &self.AFSEL9())
                .field("AFSEL10", &self.AFSEL10())
                .field("AFSEL11", &self.AFSEL11())
                .field("AFSEL12", &self.AFSEL12())
                .field("AFSEL13", &self.AFSEL13())
                .field("AFSEL14", &self.AFSEL14())
                .field("AFSEL15", &self.AFSEL15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AFRH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AFRH {{ AFSEL8: {=u8:?}, AFSEL9: {=u8:?}, AFSEL10: {=u8:?}, AFSEL11: {=u8:?}, AFSEL12: {=u8:?}, AFSEL13: {=u8:?}, AFSEL14: {=u8:?}, AFSEL15: {=u8:?} }}",
                self.AFSEL8(),
                self.AFSEL9(),
                self.AFSEL10(),
                self.AFSEL11(),
                self.AFSEL12(),
                self.AFSEL13(),
                self.AFSEL14(),
                self.AFSEL15()
            )
        }
    }
    #[doc = "GPIO alternate function low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AFRL(pub u32);
    impl AFRL {
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[inline(always)]
        pub const fn set_AFSEL0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[inline(always)]
        pub const fn set_AFSEL1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[inline(always)]
        pub const fn set_AFSEL2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[inline(always)]
        pub const fn set_AFSEL3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[inline(always)]
        pub const fn set_AFSEL4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[inline(always)]
        pub const fn set_AFSEL5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[inline(always)]
        pub const fn set_AFSEL6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[must_use]
        #[inline(always)]
        pub const fn AFSEL7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for port x bit y (y = 0..7)."]
        #[inline(always)]
        pub const fn set_AFSEL7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for AFRL {
        #[inline(always)]
        fn default() -> AFRL {
            AFRL(0)
        }
    }
    impl core::fmt::Debug for AFRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AFRL")
                .field("AFSEL0", &self.AFSEL0())
                .field("AFSEL1", &self.AFSEL1())
                .field("AFSEL2", &self.AFSEL2())
                .field("AFSEL3", &self.AFSEL3())
                .field("AFSEL4", &self.AFSEL4())
                .field("AFSEL5", &self.AFSEL5())
                .field("AFSEL6", &self.AFSEL6())
                .field("AFSEL7", &self.AFSEL7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AFRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AFRL {{ AFSEL0: {=u8:?}, AFSEL1: {=u8:?}, AFSEL2: {=u8:?}, AFSEL3: {=u8:?}, AFSEL4: {=u8:?}, AFSEL5: {=u8:?}, AFSEL6: {=u8:?}, AFSEL7: {=u8:?} }}",
                self.AFSEL0(),
                self.AFSEL1(),
                self.AFSEL2(),
                self.AFSEL3(),
                self.AFSEL4(),
                self.AFSEL5(),
                self.AFSEL6(),
                self.AFSEL7()
            )
        }
    }
    #[doc = "Port bit reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BRR(pub u32);
    impl BRR {
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port x Reset bit y."]
        #[must_use]
        #[inline(always)]
        pub const fn BR15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port x Reset bit y."]
        #[inline(always)]
        pub const fn set_BR15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for BRR {
        #[inline(always)]
        fn default() -> BRR {
            BRR(0)
        }
    }
    impl core::fmt::Debug for BRR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BRR")
                .field("BR0", &self.BR0())
                .field("BR1", &self.BR1())
                .field("BR2", &self.BR2())
                .field("BR3", &self.BR3())
                .field("BR4", &self.BR4())
                .field("BR5", &self.BR5())
                .field("BR6", &self.BR6())
                .field("BR7", &self.BR7())
                .field("BR8", &self.BR8())
                .field("BR9", &self.BR9())
                .field("BR10", &self.BR10())
                .field("BR11", &self.BR11())
                .field("BR12", &self.BR12())
                .field("BR13", &self.BR13())
                .field("BR14", &self.BR14())
                .field("BR15", &self.BR15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BRR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BRR {{ BR0: {=bool:?}, BR1: {=bool:?}, BR2: {=bool:?}, BR3: {=bool:?}, BR4: {=bool:?}, BR5: {=bool:?}, BR6: {=bool:?}, BR7: {=bool:?}, BR8: {=bool:?}, BR9: {=bool:?}, BR10: {=bool:?}, BR11: {=bool:?}, BR12: {=bool:?}, BR13: {=bool:?}, BR14: {=bool:?}, BR15: {=bool:?} }}",
                self.BR0(),
                self.BR1(),
                self.BR2(),
                self.BR3(),
                self.BR4(),
                self.BR5(),
                self.BR6(),
                self.BR7(),
                self.BR8(),
                self.BR9(),
                self.BR10(),
                self.BR11(),
                self.BR12(),
                self.BR13(),
                self.BR14(),
                self.BR15()
            )
        }
    }
    #[doc = "GPIO port bit set/reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BSRR(pub u32);
    impl BSRR {
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BS15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BS15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_BR0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn BR15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Port x reset bit y (y = 0..15)."]
        #[inline(always)]
        pub const fn set_BR15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for BSRR {
        #[inline(always)]
        fn default() -> BSRR {
            BSRR(0)
        }
    }
    impl core::fmt::Debug for BSRR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BSRR")
                .field("BS0", &self.BS0())
                .field("BS1", &self.BS1())
                .field("BS2", &self.BS2())
                .field("BS3", &self.BS3())
                .field("BS4", &self.BS4())
                .field("BS5", &self.BS5())
                .field("BS6", &self.BS6())
                .field("BS7", &self.BS7())
                .field("BS8", &self.BS8())
                .field("BS9", &self.BS9())
                .field("BS10", &self.BS10())
                .field("BS11", &self.BS11())
                .field("BS12", &self.BS12())
                .field("BS13", &self.BS13())
                .field("BS14", &self.BS14())
                .field("BS15", &self.BS15())
                .field("BR0", &self.BR0())
                .field("BR1", &self.BR1())
                .field("BR2", &self.BR2())
                .field("BR3", &self.BR3())
                .field("BR4", &self.BR4())
                .field("BR5", &self.BR5())
                .field("BR6", &self.BR6())
                .field("BR7", &self.BR7())
                .field("BR8", &self.BR8())
                .field("BR9", &self.BR9())
                .field("BR10", &self.BR10())
                .field("BR11", &self.BR11())
                .field("BR12", &self.BR12())
                .field("BR13", &self.BR13())
                .field("BR14", &self.BR14())
                .field("BR15", &self.BR15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BSRR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BSRR {{ BS0: {=bool:?}, BS1: {=bool:?}, BS2: {=bool:?}, BS3: {=bool:?}, BS4: {=bool:?}, BS5: {=bool:?}, BS6: {=bool:?}, BS7: {=bool:?}, BS8: {=bool:?}, BS9: {=bool:?}, BS10: {=bool:?}, BS11: {=bool:?}, BS12: {=bool:?}, BS13: {=bool:?}, BS14: {=bool:?}, BS15: {=bool:?}, BR0: {=bool:?}, BR1: {=bool:?}, BR2: {=bool:?}, BR3: {=bool:?}, BR4: {=bool:?}, BR5: {=bool:?}, BR6: {=bool:?}, BR7: {=bool:?}, BR8: {=bool:?}, BR9: {=bool:?}, BR10: {=bool:?}, BR11: {=bool:?}, BR12: {=bool:?}, BR13: {=bool:?}, BR14: {=bool:?}, BR15: {=bool:?} }}",
                self.BS0(),
                self.BS1(),
                self.BS2(),
                self.BS3(),
                self.BS4(),
                self.BS5(),
                self.BS6(),
                self.BS7(),
                self.BS8(),
                self.BS9(),
                self.BS10(),
                self.BS11(),
                self.BS12(),
                self.BS13(),
                self.BS14(),
                self.BS15(),
                self.BR0(),
                self.BR1(),
                self.BR2(),
                self.BR3(),
                self.BR4(),
                self.BR5(),
                self.BR6(),
                self.BR7(),
                self.BR8(),
                self.BR9(),
                self.BR10(),
                self.BR11(),
                self.BR12(),
                self.BR13(),
                self.BR14(),
                self.BR15()
            )
        }
    }
    #[doc = "GPIO port input data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IDR(pub u32);
    impl IDR {
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port input data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn IDR15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port input data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_IDR15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IDR {
        #[inline(always)]
        fn default() -> IDR {
            IDR(0)
        }
    }
    impl core::fmt::Debug for IDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IDR")
                .field("IDR0", &self.IDR0())
                .field("IDR1", &self.IDR1())
                .field("IDR2", &self.IDR2())
                .field("IDR3", &self.IDR3())
                .field("IDR4", &self.IDR4())
                .field("IDR5", &self.IDR5())
                .field("IDR6", &self.IDR6())
                .field("IDR7", &self.IDR7())
                .field("IDR8", &self.IDR8())
                .field("IDR9", &self.IDR9())
                .field("IDR10", &self.IDR10())
                .field("IDR11", &self.IDR11())
                .field("IDR12", &self.IDR12())
                .field("IDR13", &self.IDR13())
                .field("IDR14", &self.IDR14())
                .field("IDR15", &self.IDR15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IDR {{ IDR0: {=bool:?}, IDR1: {=bool:?}, IDR2: {=bool:?}, IDR3: {=bool:?}, IDR4: {=bool:?}, IDR5: {=bool:?}, IDR6: {=bool:?}, IDR7: {=bool:?}, IDR8: {=bool:?}, IDR9: {=bool:?}, IDR10: {=bool:?}, IDR11: {=bool:?}, IDR12: {=bool:?}, IDR13: {=bool:?}, IDR14: {=bool:?}, IDR15: {=bool:?} }}",
                self.IDR0(),
                self.IDR1(),
                self.IDR2(),
                self.IDR3(),
                self.IDR4(),
                self.IDR5(),
                self.IDR6(),
                self.IDR7(),
                self.IDR8(),
                self.IDR9(),
                self.IDR10(),
                self.IDR11(),
                self.IDR12(),
                self.IDR13(),
                self.IDR14(),
                self.IDR15()
            )
        }
    }
    #[doc = "IOSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IOSR(pub u32);
    impl IOSR {
        #[doc = "IOSENX."]
        #[must_use]
        #[inline(always)]
        pub const fn IOSENX(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "IOSENX."]
        #[inline(always)]
        pub const fn set_IOSENX(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for IOSR {
        #[inline(always)]
        fn default() -> IOSR {
            IOSR(0)
        }
    }
    impl core::fmt::Debug for IOSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IOSR")
                .field("IOSENX", &self.IOSENX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IOSR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IOSR {{ IOSENX: {=u16:?} }}", self.IOSENX())
        }
    }
    #[doc = "GPIO port configuration lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LCKR(pub u32);
    impl LCKR {
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCK15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCK15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn LCKK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Port x lock bit y (y= 0..15)."]
        #[inline(always)]
        pub const fn set_LCKK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for LCKR {
        #[inline(always)]
        fn default() -> LCKR {
            LCKR(0)
        }
    }
    impl core::fmt::Debug for LCKR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LCKR")
                .field("LCK0", &self.LCK0())
                .field("LCK1", &self.LCK1())
                .field("LCK2", &self.LCK2())
                .field("LCK3", &self.LCK3())
                .field("LCK4", &self.LCK4())
                .field("LCK5", &self.LCK5())
                .field("LCK6", &self.LCK6())
                .field("LCK7", &self.LCK7())
                .field("LCK8", &self.LCK8())
                .field("LCK9", &self.LCK9())
                .field("LCK10", &self.LCK10())
                .field("LCK11", &self.LCK11())
                .field("LCK12", &self.LCK12())
                .field("LCK13", &self.LCK13())
                .field("LCK14", &self.LCK14())
                .field("LCK15", &self.LCK15())
                .field("LCKK", &self.LCKK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LCKR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LCKR {{ LCK0: {=bool:?}, LCK1: {=bool:?}, LCK2: {=bool:?}, LCK3: {=bool:?}, LCK4: {=bool:?}, LCK5: {=bool:?}, LCK6: {=bool:?}, LCK7: {=bool:?}, LCK8: {=bool:?}, LCK9: {=bool:?}, LCK10: {=bool:?}, LCK11: {=bool:?}, LCK12: {=bool:?}, LCK13: {=bool:?}, LCK14: {=bool:?}, LCK15: {=bool:?}, LCKK: {=bool:?} }}",
                self.LCK0(),
                self.LCK1(),
                self.LCK2(),
                self.LCK3(),
                self.LCK4(),
                self.LCK5(),
                self.LCK6(),
                self.LCK7(),
                self.LCK8(),
                self.LCK9(),
                self.LCK10(),
                self.LCK11(),
                self.LCK12(),
                self.LCK13(),
                self.LCK14(),
                self.LCK15(),
                self.LCKK()
            )
        }
    }
    #[doc = "GPIO port mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MODER(pub u32);
    impl MODER {
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER8(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER9(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER10(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER11(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER13(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER14(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn MODER15(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_MODER15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for MODER {
        #[inline(always)]
        fn default() -> MODER {
            MODER(0)
        }
    }
    impl core::fmt::Debug for MODER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MODER")
                .field("MODER0", &self.MODER0())
                .field("MODER1", &self.MODER1())
                .field("MODER2", &self.MODER2())
                .field("MODER3", &self.MODER3())
                .field("MODER4", &self.MODER4())
                .field("MODER5", &self.MODER5())
                .field("MODER6", &self.MODER6())
                .field("MODER7", &self.MODER7())
                .field("MODER8", &self.MODER8())
                .field("MODER9", &self.MODER9())
                .field("MODER10", &self.MODER10())
                .field("MODER11", &self.MODER11())
                .field("MODER12", &self.MODER12())
                .field("MODER13", &self.MODER13())
                .field("MODER14", &self.MODER14())
                .field("MODER15", &self.MODER15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MODER {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MODER {{ MODER0: {=u8:?}, MODER1: {=u8:?}, MODER2: {=u8:?}, MODER3: {=u8:?}, MODER4: {=u8:?}, MODER5: {=u8:?}, MODER6: {=u8:?}, MODER7: {=u8:?}, MODER8: {=u8:?}, MODER9: {=u8:?}, MODER10: {=u8:?}, MODER11: {=u8:?}, MODER12: {=u8:?}, MODER13: {=u8:?}, MODER14: {=u8:?}, MODER15: {=u8:?} }}",
                self.MODER0(),
                self.MODER1(),
                self.MODER2(),
                self.MODER3(),
                self.MODER4(),
                self.MODER5(),
                self.MODER6(),
                self.MODER7(),
                self.MODER8(),
                self.MODER9(),
                self.MODER10(),
                self.MODER11(),
                self.MODER12(),
                self.MODER13(),
                self.MODER14(),
                self.MODER15()
            )
        }
    }
    #[doc = "GPIO port output data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ODR(pub u32);
    impl ODR {
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port output data (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn ODR15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port output data (y = 0..15)."]
        #[inline(always)]
        pub const fn set_ODR15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for ODR {
        #[inline(always)]
        fn default() -> ODR {
            ODR(0)
        }
    }
    impl core::fmt::Debug for ODR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ODR")
                .field("ODR0", &self.ODR0())
                .field("ODR1", &self.ODR1())
                .field("ODR2", &self.ODR2())
                .field("ODR3", &self.ODR3())
                .field("ODR4", &self.ODR4())
                .field("ODR5", &self.ODR5())
                .field("ODR6", &self.ODR6())
                .field("ODR7", &self.ODR7())
                .field("ODR8", &self.ODR8())
                .field("ODR9", &self.ODR9())
                .field("ODR10", &self.ODR10())
                .field("ODR11", &self.ODR11())
                .field("ODR12", &self.ODR12())
                .field("ODR13", &self.ODR13())
                .field("ODR14", &self.ODR14())
                .field("ODR15", &self.ODR15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ODR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ODR {{ ODR0: {=bool:?}, ODR1: {=bool:?}, ODR2: {=bool:?}, ODR3: {=bool:?}, ODR4: {=bool:?}, ODR5: {=bool:?}, ODR6: {=bool:?}, ODR7: {=bool:?}, ODR8: {=bool:?}, ODR9: {=bool:?}, ODR10: {=bool:?}, ODR11: {=bool:?}, ODR12: {=bool:?}, ODR13: {=bool:?}, ODR14: {=bool:?}, ODR15: {=bool:?} }}",
                self.ODR0(),
                self.ODR1(),
                self.ODR2(),
                self.ODR3(),
                self.ODR4(),
                self.ODR5(),
                self.ODR6(),
                self.ODR7(),
                self.ODR8(),
                self.ODR9(),
                self.ODR10(),
                self.ODR11(),
                self.ODR12(),
                self.ODR13(),
                self.ODR14(),
                self.ODR15()
            )
        }
    }
    #[doc = "GPIO port output speed register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSPEEDR(pub u32);
    impl OSPEEDR {
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR8(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR9(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR10(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR11(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR13(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR14(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OSPEEDR15(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OSPEEDR15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for OSPEEDR {
        #[inline(always)]
        fn default() -> OSPEEDR {
            OSPEEDR(0)
        }
    }
    impl core::fmt::Debug for OSPEEDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OSPEEDR")
                .field("OSPEEDR0", &self.OSPEEDR0())
                .field("OSPEEDR1", &self.OSPEEDR1())
                .field("OSPEEDR2", &self.OSPEEDR2())
                .field("OSPEEDR3", &self.OSPEEDR3())
                .field("OSPEEDR4", &self.OSPEEDR4())
                .field("OSPEEDR5", &self.OSPEEDR5())
                .field("OSPEEDR6", &self.OSPEEDR6())
                .field("OSPEEDR7", &self.OSPEEDR7())
                .field("OSPEEDR8", &self.OSPEEDR8())
                .field("OSPEEDR9", &self.OSPEEDR9())
                .field("OSPEEDR10", &self.OSPEEDR10())
                .field("OSPEEDR11", &self.OSPEEDR11())
                .field("OSPEEDR12", &self.OSPEEDR12())
                .field("OSPEEDR13", &self.OSPEEDR13())
                .field("OSPEEDR14", &self.OSPEEDR14())
                .field("OSPEEDR15", &self.OSPEEDR15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OSPEEDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OSPEEDR {{ OSPEEDR0: {=u8:?}, OSPEEDR1: {=u8:?}, OSPEEDR2: {=u8:?}, OSPEEDR3: {=u8:?}, OSPEEDR4: {=u8:?}, OSPEEDR5: {=u8:?}, OSPEEDR6: {=u8:?}, OSPEEDR7: {=u8:?}, OSPEEDR8: {=u8:?}, OSPEEDR9: {=u8:?}, OSPEEDR10: {=u8:?}, OSPEEDR11: {=u8:?}, OSPEEDR12: {=u8:?}, OSPEEDR13: {=u8:?}, OSPEEDR14: {=u8:?}, OSPEEDR15: {=u8:?} }}",
                self.OSPEEDR0(),
                self.OSPEEDR1(),
                self.OSPEEDR2(),
                self.OSPEEDR3(),
                self.OSPEEDR4(),
                self.OSPEEDR5(),
                self.OSPEEDR6(),
                self.OSPEEDR7(),
                self.OSPEEDR8(),
                self.OSPEEDR9(),
                self.OSPEEDR10(),
                self.OSPEEDR11(),
                self.OSPEEDR12(),
                self.OSPEEDR13(),
                self.OSPEEDR14(),
                self.OSPEEDR15()
            )
        }
    }
    #[doc = "GPIO port output type register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OTYPER(pub u32);
    impl OTYPER {
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn OT15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_OT15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for OTYPER {
        #[inline(always)]
        fn default() -> OTYPER {
            OTYPER(0)
        }
    }
    impl core::fmt::Debug for OTYPER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OTYPER")
                .field("OT0", &self.OT0())
                .field("OT1", &self.OT1())
                .field("OT2", &self.OT2())
                .field("OT3", &self.OT3())
                .field("OT4", &self.OT4())
                .field("OT5", &self.OT5())
                .field("OT6", &self.OT6())
                .field("OT7", &self.OT7())
                .field("OT8", &self.OT8())
                .field("OT9", &self.OT9())
                .field("OT10", &self.OT10())
                .field("OT11", &self.OT11())
                .field("OT12", &self.OT12())
                .field("OT13", &self.OT13())
                .field("OT14", &self.OT14())
                .field("OT15", &self.OT15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OTYPER {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OTYPER {{ OT0: {=bool:?}, OT1: {=bool:?}, OT2: {=bool:?}, OT3: {=bool:?}, OT4: {=bool:?}, OT5: {=bool:?}, OT6: {=bool:?}, OT7: {=bool:?}, OT8: {=bool:?}, OT9: {=bool:?}, OT10: {=bool:?}, OT11: {=bool:?}, OT12: {=bool:?}, OT13: {=bool:?}, OT14: {=bool:?}, OT15: {=bool:?} }}",
                self.OT0(),
                self.OT1(),
                self.OT2(),
                self.OT3(),
                self.OT4(),
                self.OT5(),
                self.OT6(),
                self.OT7(),
                self.OT8(),
                self.OT9(),
                self.OT10(),
                self.OT11(),
                self.OT12(),
                self.OT13(),
                self.OT14(),
                self.OT15()
            )
        }
    }
    #[doc = "GPIO port pull-up/pull-down register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PUPDR(pub u32);
    impl PUPDR {
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR8(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR9(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR10(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR11(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR13(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR14(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[must_use]
        #[inline(always)]
        pub const fn PUPDR15(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Port x configuration bits (y = 0..15)."]
        #[inline(always)]
        pub const fn set_PUPDR15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for PUPDR {
        #[inline(always)]
        fn default() -> PUPDR {
            PUPDR(0)
        }
    }
    impl core::fmt::Debug for PUPDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PUPDR")
                .field("PUPDR0", &self.PUPDR0())
                .field("PUPDR1", &self.PUPDR1())
                .field("PUPDR2", &self.PUPDR2())
                .field("PUPDR3", &self.PUPDR3())
                .field("PUPDR4", &self.PUPDR4())
                .field("PUPDR5", &self.PUPDR5())
                .field("PUPDR6", &self.PUPDR6())
                .field("PUPDR7", &self.PUPDR7())
                .field("PUPDR8", &self.PUPDR8())
                .field("PUPDR9", &self.PUPDR9())
                .field("PUPDR10", &self.PUPDR10())
                .field("PUPDR11", &self.PUPDR11())
                .field("PUPDR12", &self.PUPDR12())
                .field("PUPDR13", &self.PUPDR13())
                .field("PUPDR14", &self.PUPDR14())
                .field("PUPDR15", &self.PUPDR15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PUPDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PUPDR {{ PUPDR0: {=u8:?}, PUPDR1: {=u8:?}, PUPDR2: {=u8:?}, PUPDR3: {=u8:?}, PUPDR4: {=u8:?}, PUPDR5: {=u8:?}, PUPDR6: {=u8:?}, PUPDR7: {=u8:?}, PUPDR8: {=u8:?}, PUPDR9: {=u8:?}, PUPDR10: {=u8:?}, PUPDR11: {=u8:?}, PUPDR12: {=u8:?}, PUPDR13: {=u8:?}, PUPDR14: {=u8:?}, PUPDR15: {=u8:?} }}",
                self.PUPDR0(),
                self.PUPDR1(),
                self.PUPDR2(),
                self.PUPDR3(),
                self.PUPDR4(),
                self.PUPDR5(),
                self.PUPDR6(),
                self.PUPDR7(),
                self.PUPDR8(),
                self.PUPDR9(),
                self.PUPDR10(),
                self.PUPDR11(),
                self.PUPDR12(),
                self.PUPDR13(),
                self.PUPDR14(),
                self.PUPDR15()
            )
        }
    }
}
pub mod rcc_f030m {
    #[doc = "Reset and clock control."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RCC {
        ptr: *mut u8,
    }
    unsafe impl Send for RCC {}
    unsafe impl Sync for RCC {}
    impl RCC {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Clock control register."]
        #[inline(always)]
        pub const fn CR(self) -> crate::common::Reg<CR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Clock configuration register (RCC_CFGR)."]
        #[inline(always)]
        pub const fn CFGR(self) -> crate::common::Reg<CFGR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Clock interrupt register (RCC_CIR)."]
        #[inline(always)]
        pub const fn CIR(self) -> crate::common::Reg<CIR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "APB2 peripheral reset register (RCC_APB2RSTR)."]
        #[inline(always)]
        pub const fn APB2RSTR(self) -> crate::common::Reg<APB2RSTR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "APB1 peripheral reset register (RCC_APB1RSTR)."]
        #[inline(always)]
        pub const fn APB1RSTR(self) -> crate::common::Reg<APB1RSTR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)."]
        #[inline(always)]
        pub const fn AHBENR(self) -> crate::common::Reg<AHBENR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)."]
        #[inline(always)]
        pub const fn APB2ENR(self) -> crate::common::Reg<APB2ENR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)."]
        #[inline(always)]
        pub const fn APB1ENR(self) -> crate::common::Reg<APB1ENR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "Control/status register (RCC_CSR)."]
        #[inline(always)]
        pub const fn CSR(self) -> crate::common::Reg<CSR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[doc = "AHB peripheral reset register."]
        #[inline(always)]
        pub const fn AHBRSTR(self) -> crate::common::Reg<AHBRSTR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
        #[doc = "Clock configuration register 3."]
        #[inline(always)]
        pub const fn CFGR3(self) -> crate::common::Reg<CFGR3, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
        }
        #[doc = "RCC control register."]
        #[inline(always)]
        pub const fn RCC_CSS(self) -> crate::common::Reg<RCC_CSS, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
        }
        #[doc = "Clock configuration register 4."]
        #[inline(always)]
        pub const fn RCC_CFGR4(self) -> crate::common::Reg<RCC_CFGR4, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
        }
    }
    #[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBENR(pub u32);
    impl AHBENR {
        #[doc = "SRAM interface clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn SRAMEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM interface clock enable."]
        #[inline(always)]
        pub const fn set_SRAMEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FLITF clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn FLITFEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FLITF clock enable."]
        #[inline(always)]
        pub const fn set_FLITFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CRC clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn CRCEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable."]
        #[inline(always)]
        pub const fn set_CRCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I/O port A clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn IOPAEN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port A clock enable."]
        #[inline(always)]
        pub const fn set_IOPAEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "I/O port B clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn IOPBEN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port B clock enable."]
        #[inline(always)]
        pub const fn set_IOPBEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "I/O port C clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn IOPCEN(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port C clock enable."]
        #[inline(always)]
        pub const fn set_IOPCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "I/O port D clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn IOPDEN(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port D clock enable."]
        #[inline(always)]
        pub const fn set_IOPDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for AHBENR {
        #[inline(always)]
        fn default() -> AHBENR {
            AHBENR(0)
        }
    }
    impl core::fmt::Debug for AHBENR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBENR")
                .field("SRAMEN", &self.SRAMEN())
                .field("FLITFEN", &self.FLITFEN())
                .field("CRCEN", &self.CRCEN())
                .field("IOPAEN", &self.IOPAEN())
                .field("IOPBEN", &self.IOPBEN())
                .field("IOPCEN", &self.IOPCEN())
                .field("IOPDEN", &self.IOPDEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBENR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHBENR {{ SRAMEN: {=bool:?}, FLITFEN: {=bool:?}, CRCEN: {=bool:?}, IOPAEN: {=bool:?}, IOPBEN: {=bool:?}, IOPCEN: {=bool:?}, IOPDEN: {=bool:?} }}",
                self.SRAMEN(),
                self.FLITFEN(),
                self.CRCEN(),
                self.IOPAEN(),
                self.IOPBEN(),
                self.IOPCEN(),
                self.IOPDEN()
            )
        }
    }
    #[doc = "AHB peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBRSTR(pub u32);
    impl AHBRSTR {
        #[doc = "CRC reset."]
        #[must_use]
        #[inline(always)]
        pub const fn CRCRST(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reset."]
        #[inline(always)]
        pub const fn set_CRCRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I/O port A reset."]
        #[must_use]
        #[inline(always)]
        pub const fn IOPARST(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port A reset."]
        #[inline(always)]
        pub const fn set_IOPARST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "I/O port B reset."]
        #[must_use]
        #[inline(always)]
        pub const fn IOPBRST(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port B reset."]
        #[inline(always)]
        pub const fn set_IOPBRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "I/O port C reset."]
        #[must_use]
        #[inline(always)]
        pub const fn IOPCRST(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port C reset."]
        #[inline(always)]
        pub const fn set_IOPCRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "I/O port D reset."]
        #[must_use]
        #[inline(always)]
        pub const fn IOPDRST(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port D reset."]
        #[inline(always)]
        pub const fn set_IOPDRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for AHBRSTR {
        #[inline(always)]
        fn default() -> AHBRSTR {
            AHBRSTR(0)
        }
    }
    impl core::fmt::Debug for AHBRSTR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBRSTR")
                .field("CRCRST", &self.CRCRST())
                .field("IOPARST", &self.IOPARST())
                .field("IOPBRST", &self.IOPBRST())
                .field("IOPCRST", &self.IOPCRST())
                .field("IOPDRST", &self.IOPDRST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBRSTR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHBRSTR {{ CRCRST: {=bool:?}, IOPARST: {=bool:?}, IOPBRST: {=bool:?}, IOPCRST: {=bool:?}, IOPDRST: {=bool:?} }}",
                self.CRCRST(),
                self.IOPARST(),
                self.IOPBRST(),
                self.IOPCRST(),
                self.IOPDRST()
            )
        }
    }
    #[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APB1ENR(pub u32);
    impl APB1ENR {
        #[doc = "Timer 2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn TIM2EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 clock enable."]
        #[inline(always)]
        pub const fn set_TIM2EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 6 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn TIM6EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 6 clock enable."]
        #[inline(always)]
        pub const fn set_TIM6EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Window watchdog clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn WWDGEN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clock enable."]
        #[inline(always)]
        pub const fn set_WWDGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "stop wakeup enable."]
        #[must_use]
        #[inline(always)]
        pub const fn AWUEN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "stop wakeup enable."]
        #[inline(always)]
        pub const fn set_AWUEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "I2C 1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn I2C1EN(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 1 clock enable."]
        #[inline(always)]
        pub const fn set_I2C1EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Power interface clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn PWREN(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clock enable."]
        #[inline(always)]
        pub const fn set_PWREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "beeper clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn BEEPEREN(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "beeper clock enable."]
        #[inline(always)]
        pub const fn set_BEEPEREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "IOMUX interface clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn IOMUXEN(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "IOMUX interface clock enable."]
        #[inline(always)]
        pub const fn set_IOMUXEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for APB1ENR {
        #[inline(always)]
        fn default() -> APB1ENR {
            APB1ENR(0)
        }
    }
    impl core::fmt::Debug for APB1ENR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APB1ENR")
                .field("TIM2EN", &self.TIM2EN())
                .field("TIM6EN", &self.TIM6EN())
                .field("WWDGEN", &self.WWDGEN())
                .field("AWUEN", &self.AWUEN())
                .field("I2C1EN", &self.I2C1EN())
                .field("PWREN", &self.PWREN())
                .field("BEEPEREN", &self.BEEPEREN())
                .field("IOMUXEN", &self.IOMUXEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APB1ENR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "APB1ENR {{ TIM2EN: {=bool:?}, TIM6EN: {=bool:?}, WWDGEN: {=bool:?}, AWUEN: {=bool:?}, I2C1EN: {=bool:?}, PWREN: {=bool:?}, BEEPEREN: {=bool:?}, IOMUXEN: {=bool:?} }}",
                self.TIM2EN(),
                self.TIM6EN(),
                self.WWDGEN(),
                self.AWUEN(),
                self.I2C1EN(),
                self.PWREN(),
                self.BEEPEREN(),
                self.IOMUXEN()
            )
        }
    }
    #[doc = "APB1 peripheral reset register (RCC_APB1RSTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APB1RSTR(pub u32);
    impl APB1RSTR {
        #[doc = "Timer 2 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn TIM2RST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 reset."]
        #[inline(always)]
        pub const fn set_TIM2RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 6 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn TIM6RST(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 6 reset."]
        #[inline(always)]
        pub const fn set_TIM6RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Window watchdog reset."]
        #[must_use]
        #[inline(always)]
        pub const fn WWDGRST(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset."]
        #[inline(always)]
        pub const fn set_WWDGRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "STOP wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn AWURST(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "STOP wakeup."]
        #[inline(always)]
        pub const fn set_AWURST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "I2C1 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn I2C1RST(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset."]
        #[inline(always)]
        pub const fn set_I2C1RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Power interface reset."]
        #[must_use]
        #[inline(always)]
        pub const fn PWRRST(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface reset."]
        #[inline(always)]
        pub const fn set_PWRRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "beeper reset."]
        #[must_use]
        #[inline(always)]
        pub const fn BEEPERRST(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "beeper reset."]
        #[inline(always)]
        pub const fn set_BEEPERRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "IO MUX reset."]
        #[must_use]
        #[inline(always)]
        pub const fn IOMUXRST(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "IO MUX reset."]
        #[inline(always)]
        pub const fn set_IOMUXRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for APB1RSTR {
        #[inline(always)]
        fn default() -> APB1RSTR {
            APB1RSTR(0)
        }
    }
    impl core::fmt::Debug for APB1RSTR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APB1RSTR")
                .field("TIM2RST", &self.TIM2RST())
                .field("TIM6RST", &self.TIM6RST())
                .field("WWDGRST", &self.WWDGRST())
                .field("AWURST", &self.AWURST())
                .field("I2C1RST", &self.I2C1RST())
                .field("PWRRST", &self.PWRRST())
                .field("BEEPERRST", &self.BEEPERRST())
                .field("IOMUXRST", &self.IOMUXRST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APB1RSTR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "APB1RSTR {{ TIM2RST: {=bool:?}, TIM6RST: {=bool:?}, WWDGRST: {=bool:?}, AWURST: {=bool:?}, I2C1RST: {=bool:?}, PWRRST: {=bool:?}, BEEPERRST: {=bool:?}, IOMUXRST: {=bool:?} }}",
                self.TIM2RST(),
                self.TIM6RST(),
                self.WWDGRST(),
                self.AWURST(),
                self.I2C1RST(),
                self.PWRRST(),
                self.BEEPERRST(),
                self.IOMUXRST()
            )
        }
    }
    #[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APB2ENR(pub u32);
    impl APB2ENR {
        #[doc = "SYSCFG clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn SYSCFGEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG clock enable."]
        #[inline(always)]
        pub const fn set_SYSCFGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC 1 interface clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ADCEN(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 interface clock enable."]
        #[inline(always)]
        pub const fn set_ADCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "TIM1 Timer clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn TIM1EN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 Timer clock enable."]
        #[inline(always)]
        pub const fn set_TIM1EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn SPI1EN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 clock enable."]
        #[inline(always)]
        pub const fn set_SPI1EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn USART1EN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable."]
        #[inline(always)]
        pub const fn set_USART1EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "MCU debug module clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn DBGMCUEN(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "MCU debug module clock enable."]
        #[inline(always)]
        pub const fn set_DBGMCUEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for APB2ENR {
        #[inline(always)]
        fn default() -> APB2ENR {
            APB2ENR(0)
        }
    }
    impl core::fmt::Debug for APB2ENR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APB2ENR")
                .field("SYSCFGEN", &self.SYSCFGEN())
                .field("ADCEN", &self.ADCEN())
                .field("TIM1EN", &self.TIM1EN())
                .field("SPI1EN", &self.SPI1EN())
                .field("USART1EN", &self.USART1EN())
                .field("DBGMCUEN", &self.DBGMCUEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APB2ENR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "APB2ENR {{ SYSCFGEN: {=bool:?}, ADCEN: {=bool:?}, TIM1EN: {=bool:?}, SPI1EN: {=bool:?}, USART1EN: {=bool:?}, DBGMCUEN: {=bool:?} }}",
                self.SYSCFGEN(),
                self.ADCEN(),
                self.TIM1EN(),
                self.SPI1EN(),
                self.USART1EN(),
                self.DBGMCUEN()
            )
        }
    }
    #[doc = "APB2 peripheral reset register (RCC_APB2RSTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APB2RSTR(pub u32);
    impl APB2RSTR {
        #[doc = "SYSCFG and COMP reset."]
        #[must_use]
        #[inline(always)]
        pub const fn SYSCFGRST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG and COMP reset."]
        #[inline(always)]
        pub const fn set_SYSCFGRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC interface reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ADCRST(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC interface reset."]
        #[inline(always)]
        pub const fn set_ADCRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "TIM1 timer reset."]
        #[must_use]
        #[inline(always)]
        pub const fn TIM1RST(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 timer reset."]
        #[inline(always)]
        pub const fn set_TIM1RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 1 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn SPI1RST(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 reset."]
        #[inline(always)]
        pub const fn set_SPI1RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn USART1RST(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset."]
        #[inline(always)]
        pub const fn set_USART1RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Debug MCU reset."]
        #[must_use]
        #[inline(always)]
        pub const fn DBGMCURST(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Debug MCU reset."]
        #[inline(always)]
        pub const fn set_DBGMCURST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for APB2RSTR {
        #[inline(always)]
        fn default() -> APB2RSTR {
            APB2RSTR(0)
        }
    }
    impl core::fmt::Debug for APB2RSTR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APB2RSTR")
                .field("SYSCFGRST", &self.SYSCFGRST())
                .field("ADCRST", &self.ADCRST())
                .field("TIM1RST", &self.TIM1RST())
                .field("SPI1RST", &self.SPI1RST())
                .field("USART1RST", &self.USART1RST())
                .field("DBGMCURST", &self.DBGMCURST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APB2RSTR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "APB2RSTR {{ SYSCFGRST: {=bool:?}, ADCRST: {=bool:?}, TIM1RST: {=bool:?}, SPI1RST: {=bool:?}, USART1RST: {=bool:?}, DBGMCURST: {=bool:?} }}",
                self.SYSCFGRST(),
                self.ADCRST(),
                self.TIM1RST(),
                self.SPI1RST(),
                self.USART1RST(),
                self.DBGMCURST()
            )
        }
    }
    #[doc = "Clock configuration register (RCC_CFGR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CFGR(pub u32);
    impl CFGR {
        #[doc = "System clock Switch."]
        #[must_use]
        #[inline(always)]
        pub const fn SW(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "System clock Switch."]
        #[inline(always)]
        pub const fn set_SW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "System Clock Switch Status."]
        #[must_use]
        #[inline(always)]
        pub const fn SWS(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "System Clock Switch Status."]
        #[inline(always)]
        pub const fn set_SWS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "AHB prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn HPRE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "AHB prescaler."]
        #[inline(always)]
        pub const fn set_HPRE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "APB Low speed prescaler (APB1)."]
        #[must_use]
        #[inline(always)]
        pub const fn PPRE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "APB Low speed prescaler (APB1)."]
        #[inline(always)]
        pub const fn set_PPRE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Microcontroller clock output."]
        #[must_use]
        #[inline(always)]
        pub const fn MCO(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Microcontroller clock output."]
        #[inline(always)]
        pub const fn set_MCO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Microcontroller Clock Output Prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn MCOPRE(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Microcontroller Clock Output Prescaler."]
        #[inline(always)]
        pub const fn set_MCOPRE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for CFGR {
        #[inline(always)]
        fn default() -> CFGR {
            CFGR(0)
        }
    }
    impl core::fmt::Debug for CFGR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CFGR")
                .field("SW", &self.SW())
                .field("SWS", &self.SWS())
                .field("HPRE", &self.HPRE())
                .field("PPRE", &self.PPRE())
                .field("MCO", &self.MCO())
                .field("MCOPRE", &self.MCOPRE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CFGR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CFGR {{ SW: {=u8:?}, SWS: {=u8:?}, HPRE: {=u8:?}, PPRE: {=u8:?}, MCO: {=u8:?}, MCOPRE: {=u8:?} }}",
                self.SW(),
                self.SWS(),
                self.HPRE(),
                self.PPRE(),
                self.MCO(),
                self.MCOPRE()
            )
        }
    }
    #[doc = "Clock configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CFGR3(pub u32);
    impl CFGR3 {
        #[doc = "USART1 clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn USART1SW(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "USART1 clock source selection."]
        #[inline(always)]
        pub const fn set_USART1SW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "I2C1 clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn I2C1SW(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock source selection."]
        #[inline(always)]
        pub const fn set_I2C1SW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for CFGR3 {
        #[inline(always)]
        fn default() -> CFGR3 {
            CFGR3(0)
        }
    }
    impl core::fmt::Debug for CFGR3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CFGR3")
                .field("USART1SW", &self.USART1SW())
                .field("I2C1SW", &self.I2C1SW())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CFGR3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CFGR3 {{ USART1SW: {=u8:?}, I2C1SW: {=bool:?} }}",
                self.USART1SW(),
                self.I2C1SW()
            )
        }
    }
    #[doc = "Clock interrupt register (RCC_CIR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CIR(pub u32);
    impl CIR {
        #[doc = "LSI Ready Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn LSIRDYF(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt flag."]
        #[inline(always)]
        pub const fn set_LSIRDYF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSI Ready Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn HSIRDYF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt flag."]
        #[inline(always)]
        pub const fn set_HSIRDYF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EXT Ready Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn EXTRDYF(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EXT Ready Interrupt flag."]
        #[inline(always)]
        pub const fn set_EXTRDYF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clock Security System Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn CSSF(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Security System Interrupt flag."]
        #[inline(always)]
        pub const fn set_CSSF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LSI Ready Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn LSIRDYIE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn set_LSIRDYIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSI Ready Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn HSIRDYIE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn set_HSIRDYIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "EXT Ready Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn EXTRDYIE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "EXT Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn set_EXTRDYIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LSI Ready Interrupt Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn LSIRDYC(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn set_LSIRDYC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSI Ready Interrupt Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn HSIRDYC(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn set_HSIRDYC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "EXT Ready Interrupt Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn EXTRDYC(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "EXT Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn set_EXTRDYC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Clock security system interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn CSSC(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt clear."]
        #[inline(always)]
        pub const fn set_CSSC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for CIR {
        #[inline(always)]
        fn default() -> CIR {
            CIR(0)
        }
    }
    impl core::fmt::Debug for CIR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CIR")
                .field("LSIRDYF", &self.LSIRDYF())
                .field("HSIRDYF", &self.HSIRDYF())
                .field("EXTRDYF", &self.EXTRDYF())
                .field("CSSF", &self.CSSF())
                .field("LSIRDYIE", &self.LSIRDYIE())
                .field("HSIRDYIE", &self.HSIRDYIE())
                .field("EXTRDYIE", &self.EXTRDYIE())
                .field("LSIRDYC", &self.LSIRDYC())
                .field("HSIRDYC", &self.HSIRDYC())
                .field("EXTRDYC", &self.EXTRDYC())
                .field("CSSC", &self.CSSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CIR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CIR {{ LSIRDYF: {=bool:?}, HSIRDYF: {=bool:?}, EXTRDYF: {=bool:?}, CSSF: {=bool:?}, LSIRDYIE: {=bool:?}, HSIRDYIE: {=bool:?}, EXTRDYIE: {=bool:?}, LSIRDYC: {=bool:?}, HSIRDYC: {=bool:?}, EXTRDYC: {=bool:?}, CSSC: {=bool:?} }}",
                self.LSIRDYF(),
                self.HSIRDYF(),
                self.EXTRDYF(),
                self.CSSF(),
                self.LSIRDYIE(),
                self.HSIRDYIE(),
                self.EXTRDYIE(),
                self.LSIRDYC(),
                self.HSIRDYC(),
                self.EXTRDYC(),
                self.CSSC()
            )
        }
    }
    #[doc = "Clock control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CR(pub u32);
    impl CR {
        #[doc = "Internal High Speed clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn HSION(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal High Speed clock enable."]
        #[inline(always)]
        pub const fn set_HSION(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal High Speed clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn HSIRDY(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal High Speed clock ready flag."]
        #[inline(always)]
        pub const fn set_HSIRDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Internal High Speed clock trimming."]
        #[must_use]
        #[inline(always)]
        pub const fn HSITRIM(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Internal High Speed clock trimming."]
        #[inline(always)]
        pub const fn set_HSITRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "Internal High Speed clock Calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn HSICAL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Internal High Speed clock Calibration."]
        #[inline(always)]
        pub const fn set_HSICAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "External High Speed clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn EXTCLK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock enable."]
        #[inline(always)]
        pub const fn set_EXTCLK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "External High Speed clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn EXTCLKRDY(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock ready flag."]
        #[inline(always)]
        pub const fn set_EXTCLKRDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Clock Security System enable."]
        #[must_use]
        #[inline(always)]
        pub const fn CSSON(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Security System enable."]
        #[inline(always)]
        pub const fn set_CSSON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for CR {
        #[inline(always)]
        fn default() -> CR {
            CR(0)
        }
    }
    impl core::fmt::Debug for CR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CR")
                .field("HSION", &self.HSION())
                .field("HSIRDY", &self.HSIRDY())
                .field("HSITRIM", &self.HSITRIM())
                .field("HSICAL", &self.HSICAL())
                .field("EXTCLK", &self.EXTCLK())
                .field("EXTCLKRDY", &self.EXTCLKRDY())
                .field("CSSON", &self.CSSON())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CR {{ HSION: {=bool:?}, HSIRDY: {=bool:?}, HSITRIM: {=u8:?}, HSICAL: {=u8:?}, EXTCLK: {=bool:?}, EXTCLKRDY: {=bool:?}, CSSON: {=bool:?} }}",
                self.HSION(),
                self.HSIRDY(),
                self.HSITRIM(),
                self.HSICAL(),
                self.EXTCLK(),
                self.EXTCLKRDY(),
                self.CSSON()
            )
        }
    }
    #[doc = "Control/status register (RCC_CSR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CSR(pub u32);
    impl CSR {
        #[doc = "Internal low speed oscillator enable."]
        #[must_use]
        #[inline(always)]
        pub const fn LSION(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low speed oscillator enable."]
        #[inline(always)]
        pub const fn set_LSION(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal low speed oscillator ready."]
        #[must_use]
        #[inline(always)]
        pub const fn LSIRDY(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low speed oscillator ready."]
        #[inline(always)]
        pub const fn set_LSIRDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Remove reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn RMVF(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag."]
        #[inline(always)]
        pub const fn set_RMVF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PIN reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn PINRSTF(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PIN reset flag."]
        #[inline(always)]
        pub const fn set_PINRSTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "POR/PDR reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn PORRSTF(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag."]
        #[inline(always)]
        pub const fn set_PORRSTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn SFTRSTF(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag."]
        #[inline(always)]
        pub const fn set_SFTRSTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent watchdog reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn IWDGRSTF(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag."]
        #[inline(always)]
        pub const fn set_IWDGRSTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn WWDGRSTF(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset flag."]
        #[inline(always)]
        pub const fn set_WWDGRSTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn LPWRRSTF(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag."]
        #[inline(always)]
        pub const fn set_LPWRRSTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CSR {
        #[inline(always)]
        fn default() -> CSR {
            CSR(0)
        }
    }
    impl core::fmt::Debug for CSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CSR")
                .field("LSION", &self.LSION())
                .field("LSIRDY", &self.LSIRDY())
                .field("RMVF", &self.RMVF())
                .field("PINRSTF", &self.PINRSTF())
                .field("PORRSTF", &self.PORRSTF())
                .field("SFTRSTF", &self.SFTRSTF())
                .field("IWDGRSTF", &self.IWDGRSTF())
                .field("WWDGRSTF", &self.WWDGRSTF())
                .field("LPWRRSTF", &self.LPWRRSTF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CSR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CSR {{ LSION: {=bool:?}, LSIRDY: {=bool:?}, RMVF: {=bool:?}, PINRSTF: {=bool:?}, PORRSTF: {=bool:?}, SFTRSTF: {=bool:?}, IWDGRSTF: {=bool:?}, WWDGRSTF: {=bool:?}, LPWRRSTF: {=bool:?} }}",
                self.LSION(),
                self.LSIRDY(),
                self.RMVF(),
                self.PINRSTF(),
                self.PORRSTF(),
                self.SFTRSTF(),
                self.IWDGRSTF(),
                self.WWDGRSTF(),
                self.LPWRRSTF()
            )
        }
    }
    #[doc = "Clock configuration register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RCC_CFGR4(pub u32);
    impl RCC_CFGR4 {
        #[doc = "USART Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn USARTHSIPRE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "USART Clock."]
        #[inline(always)]
        pub const fn set_USARTHSIPRE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "FLITFCLK_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn FLITFCLK_SEL(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "FLITFCLK_SEL."]
        #[inline(always)]
        pub const fn set_FLITFCLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "FLITFCLK_PRE."]
        #[must_use]
        #[inline(always)]
        pub const fn FLITFCLK_PRE(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "FLITFCLK_PRE."]
        #[inline(always)]
        pub const fn set_FLITFCLK_PRE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "I2C1CLK_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn I2C1CLK_SEL(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1CLK_SEL."]
        #[inline(always)]
        pub const fn set_I2C1CLK_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "I2CHSIPRE."]
        #[must_use]
        #[inline(always)]
        pub const fn I2CHSIPRE(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "I2CHSIPRE."]
        #[inline(always)]
        pub const fn set_I2CHSIPRE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "External clock input pin selection."]
        #[must_use]
        #[inline(always)]
        pub const fn EXTCLK_SEL(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "External clock input pin selection."]
        #[inline(always)]
        pub const fn set_EXTCLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "ADCHSIPRE."]
        #[must_use]
        #[inline(always)]
        pub const fn ADCHSIPRE(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "ADCHSIPRE."]
        #[inline(always)]
        pub const fn set_ADCHSIPRE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
    }
    impl Default for RCC_CFGR4 {
        #[inline(always)]
        fn default() -> RCC_CFGR4 {
            RCC_CFGR4(0)
        }
    }
    impl core::fmt::Debug for RCC_CFGR4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RCC_CFGR4")
                .field("USARTHSIPRE", &self.USARTHSIPRE())
                .field("FLITFCLK_SEL", &self.FLITFCLK_SEL())
                .field("FLITFCLK_PRE", &self.FLITFCLK_PRE())
                .field("I2C1CLK_SEL", &self.I2C1CLK_SEL())
                .field("I2CHSIPRE", &self.I2CHSIPRE())
                .field("EXTCLK_SEL", &self.EXTCLK_SEL())
                .field("ADCHSIPRE", &self.ADCHSIPRE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RCC_CFGR4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RCC_CFGR4 {{ USARTHSIPRE: {=u8:?}, FLITFCLK_SEL: {=u8:?}, FLITFCLK_PRE: {=u8:?}, I2C1CLK_SEL: {=bool:?}, I2CHSIPRE: {=u8:?}, EXTCLK_SEL: {=u8:?}, ADCHSIPRE: {=u8:?} }}",
                self.USARTHSIPRE(),
                self.FLITFCLK_SEL(),
                self.FLITFCLK_PRE(),
                self.I2C1CLK_SEL(),
                self.I2CHSIPRE(),
                self.EXTCLK_SEL(),
                self.ADCHSIPRE()
            )
        }
    }
    #[doc = "RCC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RCC_CSS(pub u32);
    impl RCC_CSS {
        #[doc = "Control the threshold of the CSS counter."]
        #[must_use]
        #[inline(always)]
        pub const fn CSS_THRESHOLD(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[doc = "Control the threshold of the CSS counter."]
        #[inline(always)]
        pub const fn set_CSS_THRESHOLD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for RCC_CSS {
        #[inline(always)]
        fn default() -> RCC_CSS {
            RCC_CSS(0)
        }
    }
    impl core::fmt::Debug for RCC_CSS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RCC_CSS")
                .field("CSS_THRESHOLD", &self.CSS_THRESHOLD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RCC_CSS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RCC_CSS {{ CSS_THRESHOLD: {=u8:?} }}",
                self.CSS_THRESHOLD()
            )
        }
    }
}
pub mod usart_v1 {
    #[doc = "Universal synchronous asynchronous receiver transmitter."]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USART {
        ptr: *mut u8,
    }
    unsafe impl Send for USART {}
    unsafe impl Sync for USART {}
    impl USART {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Control register 1."]
        #[inline(always)]
        pub const fn CR1(self) -> crate::common::Reg<CR1, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
        }
        #[doc = "Control register 2."]
        #[inline(always)]
        pub const fn CR2(self) -> crate::common::Reg<CR2, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
        }
        #[doc = "Control register 3."]
        #[inline(always)]
        pub const fn CR3(self) -> crate::common::Reg<CR3, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
        }
        #[doc = "Baud rate register."]
        #[inline(always)]
        pub const fn BRR(self) -> crate::common::Reg<BRR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
        }
        #[doc = "Guard time and prescaler register."]
        #[inline(always)]
        pub const fn GTPR(self) -> crate::common::Reg<GTPR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
        }
        #[doc = "Receiver timeout register."]
        #[inline(always)]
        pub const fn RTOR(self) -> crate::common::Reg<RTOR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
        }
        #[doc = "Request register."]
        #[inline(always)]
        pub const fn RQR(self) -> crate::common::Reg<RQR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
        }
        #[doc = "Interrupt & status register."]
        #[inline(always)]
        pub const fn ISR(self) -> crate::common::Reg<ISR, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
        }
        #[doc = "Interrupt flag clear register."]
        #[inline(always)]
        pub const fn ICR(self) -> crate::common::Reg<ICR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
        }
        #[doc = "Receive data register."]
        #[inline(always)]
        pub const fn RDR(self) -> crate::common::Reg<RDR, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
        }
        #[doc = "Transmit data register."]
        #[inline(always)]
        pub const fn TDR(self) -> crate::common::Reg<TDR, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
        }
    }
    #[doc = "Baud rate register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BRR(pub u32);
    impl BRR {
        #[doc = "Baud rate register: include mantissa of USARTDIV(BRR\\[15:4\\]) and fraction of USARTDIV(BRR\\[3:0\\])."]
        #[must_use]
        #[inline(always)]
        pub const fn BRR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Baud rate register: include mantissa of USARTDIV(BRR\\[15:4\\]) and fraction of USARTDIV(BRR\\[3:0\\])."]
        #[inline(always)]
        pub const fn set_BRR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for BRR {
        #[inline(always)]
        fn default() -> BRR {
            BRR(0)
        }
    }
    impl core::fmt::Debug for BRR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BRR").field("BRR", &self.BRR()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BRR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BRR {{ BRR: {=u16:?} }}", self.BRR())
        }
    }
    #[doc = "Control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CR1(pub u32);
    impl CR1 {
        #[doc = "USART enable."]
        #[must_use]
        #[inline(always)]
        pub const fn UE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USART enable."]
        #[inline(always)]
        pub const fn set_UE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USART enable in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn UESM(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USART enable in Stop mode."]
        #[inline(always)]
        pub const fn set_UESM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receiver enable."]
        #[must_use]
        #[inline(always)]
        pub const fn RE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver enable."]
        #[inline(always)]
        pub const fn set_RE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn TE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter enable."]
        #[inline(always)]
        pub const fn set_TE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IDLE interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn IDLEIE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IDLE interrupt enable."]
        #[inline(always)]
        pub const fn set_IDLEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RXNE interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn RXNEIE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RXNE interrupt enable."]
        #[inline(always)]
        pub const fn set_RXNEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmission complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn TCIE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission complete interrupt enable."]
        #[inline(always)]
        pub const fn set_TCIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn TXEIE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable."]
        #[inline(always)]
        pub const fn set_TXEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PE interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn PEIE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PE interrupt enable."]
        #[inline(always)]
        pub const fn set_PEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Parity selection."]
        #[must_use]
        #[inline(always)]
        pub const fn PS(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Parity selection."]
        #[inline(always)]
        pub const fn set_PS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Parity control enable."]
        #[must_use]
        #[inline(always)]
        pub const fn PCE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Parity control enable."]
        #[inline(always)]
        pub const fn set_PCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Receiver wakeup method."]
        #[must_use]
        #[inline(always)]
        pub const fn WAKE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver wakeup method."]
        #[inline(always)]
        pub const fn set_WAKE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Word length."]
        #[must_use]
        #[inline(always)]
        pub const fn M(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Word length."]
        #[inline(always)]
        pub const fn set_M(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Mute mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn MME(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Mute mode enable."]
        #[inline(always)]
        pub const fn set_MME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Character match interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn CMIE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Character match interrupt enable."]
        #[inline(always)]
        pub const fn set_CMIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Oversampling mode."]
        #[must_use]
        #[inline(always)]
        pub const fn OVER8(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Oversampling mode."]
        #[inline(always)]
        pub const fn set_OVER8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Driver Enable deassertion time."]
        #[must_use]
        #[inline(always)]
        pub const fn DEDT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Driver Enable deassertion time."]
        #[inline(always)]
        pub const fn set_DEDT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Driver Enable assertion time."]
        #[must_use]
        #[inline(always)]
        pub const fn DEAT(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x1f;
            val as u8
        }
        #[doc = "Driver Enable assertion time."]
        #[inline(always)]
        pub const fn set_DEAT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
        }
        #[doc = "Receiver timeout interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn RTOIE(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver timeout interrupt enable."]
        #[inline(always)]
        pub const fn set_RTOIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "End of Block interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn EOBIE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "End of Block interrupt enable."]
        #[inline(always)]
        pub const fn set_EOBIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Word length."]
        #[must_use]
        #[inline(always)]
        pub const fn M1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Word length."]
        #[inline(always)]
        pub const fn set_M1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for CR1 {
        #[inline(always)]
        fn default() -> CR1 {
            CR1(0)
        }
    }
    impl core::fmt::Debug for CR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CR1")
                .field("UE", &self.UE())
                .field("UESM", &self.UESM())
                .field("RE", &self.RE())
                .field("TE", &self.TE())
                .field("IDLEIE", &self.IDLEIE())
                .field("RXNEIE", &self.RXNEIE())
                .field("TCIE", &self.TCIE())
                .field("TXEIE", &self.TXEIE())
                .field("PEIE", &self.PEIE())
                .field("PS", &self.PS())
                .field("PCE", &self.PCE())
                .field("WAKE", &self.WAKE())
                .field("M", &self.M())
                .field("MME", &self.MME())
                .field("CMIE", &self.CMIE())
                .field("OVER8", &self.OVER8())
                .field("DEDT", &self.DEDT())
                .field("DEAT", &self.DEAT())
                .field("RTOIE", &self.RTOIE())
                .field("EOBIE", &self.EOBIE())
                .field("M1", &self.M1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CR1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CR1 {{ UE: {=bool:?}, UESM: {=bool:?}, RE: {=bool:?}, TE: {=bool:?}, IDLEIE: {=bool:?}, RXNEIE: {=bool:?}, TCIE: {=bool:?}, TXEIE: {=bool:?}, PEIE: {=bool:?}, PS: {=bool:?}, PCE: {=bool:?}, WAKE: {=bool:?}, M: {=bool:?}, MME: {=bool:?}, CMIE: {=bool:?}, OVER8: {=bool:?}, DEDT: {=u8:?}, DEAT: {=u8:?}, RTOIE: {=bool:?}, EOBIE: {=bool:?}, M1: {=bool:?} }}",
                self.UE(),
                self.UESM(),
                self.RE(),
                self.TE(),
                self.IDLEIE(),
                self.RXNEIE(),
                self.TCIE(),
                self.TXEIE(),
                self.PEIE(),
                self.PS(),
                self.PCE(),
                self.WAKE(),
                self.M(),
                self.MME(),
                self.CMIE(),
                self.OVER8(),
                self.DEDT(),
                self.DEAT(),
                self.RTOIE(),
                self.EOBIE(),
                self.M1()
            )
        }
    }
    #[doc = "Control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CR2(pub u32);
    impl CR2 {
        #[doc = "7-bit Address Detection/4-bit Address Detection."]
        #[must_use]
        #[inline(always)]
        pub const fn ADDM7(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "7-bit Address Detection/4-bit Address Detection."]
        #[inline(always)]
        pub const fn set_ADDM7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LIN break detection length."]
        #[must_use]
        #[inline(always)]
        pub const fn LBDL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection length."]
        #[inline(always)]
        pub const fn set_LBDL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LIN break detection interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn LBDIE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection interrupt enable."]
        #[inline(always)]
        pub const fn set_LBDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Last bit clock pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn LBCL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Last bit clock pulse."]
        #[inline(always)]
        pub const fn set_LBCL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clock phase."]
        #[must_use]
        #[inline(always)]
        pub const fn CPHA(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clock phase."]
        #[inline(always)]
        pub const fn set_CPHA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn CPOL(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock polarity."]
        #[inline(always)]
        pub const fn set_CPOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn CLKEN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clock enable."]
        #[inline(always)]
        pub const fn set_CLKEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "STOP bits."]
        #[must_use]
        #[inline(always)]
        pub const fn STOP(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "STOP bits."]
        #[inline(always)]
        pub const fn set_STOP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "LIN mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn LINEN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "LIN mode enable."]
        #[inline(always)]
        pub const fn set_LINEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Swap TX/RX pins."]
        #[must_use]
        #[inline(always)]
        pub const fn SWAP(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Swap TX/RX pins."]
        #[inline(always)]
        pub const fn set_SWAP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RX pin active level inversion."]
        #[must_use]
        #[inline(always)]
        pub const fn RXINV(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RX pin active level inversion."]
        #[inline(always)]
        pub const fn set_RXINV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TX pin active level inversion."]
        #[must_use]
        #[inline(always)]
        pub const fn TXINV(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TX pin active level inversion."]
        #[inline(always)]
        pub const fn set_TXINV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Binary data inversion."]
        #[must_use]
        #[inline(always)]
        pub const fn DATAINV(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Binary data inversion."]
        #[inline(always)]
        pub const fn set_DATAINV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Most significant bit first."]
        #[must_use]
        #[inline(always)]
        pub const fn MSBFIRST(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Most significant bit first."]
        #[inline(always)]
        pub const fn set_MSBFIRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Auto baud rate enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ABREN(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Auto baud rate enable."]
        #[inline(always)]
        pub const fn set_ABREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Auto baud rate mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ABRMOD(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "Auto baud rate mode."]
        #[inline(always)]
        pub const fn set_ABRMOD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "Receiver timeout enable."]
        #[must_use]
        #[inline(always)]
        pub const fn RTOEN(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver timeout enable."]
        #[inline(always)]
        pub const fn set_RTOEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Address of the USART node."]
        #[must_use]
        #[inline(always)]
        pub const fn ADD(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Address of the USART node."]
        #[inline(always)]
        pub const fn set_ADD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Address of the USART node."]
        #[must_use]
        #[inline(always)]
        pub const fn ADD4(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Address of the USART node."]
        #[inline(always)]
        pub const fn set_ADD4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for CR2 {
        #[inline(always)]
        fn default() -> CR2 {
            CR2(0)
        }
    }
    impl core::fmt::Debug for CR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CR2")
                .field("ADDM7", &self.ADDM7())
                .field("LBDL", &self.LBDL())
                .field("LBDIE", &self.LBDIE())
                .field("LBCL", &self.LBCL())
                .field("CPHA", &self.CPHA())
                .field("CPOL", &self.CPOL())
                .field("CLKEN", &self.CLKEN())
                .field("STOP", &self.STOP())
                .field("LINEN", &self.LINEN())
                .field("SWAP", &self.SWAP())
                .field("RXINV", &self.RXINV())
                .field("TXINV", &self.TXINV())
                .field("DATAINV", &self.DATAINV())
                .field("MSBFIRST", &self.MSBFIRST())
                .field("ABREN", &self.ABREN())
                .field("ABRMOD", &self.ABRMOD())
                .field("RTOEN", &self.RTOEN())
                .field("ADD", &self.ADD())
                .field("ADD4", &self.ADD4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CR2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CR2 {{ ADDM7: {=bool:?}, LBDL: {=bool:?}, LBDIE: {=bool:?}, LBCL: {=bool:?}, CPHA: {=bool:?}, CPOL: {=bool:?}, CLKEN: {=bool:?}, STOP: {=u8:?}, LINEN: {=bool:?}, SWAP: {=bool:?}, RXINV: {=bool:?}, TXINV: {=bool:?}, DATAINV: {=bool:?}, MSBFIRST: {=bool:?}, ABREN: {=bool:?}, ABRMOD: {=u8:?}, RTOEN: {=bool:?}, ADD: {=u8:?}, ADD4: {=u8:?} }}",
                self.ADDM7(),
                self.LBDL(),
                self.LBDIE(),
                self.LBCL(),
                self.CPHA(),
                self.CPOL(),
                self.CLKEN(),
                self.STOP(),
                self.LINEN(),
                self.SWAP(),
                self.RXINV(),
                self.TXINV(),
                self.DATAINV(),
                self.MSBFIRST(),
                self.ABREN(),
                self.ABRMOD(),
                self.RTOEN(),
                self.ADD(),
                self.ADD4()
            )
        }
    }
    #[doc = "Control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CR3(pub u32);
    impl CR3 {
        #[doc = "Error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn EIE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub const fn set_EIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IrDA mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn IREN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IrDA mode enable."]
        #[inline(always)]
        pub const fn set_IREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IrDA low-power."]
        #[must_use]
        #[inline(always)]
        pub const fn IRLP(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IrDA low-power."]
        #[inline(always)]
        pub const fn set_IRLP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Half-duplex selection."]
        #[must_use]
        #[inline(always)]
        pub const fn HDSEL(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Half-duplex selection."]
        #[inline(always)]
        pub const fn set_HDSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Smartcard NACK enable."]
        #[must_use]
        #[inline(always)]
        pub const fn NACK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Smartcard NACK enable."]
        #[inline(always)]
        pub const fn set_NACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Smartcard mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn SCEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Smartcard mode enable."]
        #[inline(always)]
        pub const fn set_SCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RTS enable."]
        #[must_use]
        #[inline(always)]
        pub const fn RTSE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RTS enable."]
        #[inline(always)]
        pub const fn set_RTSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CTS enable."]
        #[must_use]
        #[inline(always)]
        pub const fn CTSE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CTS enable."]
        #[inline(always)]
        pub const fn set_CTSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CTS interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn CTSIE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CTS interrupt enable."]
        #[inline(always)]
        pub const fn set_CTSIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "One sample bit method enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ONEBIT(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "One sample bit method enable."]
        #[inline(always)]
        pub const fn set_ONEBIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Overrun Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn OVRDIS(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun Disable."]
        #[inline(always)]
        pub const fn set_OVRDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Driver enable mode."]
        #[must_use]
        #[inline(always)]
        pub const fn DEM(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Driver enable mode."]
        #[inline(always)]
        pub const fn set_DEM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Driver enable polarity selection."]
        #[must_use]
        #[inline(always)]
        pub const fn DEP(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Driver enable polarity selection."]
        #[inline(always)]
        pub const fn set_DEP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Smartcard auto-retry count."]
        #[must_use]
        #[inline(always)]
        pub const fn SCARCNT(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Smartcard auto-retry count."]
        #[inline(always)]
        pub const fn set_SCARCNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Wakeup from Stop mode interrupt flag selection."]
        #[must_use]
        #[inline(always)]
        pub const fn WUS(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Wakeup from Stop mode interrupt flag selection."]
        #[inline(always)]
        pub const fn set_WUS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Wakeup from Stop mode interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn WUFIE(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup from Stop mode interrupt enable."]
        #[inline(always)]
        pub const fn set_WUFIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for CR3 {
        #[inline(always)]
        fn default() -> CR3 {
            CR3(0)
        }
    }
    impl core::fmt::Debug for CR3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CR3")
                .field("EIE", &self.EIE())
                .field("IREN", &self.IREN())
                .field("IRLP", &self.IRLP())
                .field("HDSEL", &self.HDSEL())
                .field("NACK", &self.NACK())
                .field("SCEN", &self.SCEN())
                .field("RTSE", &self.RTSE())
                .field("CTSE", &self.CTSE())
                .field("CTSIE", &self.CTSIE())
                .field("ONEBIT", &self.ONEBIT())
                .field("OVRDIS", &self.OVRDIS())
                .field("DEM", &self.DEM())
                .field("DEP", &self.DEP())
                .field("SCARCNT", &self.SCARCNT())
                .field("WUS", &self.WUS())
                .field("WUFIE", &self.WUFIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CR3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CR3 {{ EIE: {=bool:?}, IREN: {=bool:?}, IRLP: {=bool:?}, HDSEL: {=bool:?}, NACK: {=bool:?}, SCEN: {=bool:?}, RTSE: {=bool:?}, CTSE: {=bool:?}, CTSIE: {=bool:?}, ONEBIT: {=bool:?}, OVRDIS: {=bool:?}, DEM: {=bool:?}, DEP: {=bool:?}, SCARCNT: {=u8:?}, WUS: {=u8:?}, WUFIE: {=bool:?} }}",
                self.EIE(),
                self.IREN(),
                self.IRLP(),
                self.HDSEL(),
                self.NACK(),
                self.SCEN(),
                self.RTSE(),
                self.CTSE(),
                self.CTSIE(),
                self.ONEBIT(),
                self.OVRDIS(),
                self.DEM(),
                self.DEP(),
                self.SCARCNT(),
                self.WUS(),
                self.WUFIE()
            )
        }
    }
    #[doc = "Guard time and prescaler register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GTPR(pub u32);
    impl GTPR {
        #[doc = "Prescaler value."]
        #[must_use]
        #[inline(always)]
        pub const fn PSC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Prescaler value."]
        #[inline(always)]
        pub const fn set_PSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Guard time value."]
        #[must_use]
        #[inline(always)]
        pub const fn GT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Guard time value."]
        #[inline(always)]
        pub const fn set_GT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for GTPR {
        #[inline(always)]
        fn default() -> GTPR {
            GTPR(0)
        }
    }
    impl core::fmt::Debug for GTPR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GTPR")
                .field("PSC", &self.PSC())
                .field("GT", &self.GT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GTPR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GTPR {{ PSC: {=u8:?}, GT: {=u8:?} }}",
                self.PSC(),
                self.GT()
            )
        }
    }
    #[doc = "Interrupt flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ICR(pub u32);
    impl ICR {
        #[doc = "Parity error clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn PECF(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error clear flag."]
        #[inline(always)]
        pub const fn set_PECF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Framing error clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn FECF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Framing error clear flag."]
        #[inline(always)]
        pub const fn set_FECF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Noise detected clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn NCF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Noise detected clear flag."]
        #[inline(always)]
        pub const fn set_NCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun error clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ORECF(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error clear flag."]
        #[inline(always)]
        pub const fn set_ORECF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Idle line detected clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn IDLECF(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Idle line detected clear flag."]
        #[inline(always)]
        pub const fn set_IDLECF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmission complete clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn TCCF(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission complete clear flag."]
        #[inline(always)]
        pub const fn set_TCCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LIN break detection clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn LBDCF(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection clear flag."]
        #[inline(always)]
        pub const fn set_LBDCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CTS clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn CTSCF(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CTS clear flag."]
        #[inline(always)]
        pub const fn set_CTSCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Receiver timeout clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn RTOCF(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver timeout clear flag."]
        #[inline(always)]
        pub const fn set_RTOCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "End of timeout clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn EOBCF(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "End of timeout clear flag."]
        #[inline(always)]
        pub const fn set_EOBCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Character match clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn CMCF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Character match clear flag."]
        #[inline(always)]
        pub const fn set_CMCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Wakeup from Stop mode clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn WUCF(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup from Stop mode clear flag."]
        #[inline(always)]
        pub const fn set_WUCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for ICR {
        #[inline(always)]
        fn default() -> ICR {
            ICR(0)
        }
    }
    impl core::fmt::Debug for ICR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ICR")
                .field("PECF", &self.PECF())
                .field("FECF", &self.FECF())
                .field("NCF", &self.NCF())
                .field("ORECF", &self.ORECF())
                .field("IDLECF", &self.IDLECF())
                .field("TCCF", &self.TCCF())
                .field("LBDCF", &self.LBDCF())
                .field("CTSCF", &self.CTSCF())
                .field("RTOCF", &self.RTOCF())
                .field("EOBCF", &self.EOBCF())
                .field("CMCF", &self.CMCF())
                .field("WUCF", &self.WUCF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ICR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ICR {{ PECF: {=bool:?}, FECF: {=bool:?}, NCF: {=bool:?}, ORECF: {=bool:?}, IDLECF: {=bool:?}, TCCF: {=bool:?}, LBDCF: {=bool:?}, CTSCF: {=bool:?}, RTOCF: {=bool:?}, EOBCF: {=bool:?}, CMCF: {=bool:?}, WUCF: {=bool:?} }}",
                self.PECF(),
                self.FECF(),
                self.NCF(),
                self.ORECF(),
                self.IDLECF(),
                self.TCCF(),
                self.LBDCF(),
                self.CTSCF(),
                self.RTOCF(),
                self.EOBCF(),
                self.CMCF(),
                self.WUCF()
            )
        }
    }
    #[doc = "Interrupt & status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ISR(pub u32);
    impl ISR {
        #[doc = "Parity error."]
        #[must_use]
        #[inline(always)]
        pub const fn PE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error."]
        #[inline(always)]
        pub const fn set_PE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Framing error."]
        #[must_use]
        #[inline(always)]
        pub const fn FE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Framing error."]
        #[inline(always)]
        pub const fn set_FE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Noise detected flag."]
        #[must_use]
        #[inline(always)]
        pub const fn NF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Noise detected flag."]
        #[inline(always)]
        pub const fn set_NF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun error."]
        #[must_use]
        #[inline(always)]
        pub const fn ORE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error."]
        #[inline(always)]
        pub const fn set_ORE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Idle line detected."]
        #[must_use]
        #[inline(always)]
        pub const fn IDLE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Idle line detected."]
        #[inline(always)]
        pub const fn set_IDLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Read data register not empty."]
        #[must_use]
        #[inline(always)]
        pub const fn RXNE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Read data register not empty."]
        #[inline(always)]
        pub const fn set_RXNE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmission complete."]
        #[must_use]
        #[inline(always)]
        pub const fn TC(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission complete."]
        #[inline(always)]
        pub const fn set_TC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transmit data register empty."]
        #[must_use]
        #[inline(always)]
        pub const fn TXE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit data register empty."]
        #[inline(always)]
        pub const fn set_TXE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LIN break detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn LBDF(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection flag."]
        #[inline(always)]
        pub const fn set_LBDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CTS interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn CTSIF(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CTS interrupt flag."]
        #[inline(always)]
        pub const fn set_CTSIF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CTS flag."]
        #[must_use]
        #[inline(always)]
        pub const fn CTS(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CTS flag."]
        #[inline(always)]
        pub const fn set_CTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Receiver timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn RTOF(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver timeout."]
        #[inline(always)]
        pub const fn set_RTOF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "End of block flag."]
        #[must_use]
        #[inline(always)]
        pub const fn EOBF(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "End of block flag."]
        #[inline(always)]
        pub const fn set_EOBF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Auto baud rate error."]
        #[must_use]
        #[inline(always)]
        pub const fn ABRE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Auto baud rate error."]
        #[inline(always)]
        pub const fn set_ABRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Auto baud rate flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ABRF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Auto baud rate flag."]
        #[inline(always)]
        pub const fn set_ABRF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Busy flag."]
        #[must_use]
        #[inline(always)]
        pub const fn BUSY(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag."]
        #[inline(always)]
        pub const fn set_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "character match flag."]
        #[must_use]
        #[inline(always)]
        pub const fn CMF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "character match flag."]
        #[inline(always)]
        pub const fn set_CMF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Send break flag."]
        #[must_use]
        #[inline(always)]
        pub const fn SBKF(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Send break flag."]
        #[inline(always)]
        pub const fn set_SBKF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Receiver wakeup from Mute mode."]
        #[must_use]
        #[inline(always)]
        pub const fn RWU(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver wakeup from Mute mode."]
        #[inline(always)]
        pub const fn set_RWU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Wakeup from Stop mode flag."]
        #[must_use]
        #[inline(always)]
        pub const fn WUF(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup from Stop mode flag."]
        #[inline(always)]
        pub const fn set_WUF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Transmit enable acknowledge flag."]
        #[must_use]
        #[inline(always)]
        pub const fn TEACK(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit enable acknowledge flag."]
        #[inline(always)]
        pub const fn set_TEACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Receive enable acknowledge flag."]
        #[must_use]
        #[inline(always)]
        pub const fn REACK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Receive enable acknowledge flag."]
        #[inline(always)]
        pub const fn set_REACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for ISR {
        #[inline(always)]
        fn default() -> ISR {
            ISR(0)
        }
    }
    impl core::fmt::Debug for ISR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ISR")
                .field("PE", &self.PE())
                .field("FE", &self.FE())
                .field("NF", &self.NF())
                .field("ORE", &self.ORE())
                .field("IDLE", &self.IDLE())
                .field("RXNE", &self.RXNE())
                .field("TC", &self.TC())
                .field("TXE", &self.TXE())
                .field("LBDF", &self.LBDF())
                .field("CTSIF", &self.CTSIF())
                .field("CTS", &self.CTS())
                .field("RTOF", &self.RTOF())
                .field("EOBF", &self.EOBF())
                .field("ABRE", &self.ABRE())
                .field("ABRF", &self.ABRF())
                .field("BUSY", &self.BUSY())
                .field("CMF", &self.CMF())
                .field("SBKF", &self.SBKF())
                .field("RWU", &self.RWU())
                .field("WUF", &self.WUF())
                .field("TEACK", &self.TEACK())
                .field("REACK", &self.REACK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ISR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ISR {{ PE: {=bool:?}, FE: {=bool:?}, NF: {=bool:?}, ORE: {=bool:?}, IDLE: {=bool:?}, RXNE: {=bool:?}, TC: {=bool:?}, TXE: {=bool:?}, LBDF: {=bool:?}, CTSIF: {=bool:?}, CTS: {=bool:?}, RTOF: {=bool:?}, EOBF: {=bool:?}, ABRE: {=bool:?}, ABRF: {=bool:?}, BUSY: {=bool:?}, CMF: {=bool:?}, SBKF: {=bool:?}, RWU: {=bool:?}, WUF: {=bool:?}, TEACK: {=bool:?}, REACK: {=bool:?} }}",
                self.PE(),
                self.FE(),
                self.NF(),
                self.ORE(),
                self.IDLE(),
                self.RXNE(),
                self.TC(),
                self.TXE(),
                self.LBDF(),
                self.CTSIF(),
                self.CTS(),
                self.RTOF(),
                self.EOBF(),
                self.ABRE(),
                self.ABRF(),
                self.BUSY(),
                self.CMF(),
                self.SBKF(),
                self.RWU(),
                self.WUF(),
                self.TEACK(),
                self.REACK()
            )
        }
    }
    #[doc = "Receive data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RDR(pub u32);
    impl RDR {
        #[doc = "Receive data value."]
        #[must_use]
        #[inline(always)]
        pub const fn RDR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Receive data value."]
        #[inline(always)]
        pub const fn set_RDR(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for RDR {
        #[inline(always)]
        fn default() -> RDR {
            RDR(0)
        }
    }
    impl core::fmt::Debug for RDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RDR").field("RDR", &self.RDR()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RDR {{ RDR: {=u16:?} }}", self.RDR())
        }
    }
    #[doc = "Request register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RQR(pub u32);
    impl RQR {
        #[doc = "Auto baud rate request."]
        #[must_use]
        #[inline(always)]
        pub const fn ABRRQ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Auto baud rate request."]
        #[inline(always)]
        pub const fn set_ABRRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Send break request."]
        #[must_use]
        #[inline(always)]
        pub const fn SBKRQ(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Send break request."]
        #[inline(always)]
        pub const fn set_SBKRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Mute mode request."]
        #[must_use]
        #[inline(always)]
        pub const fn MMRQ(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Mute mode request."]
        #[inline(always)]
        pub const fn set_MMRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive data flush request."]
        #[must_use]
        #[inline(always)]
        pub const fn RXFRQ(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Receive data flush request."]
        #[inline(always)]
        pub const fn set_RXFRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmit data flush request."]
        #[must_use]
        #[inline(always)]
        pub const fn TXFRQ(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit data flush request."]
        #[inline(always)]
        pub const fn set_TXFRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for RQR {
        #[inline(always)]
        fn default() -> RQR {
            RQR(0)
        }
    }
    impl core::fmt::Debug for RQR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RQR")
                .field("ABRRQ", &self.ABRRQ())
                .field("SBKRQ", &self.SBKRQ())
                .field("MMRQ", &self.MMRQ())
                .field("RXFRQ", &self.RXFRQ())
                .field("TXFRQ", &self.TXFRQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RQR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RQR {{ ABRRQ: {=bool:?}, SBKRQ: {=bool:?}, MMRQ: {=bool:?}, RXFRQ: {=bool:?}, TXFRQ: {=bool:?} }}",
                self.ABRRQ(),
                self.SBKRQ(),
                self.MMRQ(),
                self.RXFRQ(),
                self.TXFRQ()
            )
        }
    }
    #[doc = "Receiver timeout register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RTOR(pub u32);
    impl RTOR {
        #[doc = "Receiver timeout value."]
        #[must_use]
        #[inline(always)]
        pub const fn RTO(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Receiver timeout value."]
        #[inline(always)]
        pub const fn set_RTO(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize))
                | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "Block Length."]
        #[must_use]
        #[inline(always)]
        pub const fn BLEN(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Block Length."]
        #[inline(always)]
        pub const fn set_BLEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for RTOR {
        #[inline(always)]
        fn default() -> RTOR {
            RTOR(0)
        }
    }
    impl core::fmt::Debug for RTOR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RTOR")
                .field("RTO", &self.RTO())
                .field("BLEN", &self.BLEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RTOR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RTOR {{ RTO: {=u32:?}, BLEN: {=u8:?} }}",
                self.RTO(),
                self.BLEN()
            )
        }
    }
    #[doc = "Transmit data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TDR(pub u32);
    impl TDR {
        #[doc = "Transmit data value."]
        #[must_use]
        #[inline(always)]
        pub const fn TDR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Transmit data value."]
        #[inline(always)]
        pub const fn set_TDR(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for TDR {
        #[inline(always)]
        fn default() -> TDR {
            TDR(0)
        }
    }
    impl core::fmt::Debug for TDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TDR").field("TDR", &self.TDR()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TDR {{ TDR: {=u16:?} }}", self.TDR())
        }
    }
}
