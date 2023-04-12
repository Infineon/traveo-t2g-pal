#[doc = "Register `PLL800_STATUS` reader"]
pub struct R(crate::R<PLL800_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL800_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL800_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL800_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL800_STATUS` writer"]
pub struct W(crate::W<PLL800_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL800_STATUS_SPEC>;
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
impl From<crate::W<PLL800_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL800_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKED` reader - PLL Lock Indicator. Setting OUTPUT_ENABLE=1 is only permitted when this bit is 1"]
pub type LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `UNLOCK_OCCURRED` reader - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
pub type UNLOCK_OCCURRED_R = crate::BitReader<bool>;
#[doc = "Field `UNLOCK_OCCURRED` writer - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
pub type UNLOCK_OCCURRED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL800_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PLL Lock Indicator. Setting OUTPUT_ENABLE=1 is only permitted when this bit is 1"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    pub fn unlock_occurred(&self) -> UNLOCK_OCCURRED_R {
        UNLOCK_OCCURRED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    #[must_use]
    pub fn unlock_occurred(&mut self) -> UNLOCK_OCCURRED_W<1> {
        UNLOCK_OCCURRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "800MHz PLL Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll800_status](index.html) module"]
pub struct PLL800_STATUS_SPEC;
impl crate::RegisterSpec for PLL800_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll800_status::R](R) reader structure"]
impl crate::Readable for PLL800_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll800_status::W](W) writer structure"]
impl crate::Writable for PLL800_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL800_STATUS to value 0"]
impl crate::Resettable for PLL800_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
