#[doc = "Register `GTUC11` reader"]
pub struct R(crate::R<GTUC11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUC11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUC11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUC11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUC11` writer"]
pub struct W(crate::W<GTUC11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUC11_SPEC>;
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
impl From<crate::W<GTUC11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUC11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOCC` reader - External Offset Correction Control (vExternOffsetControl) By writing to EOCC\\[1:0\\]
the external offset correction is enabled as specified below. Should be modified only outside NIT. 00, 01 = No external offset correction 10 = External offset correction value subtracted from calculated offset correction value 11 = External offset correction value added to calculated offset correction value"]
pub type EOCC_R = crate::FieldReader<u8, EOCC_A>;
#[doc = "External Offset Correction Control (vExternOffsetControl) By writing to EOCC\\[1:0\\]
the external offset correction is enabled as specified below. Should be modified only outside NIT. 00, 01 = No external offset correction 10 = External offset correction value subtracted from calculated offset correction value 11 = External offset correction value added to calculated offset correction value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EOCC_A {
    #[doc = "0: N/A"]
    NO_EXT_OFFSET_CORRECTION = 0,
    #[doc = "1: N/A"]
    NO_EXT_OFFSET_CORRECTION_MIRROR = 1,
    #[doc = "2: N/A"]
    EXT_OFFSET_CORRECTION_SUBTRACTED = 2,
    #[doc = "3: N/A"]
    EXT_OFFSET_CORRECTION_ADDED = 3,
}
impl From<EOCC_A> for u8 {
    #[inline(always)]
    fn from(variant: EOCC_A) -> Self {
        variant as _
    }
}
impl EOCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCC_A {
        match self.bits {
            0 => EOCC_A::NO_EXT_OFFSET_CORRECTION,
            1 => EOCC_A::NO_EXT_OFFSET_CORRECTION_MIRROR,
            2 => EOCC_A::EXT_OFFSET_CORRECTION_SUBTRACTED,
            3 => EOCC_A::EXT_OFFSET_CORRECTION_ADDED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EXT_OFFSET_CORRECTION`"]
    #[inline(always)]
    pub fn is_no_ext_offset_correction(&self) -> bool {
        *self == EOCC_A::NO_EXT_OFFSET_CORRECTION
    }
    #[doc = "Checks if the value of the field is `NO_EXT_OFFSET_CORRECTION_MIRROR`"]
    #[inline(always)]
    pub fn is_no_ext_offset_correction_mirror(&self) -> bool {
        *self == EOCC_A::NO_EXT_OFFSET_CORRECTION_MIRROR
    }
    #[doc = "Checks if the value of the field is `EXT_OFFSET_CORRECTION_SUBTRACTED`"]
    #[inline(always)]
    pub fn is_ext_offset_correction_subtracted(&self) -> bool {
        *self == EOCC_A::EXT_OFFSET_CORRECTION_SUBTRACTED
    }
    #[doc = "Checks if the value of the field is `EXT_OFFSET_CORRECTION_ADDED`"]
    #[inline(always)]
    pub fn is_ext_offset_correction_added(&self) -> bool {
        *self == EOCC_A::EXT_OFFSET_CORRECTION_ADDED
    }
}
#[doc = "Field `EOCC` writer - External Offset Correction Control (vExternOffsetControl) By writing to EOCC\\[1:0\\]
the external offset correction is enabled as specified below. Should be modified only outside NIT. 00, 01 = No external offset correction 10 = External offset correction value subtracted from calculated offset correction value 11 = External offset correction value added to calculated offset correction value"]
pub type EOCC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTUC11_SPEC, u8, EOCC_A, 2, O>;
impl<'a, const O: u8> EOCC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ext_offset_correction(self) -> &'a mut W {
        self.variant(EOCC_A::NO_EXT_OFFSET_CORRECTION)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ext_offset_correction_mirror(self) -> &'a mut W {
        self.variant(EOCC_A::NO_EXT_OFFSET_CORRECTION_MIRROR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ext_offset_correction_subtracted(self) -> &'a mut W {
        self.variant(EOCC_A::EXT_OFFSET_CORRECTION_SUBTRACTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ext_offset_correction_added(self) -> &'a mut W {
        self.variant(EOCC_A::EXT_OFFSET_CORRECTION_ADDED)
    }
}
#[doc = "Field `ERCC` reader - External Rate Correction Control (vExternRateControl) By writing to ERCC\\[1:0\\]
the external rate correction is enabled as specified below. Should be modified only outside NIT. 00, 01 = No external rate correction 10 = External rate correction value subtracted from calculated rate correction value 11 = External rate correction value added to calculated rate correction value"]
pub type ERCC_R = crate::FieldReader<u8, ERCC_A>;
#[doc = "External Rate Correction Control (vExternRateControl) By writing to ERCC\\[1:0\\]
the external rate correction is enabled as specified below. Should be modified only outside NIT. 00, 01 = No external rate correction 10 = External rate correction value subtracted from calculated rate correction value 11 = External rate correction value added to calculated rate correction value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERCC_A {
    #[doc = "0: N/A"]
    NO_EXT_RATE_CORRECTION = 0,
    #[doc = "1: N/A"]
    NO_EXT_RATE_CORRECTION_MIRROR = 1,
    #[doc = "2: N/A"]
    EXT_RATE_CORRECTION_SUBTRACTED = 2,
    #[doc = "3: N/A"]
    EXT_RATE_CORRECTION_ADDED = 3,
}
impl From<ERCC_A> for u8 {
    #[inline(always)]
    fn from(variant: ERCC_A) -> Self {
        variant as _
    }
}
impl ERCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERCC_A {
        match self.bits {
            0 => ERCC_A::NO_EXT_RATE_CORRECTION,
            1 => ERCC_A::NO_EXT_RATE_CORRECTION_MIRROR,
            2 => ERCC_A::EXT_RATE_CORRECTION_SUBTRACTED,
            3 => ERCC_A::EXT_RATE_CORRECTION_ADDED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EXT_RATE_CORRECTION`"]
    #[inline(always)]
    pub fn is_no_ext_rate_correction(&self) -> bool {
        *self == ERCC_A::NO_EXT_RATE_CORRECTION
    }
    #[doc = "Checks if the value of the field is `NO_EXT_RATE_CORRECTION_MIRROR`"]
    #[inline(always)]
    pub fn is_no_ext_rate_correction_mirror(&self) -> bool {
        *self == ERCC_A::NO_EXT_RATE_CORRECTION_MIRROR
    }
    #[doc = "Checks if the value of the field is `EXT_RATE_CORRECTION_SUBTRACTED`"]
    #[inline(always)]
    pub fn is_ext_rate_correction_subtracted(&self) -> bool {
        *self == ERCC_A::EXT_RATE_CORRECTION_SUBTRACTED
    }
    #[doc = "Checks if the value of the field is `EXT_RATE_CORRECTION_ADDED`"]
    #[inline(always)]
    pub fn is_ext_rate_correction_added(&self) -> bool {
        *self == ERCC_A::EXT_RATE_CORRECTION_ADDED
    }
}
#[doc = "Field `ERCC` writer - External Rate Correction Control (vExternRateControl) By writing to ERCC\\[1:0\\]
the external rate correction is enabled as specified below. Should be modified only outside NIT. 00, 01 = No external rate correction 10 = External rate correction value subtracted from calculated rate correction value 11 = External rate correction value added to calculated rate correction value"]
pub type ERCC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTUC11_SPEC, u8, ERCC_A, 2, O>;
impl<'a, const O: u8> ERCC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ext_rate_correction(self) -> &'a mut W {
        self.variant(ERCC_A::NO_EXT_RATE_CORRECTION)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ext_rate_correction_mirror(self) -> &'a mut W {
        self.variant(ERCC_A::NO_EXT_RATE_CORRECTION_MIRROR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ext_rate_correction_subtracted(self) -> &'a mut W {
        self.variant(ERCC_A::EXT_RATE_CORRECTION_SUBTRACTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ext_rate_correction_added(self) -> &'a mut W {
        self.variant(ERCC_A::EXT_RATE_CORRECTION_ADDED)
    }
}
#[doc = "Field `EOC` reader - External Offset Correction (pExternOffsetCorrection) Holds the external offset correction value in microticks to be applied by the internal clock synchronization algorithm. The value is subtracted / added from / to the calculated offset correction value. The value is applied during NIT. May be modified in DEFAULT_CONFIG or CONFIG state only. Valid values are 0 to 7 uT."]
pub type EOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EOC` writer - External Offset Correction (pExternOffsetCorrection) Holds the external offset correction value in microticks to be applied by the internal clock synchronization algorithm. The value is subtracted / added from / to the calculated offset correction value. The value is applied during NIT. May be modified in DEFAULT_CONFIG or CONFIG state only. Valid values are 0 to 7 uT."]
pub type EOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC11_SPEC, u8, u8, 3, O>;
#[doc = "Field `ERC` reader - External Rate Correction (pExternRateCorrection) Holds the external rate correction value in microticks to be applied by the internal clock synchronization algorithm. The value is subtracted / added from / to the calculated rate correction value. The value is applied during NIT. May be modified in DEFAULT_CONFIG or CONFIG state only. Valid values are 0 to 7 uT."]
pub type ERC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERC` writer - External Rate Correction (pExternRateCorrection) Holds the external rate correction value in microticks to be applied by the internal clock synchronization algorithm. The value is subtracted / added from / to the calculated rate correction value. The value is applied during NIT. May be modified in DEFAULT_CONFIG or CONFIG state only. Valid values are 0 to 7 uT."]
pub type ERC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC11_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - External Offset Correction Control (vExternOffsetControl) By writing to EOCC\\[1:0\\]
the external offset correction is enabled as specified below. Should be modified only outside NIT. 00, 01 = No external offset correction 10 = External offset correction value subtracted from calculated offset correction value 11 = External offset correction value added to calculated offset correction value"]
    #[inline(always)]
    pub fn eocc(&self) -> EOCC_R {
        EOCC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Rate Correction Control (vExternRateControl) By writing to ERCC\\[1:0\\]
the external rate correction is enabled as specified below. Should be modified only outside NIT. 00, 01 = No external rate correction 10 = External rate correction value subtracted from calculated rate correction value 11 = External rate correction value added to calculated rate correction value"]
    #[inline(always)]
    pub fn ercc(&self) -> ERCC_R {
        ERCC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:18 - External Offset Correction (pExternOffsetCorrection) Holds the external offset correction value in microticks to be applied by the internal clock synchronization algorithm. The value is subtracted / added from / to the calculated offset correction value. The value is applied during NIT. May be modified in DEFAULT_CONFIG or CONFIG state only. Valid values are 0 to 7 uT."]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - External Rate Correction (pExternRateCorrection) Holds the external rate correction value in microticks to be applied by the internal clock synchronization algorithm. The value is subtracted / added from / to the calculated rate correction value. The value is applied during NIT. May be modified in DEFAULT_CONFIG or CONFIG state only. Valid values are 0 to 7 uT."]
    #[inline(always)]
    pub fn erc(&self) -> ERC_R {
        ERC_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Offset Correction Control (vExternOffsetControl) By writing to EOCC\\[1:0\\]
the external offset correction is enabled as specified below. Should be modified only outside NIT. 00, 01 = No external offset correction 10 = External offset correction value subtracted from calculated offset correction value 11 = External offset correction value added to calculated offset correction value"]
    #[inline(always)]
    #[must_use]
    pub fn eocc(&mut self) -> EOCC_W<0> {
        EOCC_W::new(self)
    }
    #[doc = "Bits 8:9 - External Rate Correction Control (vExternRateControl) By writing to ERCC\\[1:0\\]
the external rate correction is enabled as specified below. Should be modified only outside NIT. 00, 01 = No external rate correction 10 = External rate correction value subtracted from calculated rate correction value 11 = External rate correction value added to calculated rate correction value"]
    #[inline(always)]
    #[must_use]
    pub fn ercc(&mut self) -> ERCC_W<8> {
        ERCC_W::new(self)
    }
    #[doc = "Bits 16:18 - External Offset Correction (pExternOffsetCorrection) Holds the external offset correction value in microticks to be applied by the internal clock synchronization algorithm. The value is subtracted / added from / to the calculated offset correction value. The value is applied during NIT. May be modified in DEFAULT_CONFIG or CONFIG state only. Valid values are 0 to 7 uT."]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<16> {
        EOC_W::new(self)
    }
    #[doc = "Bits 24:26 - External Rate Correction (pExternRateCorrection) Holds the external rate correction value in microticks to be applied by the internal clock synchronization algorithm. The value is subtracted / added from / to the calculated rate correction value. The value is applied during NIT. May be modified in DEFAULT_CONFIG or CONFIG state only. Valid values are 0 to 7 uT."]
    #[inline(always)]
    #[must_use]
    pub fn erc(&mut self) -> ERC_W<24> {
        ERC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTU Configuration Register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtuc11](index.html) module"]
pub struct GTUC11_SPEC;
impl crate::RegisterSpec for GTUC11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtuc11::R](R) reader structure"]
impl crate::Readable for GTUC11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtuc11::W](W) writer structure"]
impl crate::Writable for GTUC11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUC11 to value 0"]
impl crate::Resettable for GTUC11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
