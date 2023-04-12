#[doc = "Register `GTUC10` reader"]
pub struct R(crate::R<GTUC10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUC10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUC10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUC10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUC10` writer"]
pub struct W(crate::W<GTUC10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUC10_SPEC>;
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
impl From<crate::W<GTUC10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUC10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOC` reader - Maximum Offset Correction (pOffsetCorrectionOut) Holds the maximum permitted offset correction value to be applied by the internal clock synchronization algorithm (absolute value). The CC checks only the internal offset correction value against the maximum offset correction value. Valid values are 5 to 15266 uT."]
pub type MOC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MOC` writer - Maximum Offset Correction (pOffsetCorrectionOut) Holds the maximum permitted offset correction value to be applied by the internal clock synchronization algorithm (absolute value). The CC checks only the internal offset correction value against the maximum offset correction value. Valid values are 5 to 15266 uT."]
pub type MOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC10_SPEC, u16, u16, 14, O>;
#[doc = "Field `MRC` reader - Maximum Rate Correction (pRateCorrectionOut) Holds the maximum permitted rate correction value to be applied by the internal clock synchronization algorithm. The CC checks only the internal rate correction value against the maximum rate correction value (absolute value). Valid values are 2 to 1923 uT."]
pub type MRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MRC` writer - Maximum Rate Correction (pRateCorrectionOut) Holds the maximum permitted rate correction value to be applied by the internal clock synchronization algorithm. The CC checks only the internal rate correction value against the maximum rate correction value (absolute value). Valid values are 2 to 1923 uT."]
pub type MRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC10_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:13 - Maximum Offset Correction (pOffsetCorrectionOut) Holds the maximum permitted offset correction value to be applied by the internal clock synchronization algorithm (absolute value). The CC checks only the internal offset correction value against the maximum offset correction value. Valid values are 5 to 15266 uT."]
    #[inline(always)]
    pub fn moc(&self) -> MOC_R {
        MOC_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:26 - Maximum Rate Correction (pRateCorrectionOut) Holds the maximum permitted rate correction value to be applied by the internal clock synchronization algorithm. The CC checks only the internal rate correction value against the maximum rate correction value (absolute value). Valid values are 2 to 1923 uT."]
    #[inline(always)]
    pub fn mrc(&self) -> MRC_R {
        MRC_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Maximum Offset Correction (pOffsetCorrectionOut) Holds the maximum permitted offset correction value to be applied by the internal clock synchronization algorithm (absolute value). The CC checks only the internal offset correction value against the maximum offset correction value. Valid values are 5 to 15266 uT."]
    #[inline(always)]
    #[must_use]
    pub fn moc(&mut self) -> MOC_W<0> {
        MOC_W::new(self)
    }
    #[doc = "Bits 16:26 - Maximum Rate Correction (pRateCorrectionOut) Holds the maximum permitted rate correction value to be applied by the internal clock synchronization algorithm. The CC checks only the internal rate correction value against the maximum rate correction value (absolute value). Valid values are 2 to 1923 uT."]
    #[inline(always)]
    #[must_use]
    pub fn mrc(&mut self) -> MRC_W<16> {
        MRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTU Configuration Register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtuc10](index.html) module"]
pub struct GTUC10_SPEC;
impl crate::RegisterSpec for GTUC10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtuc10::R](R) reader structure"]
impl crate::Readable for GTUC10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtuc10::W](W) writer structure"]
impl crate::Writable for GTUC10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUC10 to value 0x0002_0005"]
impl crate::Resettable for GTUC10_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0005;
}
