#[doc = "Register `FRMSTRTTRIG` writer"]
pub struct W(crate::W<FRMSTRTTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRMSTRTTRIG_SPEC>;
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
impl From<crate::W<FRMSTRTTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRMSTRTTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSRMNT_STRT_TRIG` writer - Writing value 1 into this field initiates the measurement of the next frame. (format is unsigned integer)"]
pub type MSRMNT_STRT_TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRMSTRTTRIG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Writing value 1 into this field initiates the measurement of the next frame. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn msrmnt_strt_trig(&mut self) -> MSRMNT_STRT_TRIG_W<0> {
        MSRMNT_STRT_TRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Triggers for loading shadow registers and starting measurement with next input frame.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frmstrttrig](index.html) module"]
pub struct FRMSTRTTRIG_SPEC;
impl crate::RegisterSpec for FRMSTRTTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [frmstrttrig::W](W) writer structure"]
impl crate::Writable for FRMSTRTTRIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRMSTRTTRIG to value 0"]
impl crate::Resettable for FRMSTRTTRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
