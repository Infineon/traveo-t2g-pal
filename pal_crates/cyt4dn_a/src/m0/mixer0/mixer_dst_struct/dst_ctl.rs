#[doc = "Register `DST_CTL` reader"]
pub struct R(crate::R<DST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DST_CTL` writer"]
pub struct W(crate::W<DST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DST_CTL_SPEC>;
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
impl From<crate::W<DST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLED` reader - Destination enable: '0': Disabled. All non-retained MMIO registers (e.g. the DST_FADE_STATUS, DST_FIFO_STATUS and INTR_DST registers) have their fields reset to their default value. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Destination enable: '0': Disabled. All non-retained MMIO registers (e.g. the DST_FADE_STATUS, DST_FIFO_STATUS and INTR_DST registers) have their fields reset to their default value. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, DST_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Destination enable: '0': Disabled. All non-retained MMIO registers (e.g. the DST_FADE_STATUS, DST_FIFO_STATUS and INTR_DST registers) have their fields reset to their default value. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Destination enable: '0': Disabled. All non-retained MMIO registers (e.g. the DST_FADE_STATUS, DST_FIFO_STATUS and INTR_DST registers) have their fields reset to their default value. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_ctl](index.html) module"]
pub struct DST_CTL_SPEC;
impl crate::RegisterSpec for DST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dst_ctl::R](R) reader structure"]
impl crate::Readable for DST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dst_ctl::W](W) writer structure"]
impl crate::Writable for DST_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DST_CTL to value 0"]
impl crate::Resettable for DST_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
