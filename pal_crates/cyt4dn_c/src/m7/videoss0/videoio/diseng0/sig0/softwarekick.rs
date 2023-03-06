#[doc = "Register `SOFTWAREKICK` writer"]
pub struct W(crate::W<SOFTWAREKICK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTWAREKICK_SPEC>;
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
impl From<crate::W<SOFTWAREKICK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTWAREKICK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KICK` writer - ContinueMode.EnCont=0: Write '1' to this field in order to start signature computation with next frame. ContinueMode.EnCont=1: This field is ignored."]
pub type KICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOFTWAREKICK_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - ContinueMode.EnCont=0: Write '1' to this field in order to start signature computation with next frame. ContinueMode.EnCont=1: This field is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn kick(&mut self) -> KICK_W<0> {
        KICK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Signature measurement trigger.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softwarekick](index.html) module"]
pub struct SOFTWAREKICK_SPEC;
impl crate::RegisterSpec for SOFTWAREKICK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [softwarekick::W](W) writer structure"]
impl crate::Writable for SOFTWAREKICK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOFTWAREKICK to value 0"]
impl crate::Resettable for SOFTWAREKICK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
