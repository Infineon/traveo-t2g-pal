#[doc = "Register `SWRESET` writer"]
pub struct W(crate::W<SWRESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRESET_SPEC>;
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
impl From<crate::W<SWRESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORESTART_RSVD` writer - N/A"]
pub type CORESTART_RSVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRESET_SPEC, bool, O>;
#[doc = "Field `CORERESUME_RSVD` writer - N/A"]
pub type CORERESUME_RSVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRESET_SPEC, bool, O>;
#[doc = "Field `CORERESET` writer - Software reset of the JPEG core. Setting this field to '1' initializes the following registers (others are retained): DECODINGSTATUS INTR_DEC ERRORCODE CORRECTIONCROPSTATUS SUSPEND DNL"]
pub type CORERESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRESET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn corestart_rsvd(&mut self) -> CORESTART_RSVD_W<0> {
        CORESTART_RSVD_W::new(self)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn coreresume_rsvd(&mut self) -> CORERESUME_RSVD_W<1> {
        CORERESUME_RSVD_W::new(self)
    }
    #[doc = "Bit 31 - Software reset of the JPEG core. Setting this field to '1' initializes the following registers (others are retained): DECODINGSTATUS INTR_DEC ERRORCODE CORRECTIONCROPSTATUS SUSPEND DNL"]
    #[inline(always)]
    #[must_use]
    pub fn corereset(&mut self) -> CORERESET_W<31> {
        CORERESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software reset trigger.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreset](index.html) module"]
pub struct SWRESET_SPEC;
impl crate::RegisterSpec for SWRESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swreset::W](W) writer structure"]
impl crate::Writable for SWRESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWRESET to value 0"]
impl crate::Resettable for SWRESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
