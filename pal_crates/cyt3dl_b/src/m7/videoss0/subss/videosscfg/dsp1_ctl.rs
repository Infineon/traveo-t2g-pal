#[doc = "Register `DSP1_CTL` reader"]
pub struct R(crate::R<DSP1_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSP1_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSP1_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSP1_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSP1_CTL` writer"]
pub struct W(crate::W<DSP1_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSP1_CTL_SPEC>;
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
impl From<crate::W<DSP1_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSP1_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSP1_ENABLED` reader - This field, along with CTL.ENABLED above which enables the whole videoss IP, controls the clock and reset release of the DSP1 clock domain."]
pub type DSP1_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `DSP1_ENABLED` writer - This field, along with CTL.ENABLED above which enables the whole videoss IP, controls the clock and reset release of the DSP1 clock domain."]
pub type DSP1_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSP1_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - This field, along with CTL.ENABLED above which enables the whole videoss IP, controls the clock and reset release of the DSP1 clock domain."]
    #[inline(always)]
    pub fn dsp1_enabled(&self) -> DSP1_ENABLED_R {
        DSP1_ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - This field, along with CTL.ENABLED above which enables the whole videoss IP, controls the clock and reset release of the DSP1 clock domain."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1_enabled(&mut self) -> DSP1_ENABLED_W<31> {
        DSP1_ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable for Display Engine 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsp1_ctl](index.html) module"]
pub struct DSP1_CTL_SPEC;
impl crate::RegisterSpec for DSP1_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsp1_ctl::R](R) reader structure"]
impl crate::Readable for DSP1_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsp1_ctl::W](W) writer structure"]
impl crate::Writable for DSP1_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSP1_CTL to value 0"]
impl crate::Resettable for DSP1_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
