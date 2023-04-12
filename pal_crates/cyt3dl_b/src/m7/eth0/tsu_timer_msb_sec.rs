#[doc = "Register `TSU_TIMER_MSB_SEC` reader"]
pub struct R(crate::R<TSU_TIMER_MSB_SEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSU_TIMER_MSB_SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSU_TIMER_MSB_SEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSU_TIMER_MSB_SEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSU_TIMER_MSB_SEC` writer"]
pub struct W(crate::W<TSU_TIMER_MSB_SEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSU_TIMER_MSB_SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TSU_TIMER_MSB_SEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSU_TIMER_MSB_SEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_MSB_SEC` reader - TSU timer value (s). Most significant 16 bits of seconds timer count. The register is writeable. The 48-bit counter increments by one when the 1588 nanoseconds counter counts to one second. It may also be incremented or decremented when the timer adjust register is written (if decremented from zero the 48-bit combined count would roll back to 0xFFFFFFFFFFFF). Note: The value of this register is used only when the lower 32 bit register is written to. This is to ensure a single update of the 48 bit seconds value"]
pub type TIMER_MSB_SEC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER_MSB_SEC` writer - TSU timer value (s). Most significant 16 bits of seconds timer count. The register is writeable. The 48-bit counter increments by one when the 1588 nanoseconds counter counts to one second. It may also be incremented or decremented when the timer adjust register is written (if decremented from zero the 48-bit combined count would roll back to 0xFFFFFFFFFFFF). Note: The value of this register is used only when the lower 32 bit register is written to. This is to ensure a single update of the 48 bit seconds value"]
pub type TIMER_MSB_SEC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSU_TIMER_MSB_SEC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - TSU timer value (s). Most significant 16 bits of seconds timer count. The register is writeable. The 48-bit counter increments by one when the 1588 nanoseconds counter counts to one second. It may also be incremented or decremented when the timer adjust register is written (if decremented from zero the 48-bit combined count would roll back to 0xFFFFFFFFFFFF). Note: The value of this register is used only when the lower 32 bit register is written to. This is to ensure a single update of the 48 bit seconds value"]
    #[inline(always)]
    pub fn timer_msb_sec(&self) -> TIMER_MSB_SEC_R {
        TIMER_MSB_SEC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TSU timer value (s). Most significant 16 bits of seconds timer count. The register is writeable. The 48-bit counter increments by one when the 1588 nanoseconds counter counts to one second. It may also be incremented or decremented when the timer adjust register is written (if decremented from zero the 48-bit combined count would roll back to 0xFFFFFFFFFFFF). Note: The value of this register is used only when the lower 32 bit register is written to. This is to ensure a single update of the 48 bit seconds value"]
    #[inline(always)]
    #[must_use]
    pub fn timer_msb_sec(&mut self) -> TIMER_MSB_SEC_W<0> {
        TIMER_MSB_SEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Seconds Register (47 to 32 bits)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_timer_msb_sec](index.html) module"]
pub struct TSU_TIMER_MSB_SEC_SPEC;
impl crate::RegisterSpec for TSU_TIMER_MSB_SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsu_timer_msb_sec::R](R) reader structure"]
impl crate::Readable for TSU_TIMER_MSB_SEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsu_timer_msb_sec::W](W) writer structure"]
impl crate::Writable for TSU_TIMER_MSB_SEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSU_TIMER_MSB_SEC to value 0"]
impl crate::Resettable for TSU_TIMER_MSB_SEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
