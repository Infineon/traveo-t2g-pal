#[doc = "Register `PWR_HIB_WAKE_CTL2` reader"]
pub struct R(crate::R<PWR_HIB_WAKE_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_HIB_WAKE_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_HIB_WAKE_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_HIB_WAKE_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_HIB_WAKE_CTL2` writer"]
pub struct W(crate::W<PWR_HIB_WAKE_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_HIB_WAKE_CTL2_SPEC>;
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
impl From<crate::W<PWR_HIB_WAKE_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_HIB_WAKE_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_WAKE_SRC` reader - Each bit selects the polarity for the corresponding HIBERNATE wakeup source. The number and assignment of wakeup sources are product-specific. 0: Wakes when unmasked input is 0. 1: Wakes when unmasked input is 1."]
pub type HIB_WAKE_SRC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HIB_WAKE_SRC` writer - Each bit selects the polarity for the corresponding HIBERNATE wakeup source. The number and assignment of wakeup sources are product-specific. 0: Wakes when unmasked input is 0. 1: Wakes when unmasked input is 1."]
pub type HIB_WAKE_SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_HIB_WAKE_CTL2_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Each bit selects the polarity for the corresponding HIBERNATE wakeup source. The number and assignment of wakeup sources are product-specific. 0: Wakes when unmasked input is 0. 1: Wakes when unmasked input is 1."]
    #[inline(always)]
    pub fn hib_wake_src(&self) -> HIB_WAKE_SRC_R {
        HIB_WAKE_SRC_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Each bit selects the polarity for the corresponding HIBERNATE wakeup source. The number and assignment of wakeup sources are product-specific. 0: Wakes when unmasked input is 0. 1: Wakes when unmasked input is 1."]
    #[inline(always)]
    #[must_use]
    pub fn hib_wake_src(&mut self) -> HIB_WAKE_SRC_W<0> {
        HIB_WAKE_SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernate Wakeup Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_hib_wake_ctl2](index.html) module"]
pub struct PWR_HIB_WAKE_CTL2_SPEC;
impl crate::RegisterSpec for PWR_HIB_WAKE_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_hib_wake_ctl2::R](R) reader structure"]
impl crate::Readable for PWR_HIB_WAKE_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_hib_wake_ctl2::W](W) writer structure"]
impl crate::Writable for PWR_HIB_WAKE_CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_HIB_WAKE_CTL2 to value 0"]
impl crate::Resettable for PWR_HIB_WAKE_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
