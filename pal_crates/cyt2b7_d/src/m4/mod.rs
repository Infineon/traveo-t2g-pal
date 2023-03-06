#![doc = "Peripheral access API for CYT2B7 microcontrollers (generated using svd2rust v0.28.0 (54a7f49 2023-02-14))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
#[allow(unused_imports)]
use crate::generic::*;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
extern "C" {
    fn CPUINT_IDX0();
    fn CPUINT_IDX1();
    fn CPUINT_IDX2();
    fn CPUINT_IDX3();
    fn CPUINT_IDX4();
    fn CPUINT_IDX5();
    fn CPUINT_IDX6();
    fn CPUINT_IDX7();
    fn INTERNAL0();
    fn INTERNAL1();
    fn INTERNAL2();
    fn INTERNAL3();
    fn INTERNAL4();
    fn INTERNAL5();
    fn INTERNAL6();
    fn INTERNAL7();
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
pub static __INTERRUPTS: [Vector; 16] = [
    Vector {
        _handler: CPUINT_IDX0,
    },
    Vector {
        _handler: CPUINT_IDX1,
    },
    Vector {
        _handler: CPUINT_IDX2,
    },
    Vector {
        _handler: CPUINT_IDX3,
    },
    Vector {
        _handler: CPUINT_IDX4,
    },
    Vector {
        _handler: CPUINT_IDX5,
    },
    Vector {
        _handler: CPUINT_IDX6,
    },
    Vector {
        _handler: CPUINT_IDX7,
    },
    Vector {
        _handler: INTERNAL0,
    },
    Vector {
        _handler: INTERNAL1,
    },
    Vector {
        _handler: INTERNAL2,
    },
    Vector {
        _handler: INTERNAL3,
    },
    Vector {
        _handler: INTERNAL4,
    },
    Vector {
        _handler: INTERNAL5,
    },
    Vector {
        _handler: INTERNAL6,
    },
    Vector {
        _handler: INTERNAL7,
    },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - CPU User Interrupt #0"]
    CPUINT_IDX0 = 0,
    #[doc = "1 - CPU User Interrupt #1"]
    CPUINT_IDX1 = 1,
    #[doc = "2 - CPU User Interrupt #2"]
    CPUINT_IDX2 = 2,
    #[doc = "3 - CPU User Interrupt #3"]
    CPUINT_IDX3 = 3,
    #[doc = "4 - CPU User Interrupt #4"]
    CPUINT_IDX4 = 4,
    #[doc = "5 - CPU User Interrupt #5"]
    CPUINT_IDX5 = 5,
    #[doc = "6 - CPU User Interrupt #6"]
    CPUINT_IDX6 = 6,
    #[doc = "7 - CPU User Interrupt #7"]
    CPUINT_IDX7 = 7,
    #[doc = "8 - Internal SW Interrupt #0"]
    INTERNAL0 = 8,
    #[doc = "9 - Internal SW Interrupt #1"]
    INTERNAL1 = 9,
    #[doc = "10 - Internal SW Interrupt #2"]
    INTERNAL2 = 10,
    #[doc = "11 - Internal SW Interrupt #3"]
    INTERNAL3 = 11,
    #[doc = "12 - Internal SW Interrupt #4"]
    INTERNAL4 = 12,
    #[doc = "13 - Internal SW Interrupt #5"]
    INTERNAL5 = 13,
    #[doc = "14 - Internal SW Interrupt #6"]
    INTERNAL6 = 14,
    #[doc = "15 - Internal SW Interrupt #7"]
    INTERNAL7 = 15,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "Peripheral interconnect"]
pub struct PERI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PERI {}
impl PERI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const peri::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const peri::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PERI {
    type Target = peri::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PERI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI").finish()
    }
}
#[doc = "Peripheral interconnect"]
pub mod peri;
#[doc = "Peripheral interconnect, master interface"]
pub struct PERI_MS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PERI_MS {}
impl PERI_MS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const peri_ms::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const peri_ms::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PERI_MS {
    type Target = peri_ms::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PERI_MS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_MS").finish()
    }
}
#[doc = "Peripheral interconnect, master interface"]
pub mod peri_ms;
#[doc = "Cryptography component"]
pub struct CRYPTO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYPTO {}
impl CRYPTO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crypto::RegisterBlock = 0x4010_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crypto::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRYPTO {
    type Target = crypto::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRYPTO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRYPTO").finish()
    }
}
#[doc = "Cryptography component"]
pub mod crypto;
#[doc = "CPU subsystem (CPUSS)"]
pub struct CPUSS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPUSS {}
impl CPUSS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cpuss::RegisterBlock = 0x4020_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpuss::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CPUSS {
    type Target = cpuss::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CPUSS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUSS").finish()
    }
}
#[doc = "CPU subsystem (CPUSS)"]
pub mod cpuss;
#[doc = "Fault structures"]
pub struct FAULT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FAULT {}
impl FAULT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const fault::RegisterBlock = 0x4021_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fault::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FAULT {
    type Target = fault::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FAULT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FAULT").finish()
    }
}
#[doc = "Fault structures"]
pub mod fault;
#[doc = "IPC"]
pub struct IPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IPC {}
impl IPC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ipc::RegisterBlock = 0x4022_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ipc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for IPC {
    type Target = ipc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for IPC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPC").finish()
    }
}
#[doc = "IPC"]
pub mod ipc;
#[doc = "Protection"]
pub struct PROT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PROT {}
impl PROT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const prot::RegisterBlock = 0x4023_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const prot::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PROT {
    type Target = prot::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PROT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PROT").finish()
    }
}
#[doc = "Protection"]
pub mod prot;
#[doc = "Flash controller"]
pub struct FLASHC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASHC {}
impl FLASHC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const flashc::RegisterBlock = 0x4024_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flashc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FLASHC {
    type Target = flashc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FLASHC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHC").finish()
    }
}
#[doc = "Flash controller"]
pub mod flashc;
#[doc = "SRSS Core Registers (ver2)"]
pub struct SRSS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SRSS {}
impl SRSS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const srss::RegisterBlock = 0x4026_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const srss::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SRSS {
    type Target = srss::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SRSS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRSS").finish()
    }
}
#[doc = "SRSS Core Registers (ver2)"]
pub mod srss;
#[doc = "SRSS Backup Domain (ver2)"]
pub struct BACKUP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BACKUP {}
impl BACKUP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const backup::RegisterBlock = 0x4027_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const backup::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BACKUP {
    type Target = backup::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BACKUP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP").finish()
    }
}
#[doc = "SRSS Backup Domain (ver2)"]
pub mod backup;
#[doc = "Datawire Controller"]
pub struct DW0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DW0 {}
impl DW0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dw0::RegisterBlock = 0x4028_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dw0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DW0 {
    type Target = dw0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DW0").finish()
    }
}
#[doc = "Datawire Controller"]
pub mod dw0;
#[doc = "Datawire Controller"]
pub struct DW1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DW1 {}
impl DW1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dw0::RegisterBlock = 0x4029_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dw0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DW1 {
    type Target = dw0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DW1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DW1").finish()
    }
}
#[doc = "Datawire Controller"]
pub use self::dw0 as dw1;
#[doc = "DMAC"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dmac::RegisterBlock = 0x402a_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DMAC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC").finish()
    }
}
#[doc = "DMAC"]
pub mod dmac;
#[doc = "EFUSE MXS40 registers"]
pub struct EFUSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFUSE {}
impl EFUSE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const efuse::RegisterBlock = 0x402c_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efuse::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EFUSE {
    type Target = efuse::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EFUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE").finish()
    }
}
#[doc = "EFUSE MXS40 registers"]
pub mod efuse;
#[doc = "eFUSE memory"]
pub struct EFUSE_DATA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFUSE_DATA {}
impl EFUSE_DATA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const efuse_data::RegisterBlock = 0x402c_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efuse_data::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EFUSE_DATA {
    type Target = efuse_data::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EFUSE_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE_DATA").finish()
    }
}
#[doc = "eFUSE memory"]
pub mod efuse_data;
#[doc = "High Speed IO Matrix (HSIOM)"]
pub struct HSIOM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSIOM {}
impl HSIOM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const hsiom::RegisterBlock = 0x4030_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hsiom::RegisterBlock {
        Self::PTR
    }
}
impl Deref for HSIOM {
    type Target = hsiom::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for HSIOM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSIOM").finish()
    }
}
#[doc = "High Speed IO Matrix (HSIOM)"]
pub mod hsiom;
#[doc = "GPIO port control/configuration"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio::RegisterBlock = 0x4031_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO").finish()
    }
}
#[doc = "GPIO port control/configuration"]
pub mod gpio;
#[doc = "Programmable IO configuration"]
pub struct SMARTIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMARTIO {}
impl SMARTIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const smartio::RegisterBlock = 0x4032_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smartio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SMARTIO {
    type Target = smartio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SMARTIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMARTIO").finish()
    }
}
#[doc = "Programmable IO configuration"]
pub mod smartio;
#[doc = "Timer/Counter/PWM"]
pub struct TCPWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCPWM0 {}
impl TCPWM0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tcpwm0::RegisterBlock = 0x4038_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcpwm0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TCPWM0 {
    type Target = tcpwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TCPWM0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCPWM0").finish()
    }
}
#[doc = "Timer/Counter/PWM"]
pub mod tcpwm0;
#[doc = "Event generator"]
pub struct EVTGEN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVTGEN0 {}
impl EVTGEN0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const evtgen0::RegisterBlock = 0x403f_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const evtgen0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EVTGEN0 {
    type Target = evtgen0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EVTGEN0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTGEN0").finish()
    }
}
#[doc = "Event generator"]
pub mod evtgen0;
#[doc = "LIN"]
pub struct LIN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LIN0 {}
impl LIN0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lin0::RegisterBlock = 0x4050_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lin0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LIN0 {
    type Target = lin0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LIN0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LIN0").finish()
    }
}
#[doc = "LIN"]
pub mod lin0;
#[doc = "CAN Controller"]
pub struct CANFD0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CANFD0 {}
impl CANFD0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const canfd0::RegisterBlock = 0x4052_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const canfd0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CANFD0 {
    type Target = canfd0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CANFD0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANFD0").finish()
    }
}
#[doc = "CAN Controller"]
pub mod canfd0;
#[doc = "CAN Controller"]
pub struct CANFD1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CANFD1 {}
impl CANFD1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const canfd0::RegisterBlock = 0x4054_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const canfd0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CANFD1 {
    type Target = canfd0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CANFD1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANFD1").finish()
    }
}
#[doc = "CAN Controller"]
pub use self::canfd0 as canfd1;
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB0 {}
impl SCB0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const scb0::RegisterBlock = 0x4060_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCB0 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCB0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCB0").finish()
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub mod scb0;
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB1 {}
impl SCB1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const scb0::RegisterBlock = 0x4061_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCB1 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCB1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCB1").finish()
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub use self::scb0 as scb1;
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB2 {}
impl SCB2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const scb0::RegisterBlock = 0x4062_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCB2 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCB2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCB2").finish()
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub use self::scb0 as scb2;
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB3 {}
impl SCB3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const scb0::RegisterBlock = 0x4063_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCB3 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCB3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCB3").finish()
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub use self::scb0 as scb3;
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB4 {}
impl SCB4 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const scb0::RegisterBlock = 0x4064_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCB4 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCB4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCB4").finish()
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub use self::scb0 as scb4;
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB5 {}
impl SCB5 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const scb0::RegisterBlock = 0x4065_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCB5 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCB5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCB5").finish()
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub use self::scb0 as scb5;
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB6 {}
impl SCB6 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const scb0::RegisterBlock = 0x4066_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCB6 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCB6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCB6").finish()
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub use self::scb0 as scb6;
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub struct SCB7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB7 {}
impl SCB7 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const scb0::RegisterBlock = 0x4067_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCB7 {
    type Target = scb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCB7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCB7").finish()
    }
}
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub use self::scb0 as scb7;
#[doc = "Programmable Analog Subsystem for S40E"]
pub struct PASS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PASS0 {}
impl PASS0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pass0::RegisterBlock = 0x4090_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pass0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PASS0 {
    type Target = pass0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PASS0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PASS0").finish()
    }
}
#[doc = "Programmable Analog Subsystem for S40E"]
pub mod pass0;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PERI"]
    pub PERI: PERI,
    #[doc = "PERI_MS"]
    pub PERI_MS: PERI_MS,
    #[doc = "CRYPTO"]
    pub CRYPTO: CRYPTO,
    #[doc = "CPUSS"]
    pub CPUSS: CPUSS,
    #[doc = "FAULT"]
    pub FAULT: FAULT,
    #[doc = "IPC"]
    pub IPC: IPC,
    #[doc = "PROT"]
    pub PROT: PROT,
    #[doc = "FLASHC"]
    pub FLASHC: FLASHC,
    #[doc = "SRSS"]
    pub SRSS: SRSS,
    #[doc = "BACKUP"]
    pub BACKUP: BACKUP,
    #[doc = "DW0"]
    pub DW0: DW0,
    #[doc = "DW1"]
    pub DW1: DW1,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "EFUSE"]
    pub EFUSE: EFUSE,
    #[doc = "EFUSE_DATA"]
    pub EFUSE_DATA: EFUSE_DATA,
    #[doc = "HSIOM"]
    pub HSIOM: HSIOM,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "SMARTIO"]
    pub SMARTIO: SMARTIO,
    #[doc = "TCPWM0"]
    pub TCPWM0: TCPWM0,
    #[doc = "EVTGEN0"]
    pub EVTGEN0: EVTGEN0,
    #[doc = "LIN0"]
    pub LIN0: LIN0,
    #[doc = "CANFD0"]
    pub CANFD0: CANFD0,
    #[doc = "CANFD1"]
    pub CANFD1: CANFD1,
    #[doc = "SCB0"]
    pub SCB0: SCB0,
    #[doc = "SCB1"]
    pub SCB1: SCB1,
    #[doc = "SCB2"]
    pub SCB2: SCB2,
    #[doc = "SCB3"]
    pub SCB3: SCB3,
    #[doc = "SCB4"]
    pub SCB4: SCB4,
    #[doc = "SCB5"]
    pub SCB5: SCB5,
    #[doc = "SCB6"]
    pub SCB6: SCB6,
    #[doc = "SCB7"]
    pub SCB7: SCB7,
    #[doc = "PASS0"]
    pub PASS0: PASS0,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PERI: PERI {
                _marker: PhantomData,
            },
            PERI_MS: PERI_MS {
                _marker: PhantomData,
            },
            CRYPTO: CRYPTO {
                _marker: PhantomData,
            },
            CPUSS: CPUSS {
                _marker: PhantomData,
            },
            FAULT: FAULT {
                _marker: PhantomData,
            },
            IPC: IPC {
                _marker: PhantomData,
            },
            PROT: PROT {
                _marker: PhantomData,
            },
            FLASHC: FLASHC {
                _marker: PhantomData,
            },
            SRSS: SRSS {
                _marker: PhantomData,
            },
            BACKUP: BACKUP {
                _marker: PhantomData,
            },
            DW0: DW0 {
                _marker: PhantomData,
            },
            DW1: DW1 {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            EFUSE: EFUSE {
                _marker: PhantomData,
            },
            EFUSE_DATA: EFUSE_DATA {
                _marker: PhantomData,
            },
            HSIOM: HSIOM {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            SMARTIO: SMARTIO {
                _marker: PhantomData,
            },
            TCPWM0: TCPWM0 {
                _marker: PhantomData,
            },
            EVTGEN0: EVTGEN0 {
                _marker: PhantomData,
            },
            LIN0: LIN0 {
                _marker: PhantomData,
            },
            CANFD0: CANFD0 {
                _marker: PhantomData,
            },
            CANFD1: CANFD1 {
                _marker: PhantomData,
            },
            SCB0: SCB0 {
                _marker: PhantomData,
            },
            SCB1: SCB1 {
                _marker: PhantomData,
            },
            SCB2: SCB2 {
                _marker: PhantomData,
            },
            SCB3: SCB3 {
                _marker: PhantomData,
            },
            SCB4: SCB4 {
                _marker: PhantomData,
            },
            SCB5: SCB5 {
                _marker: PhantomData,
            },
            SCB6: SCB6 {
                _marker: PhantomData,
            },
            SCB7: SCB7 {
                _marker: PhantomData,
            },
            PASS0: PASS0 {
                _marker: PhantomData,
            },
        }
    }
}
