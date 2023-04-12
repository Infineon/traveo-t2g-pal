#[doc = "Register `READACK` writer"]
pub struct W(crate::W<READACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READACK_SPEC>;
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
impl From<crate::W<READACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READACK` writer - Writing a one acknowledges the read of the status registers ActiveDimensions and Offset."]
pub type READACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, READACK_SPEC, bool, O>;
#[doc = "Field `READACK_ERROR` writer - Writing a one acknowledges the read of the status register Error."]
pub type READACK_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, READACK_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Writing a one acknowledges the read of the status registers ActiveDimensions and Offset."]
    #[inline(always)]
    #[must_use]
    pub fn readack(&mut self) -> READACK_W<0> {
        READACK_W::new(self)
    }
    #[doc = "Bit 1 - Writing a one acknowledges the read of the status register Error."]
    #[inline(always)]
    #[must_use]
    pub fn readack_error(&mut self) -> READACK_ERROR_W<1> {
        READACK_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Drawing Engine read acknowledge register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readack](index.html) module"]
pub struct READACK_SPEC;
impl crate::RegisterSpec for READACK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [readack::W](W) writer structure"]
impl crate::Writable for READACK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READACK to value 0"]
impl crate::Resettable for READACK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
