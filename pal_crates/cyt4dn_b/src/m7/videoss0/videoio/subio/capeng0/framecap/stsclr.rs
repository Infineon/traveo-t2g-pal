#[doc = "Register `STSCLR` writer"]
pub struct W(crate::W<STSCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STSCLR_SPEC>;
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
impl From<crate::W<STSCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STSCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUSCLEAR` writer - Clears status bits of Sts register."]
pub type STATUSCLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STSCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clears status bits of Sts register."]
    #[inline(always)]
    #[must_use]
    pub fn statusclear(&mut self) -> STATUSCLEAR_W<0> {
        STATUSCLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameCap status clear register. Clears the status bits of Sts register, Mdr_SizeChange and Csv_lost field.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stsclr](index.html) module"]
pub struct STSCLR_SPEC;
impl crate::RegisterSpec for STSCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [stsclr::W](W) writer structure"]
impl crate::Writable for STSCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STSCLR to value 0"]
impl crate::Resettable for STSCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
