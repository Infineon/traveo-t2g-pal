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
pub type MOC_R = crate::FieldReader<u16, MOC_A>;
#[doc = "Maximum Offset Correction (pOffsetCorrectionOut) Holds the maximum permitted offset correction value to be applied by the internal clock synchronization algorithm (absolute value). The CC checks only the internal offset correction value against the maximum offset correction value. Valid values are 5 to 15266 uT.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum MOC_A {
    #[doc = "5: N/A"]
    MIN = 5,
    #[doc = "15266: N/A"]
    MAX = 15266,
}
impl From<MOC_A> for u16 {
    #[inline(always)]
    fn from(variant: MOC_A) -> Self {
        variant as _
    }
}
impl MOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MOC_A> {
        match self.bits {
            5 => Some(MOC_A::MIN),
            15266 => Some(MOC_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == MOC_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == MOC_A::MAX
    }
}
#[doc = "Field `MOC` writer - Maximum Offset Correction (pOffsetCorrectionOut) Holds the maximum permitted offset correction value to be applied by the internal clock synchronization algorithm (absolute value). The CC checks only the internal offset correction value against the maximum offset correction value. Valid values are 5 to 15266 uT."]
pub type MOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC10_SPEC, u16, MOC_A, 14, O>;
impl<'a, const O: u8> MOC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(MOC_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(MOC_A::MAX)
    }
}
#[doc = "Field `MRC` reader - Maximum Rate Correction (pRateCorrectionOut) Holds the maximum permitted rate correction value to be applied by the internal clock synchronization algorithm. The CC checks only the internal rate correction value against the maximum rate correction value (absolute value). Valid values are 2 to 1923 uT."]
pub type MRC_R = crate::FieldReader<u16, MRC_A>;
#[doc = "Maximum Rate Correction (pRateCorrectionOut) Holds the maximum permitted rate correction value to be applied by the internal clock synchronization algorithm. The CC checks only the internal rate correction value against the maximum rate correction value (absolute value). Valid values are 2 to 1923 uT.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum MRC_A {
    #[doc = "2: N/A"]
    MIN = 2,
    #[doc = "1923: N/A"]
    MAX = 1923,
}
impl From<MRC_A> for u16 {
    #[inline(always)]
    fn from(variant: MRC_A) -> Self {
        variant as _
    }
}
impl MRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MRC_A> {
        match self.bits {
            2 => Some(MRC_A::MIN),
            1923 => Some(MRC_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == MRC_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == MRC_A::MAX
    }
}
#[doc = "Field `MRC` writer - Maximum Rate Correction (pRateCorrectionOut) Holds the maximum permitted rate correction value to be applied by the internal clock synchronization algorithm. The CC checks only the internal rate correction value against the maximum rate correction value (absolute value). Valid values are 2 to 1923 uT."]
pub type MRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC10_SPEC, u16, MRC_A, 11, O>;
impl<'a, const O: u8> MRC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(MRC_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(MRC_A::MAX)
    }
}
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
