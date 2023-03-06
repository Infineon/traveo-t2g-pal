#[doc = "Register `INTR_MASK` reader"]
pub struct R(crate::R<INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_MASK` writer"]
pub struct W(crate::W<INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_MASK_SPEC>;
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
impl From<crate::W<INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GRP_DONE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type GRP_DONE_MASK_R = crate::BitReader<bool>;
#[doc = "Field `GRP_DONE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type GRP_DONE_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `GRP_CANCELLED_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type GRP_CANCELLED_MASK_R = crate::BitReader<bool>;
#[doc = "Field `GRP_CANCELLED_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type GRP_CANCELLED_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `GRP_OVERFLOW_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type GRP_OVERFLOW_MASK_R = crate::BitReader<bool>;
#[doc = "Field `GRP_OVERFLOW_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type GRP_OVERFLOW_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CH_RANGE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type CH_RANGE_MASK_R = crate::BitReader<bool>;
#[doc = "Field `CH_RANGE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type CH_RANGE_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CH_PULSE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type CH_PULSE_MASK_R = crate::BitReader<bool>;
#[doc = "Field `CH_PULSE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type CH_PULSE_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CH_OVERFLOW_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type CH_OVERFLOW_MASK_R = crate::BitReader<bool>;
#[doc = "Field `CH_OVERFLOW_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type CH_OVERFLOW_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn grp_done_mask(&self) -> GRP_DONE_MASK_R {
        GRP_DONE_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn grp_cancelled_mask(&self) -> GRP_CANCELLED_MASK_R {
        GRP_CANCELLED_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn grp_overflow_mask(&self) -> GRP_OVERFLOW_MASK_R {
        GRP_OVERFLOW_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ch_range_mask(&self) -> CH_RANGE_MASK_R {
        CH_RANGE_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ch_pulse_mask(&self) -> CH_PULSE_MASK_R {
        CH_PULSE_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ch_overflow_mask(&self) -> CH_OVERFLOW_MASK_R {
        CH_OVERFLOW_MASK_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn grp_done_mask(&mut self) -> GRP_DONE_MASK_W<0> {
        GRP_DONE_MASK_W::new(self)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn grp_cancelled_mask(&mut self) -> GRP_CANCELLED_MASK_W<1> {
        GRP_CANCELLED_MASK_W::new(self)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn grp_overflow_mask(&mut self) -> GRP_OVERFLOW_MASK_W<2> {
        GRP_OVERFLOW_MASK_W::new(self)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn ch_range_mask(&mut self) -> CH_RANGE_MASK_W<8> {
        CH_RANGE_MASK_W::new(self)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn ch_pulse_mask(&mut self) -> CH_PULSE_MASK_W<9> {
        CH_PULSE_MASK_W::new(self)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn ch_overflow_mask(&mut self) -> CH_OVERFLOW_MASK_W<10> {
        CH_OVERFLOW_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](index.html) module"]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_mask::R](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
