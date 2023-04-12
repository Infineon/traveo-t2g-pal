#[doc = "Register `FRFM` reader"]
pub struct R(crate::R<FRFM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRFM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRFM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRFM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRFM` writer"]
pub struct W(crate::W<FRFM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRFM_SPEC>;
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
impl From<crate::W<FRFM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRFM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFID` reader - Mask Frame ID Filter 1 = Ignore corresponding frame ID filter bit. 0 = Corresponding frame ID filter bit is used for rejection filtering"]
pub type MFID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MFID` writer - Mask Frame ID Filter 1 = Ignore corresponding frame ID filter bit. 0 = Corresponding frame ID filter bit is used for rejection filtering"]
pub type MFID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRFM_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 2:12 - Mask Frame ID Filter 1 = Ignore corresponding frame ID filter bit. 0 = Corresponding frame ID filter bit is used for rejection filtering"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R {
        MFID_R::new(((self.bits >> 2) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:12 - Mask Frame ID Filter 1 = Ignore corresponding frame ID filter bit. 0 = Corresponding frame ID filter bit is used for rejection filtering"]
    #[inline(always)]
    #[must_use]
    pub fn mfid(&mut self) -> MFID_W<2> {
        MFID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Rejection Filter Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frfm](index.html) module"]
pub struct FRFM_SPEC;
impl crate::RegisterSpec for FRFM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frfm::R](R) reader structure"]
impl crate::Readable for FRFM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frfm::W](W) writer structure"]
impl crate::Writable for FRFM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRFM to value 0"]
impl crate::Resettable for FRFM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
