#[doc = "Register `CONTROLTRIGGER` writer"]
pub struct W(crate::W<CONTROLTRIGGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROLTRIGGER_SPEC>;
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
impl From<crate::W<CONTROLTRIGGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROLTRIGGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHDTOKGEN` writer - Writing a 1 to this will load shadow registers into the active configuration with the next start of frame and send a shadow load token to subsequent units."]
pub type SHDTOKGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROLTRIGGER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Writing a 1 to this will load shadow registers into the active configuration with the next start of frame and send a shadow load token to subsequent units."]
    #[inline(always)]
    #[must_use]
    pub fn shdtokgen(&mut self) -> SHDTOKGEN_W<0> {
        SHDTOKGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow load trigger.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [controltrigger](index.html) module"]
pub struct CONTROLTRIGGER_SPEC;
impl crate::RegisterSpec for CONTROLTRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [controltrigger::W](W) writer structure"]
impl crate::Writable for CONTROLTRIGGER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROLTRIGGER to value 0"]
impl crate::Resettable for CONTROLTRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
