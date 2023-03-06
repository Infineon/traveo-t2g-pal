#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DLL_LOCKED` reader - 0: DLL is not locked 1: DLL is locked Once the reference clock to the DLL is stable and the DLL is powered (CTL.ENABLED=1), the DLL should lock within 8us. The lock time will also be incurred when transitioning from DeepSleep to Active. Wait another 4us beyond the lock indication before executing any transactions on the memory interface; the MDL and SDLs require that extra time after the lock indication to reach their final stable delay values. If the DLL suddenly becomes unlocked then the TX interface will be held in reset to prevent any further transactions to the memory. The block should be disabled (CTL.ENABLED=0) to reset all other non-retention registers. Any data in the internal FIFOs will be lost. The DLL suddenly unlocking is considered an unusual event that would only occur if the reference clock to the DLL had a sudden change in frequency, which should only happen at the direction of SW. When transitioning to DeepSleep the DLL will naturally unlock. However, upon returning to Active all non-retention registers are reset and thus disabling the block to enact that reset is not necessary."]
pub type DLL_LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - AHB Cache, AXI interface, cryptography, XIP, device interface, MPC (if present) initialization, or any other key logic busy in the IP: '0': not busy '1': busy When BUSY is '0', the IP can be safely disabled without: - the potential loss of transient write data. - the potential risk of aborting an inflight SPI device interface transfer. - disrupting MPC (if present) initialization When BUSY is '0', the mode of operation (ARB_MODE or MMIO_MODE) can be safely changed."]
pub type BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - 0: DLL is not locked 1: DLL is locked Once the reference clock to the DLL is stable and the DLL is powered (CTL.ENABLED=1), the DLL should lock within 8us. The lock time will also be incurred when transitioning from DeepSleep to Active. Wait another 4us beyond the lock indication before executing any transactions on the memory interface; the MDL and SDLs require that extra time after the lock indication to reach their final stable delay values. If the DLL suddenly becomes unlocked then the TX interface will be held in reset to prevent any further transactions to the memory. The block should be disabled (CTL.ENABLED=0) to reset all other non-retention registers. Any data in the internal FIFOs will be lost. The DLL suddenly unlocking is considered an unusual event that would only occur if the reference clock to the DLL had a sudden change in frequency, which should only happen at the direction of SW. When transitioning to DeepSleep the DLL will naturally unlock. However, upon returning to Active all non-retention registers are reset and thus disabling the block to enact that reset is not necessary."]
    #[inline(always)]
    pub fn dll_locked(&self) -> DLL_LOCKED_R {
        DLL_LOCKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - AHB Cache, AXI interface, cryptography, XIP, device interface, MPC (if present) initialization, or any other key logic busy in the IP: '0': not busy '1': busy When BUSY is '0', the IP can be safely disabled without: - the potential loss of transient write data. - the potential risk of aborting an inflight SPI device interface transfer. - disrupting MPC (if present) initialization When BUSY is '0', the mode of operation (ARB_MODE or MMIO_MODE) can be safely changed."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
