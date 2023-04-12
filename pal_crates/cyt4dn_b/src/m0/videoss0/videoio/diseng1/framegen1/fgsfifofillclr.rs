#[doc = "Register `FGSFIFOFILLCLR` writer"]
pub struct W(crate::W<FGSFIFOFILLCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGSFIFOFILLCLR_SPEC>;
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
impl From<crate::W<FGSFIFOFILLCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGSFIFOFILLCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFIFOFILLCLR` writer - Write for clearing register FgSFifoMin and FgSFifoMax. (For debugging purposes only)."]
pub type SFIFOFILLCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGSFIFOFILLCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Write for clearing register FgSFifoMin and FgSFifoMax. (For debugging purposes only)."]
    #[inline(always)]
    #[must_use]
    pub fn sfifofillclr(&mut self) -> SFIFOFILLCLR_W<0> {
        SFIFOFILLCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Secondary FIFO Fill Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgsfifofillclr](index.html) module"]
pub struct FGSFIFOFILLCLR_SPEC;
impl crate::RegisterSpec for FGSFIFOFILLCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fgsfifofillclr::W](W) writer structure"]
impl crate::Writable for FGSFIFOFILLCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGSFIFOFILLCLR to value 0"]
impl crate::Resettable for FGSFIFOFILLCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
