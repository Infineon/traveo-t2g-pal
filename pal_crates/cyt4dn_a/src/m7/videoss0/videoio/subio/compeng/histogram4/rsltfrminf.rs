#[doc = "Register `RSLTFRMINF` reader"]
pub struct R(crate::R<RSLTFRMINF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSLTFRMINF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSLTFRMINF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSLTFRMINF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRM_IDX` reader - Each frame passing the histogram unit gets an index, which is the value of a wrapped around counter, which counts the frames independent from how the histogram unit is configured. The index of the frame of the ongoing readout can be read here. In display stream context, if the display operation ends (framegen is switched off and sequence complete occurs) this frame index is reset to 0."]
pub type FRM_IDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLD_POLARITY` reader - Returns field polarity of the measured frame. This read value of this field is undefined in display stream context; use in storage, content or safety stream context only."]
pub type FLD_POLARITY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - Each frame passing the histogram unit gets an index, which is the value of a wrapped around counter, which counts the frames independent from how the histogram unit is configured. The index of the frame of the ongoing readout can be read here. In display stream context, if the display operation ends (framegen is switched off and sequence complete occurs) this frame index is reset to 0."]
    #[inline(always)]
    pub fn frm_idx(&self) -> FRM_IDX_R {
        FRM_IDX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Returns field polarity of the measured frame. This read value of this field is undefined in display stream context; use in storage, content or safety stream context only."]
    #[inline(always)]
    pub fn fld_polarity(&self) -> FLD_POLARITY_R {
        FLD_POLARITY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Additional information on the measurement results.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsltfrminf](index.html) module"]
pub struct RSLTFRMINF_SPEC;
impl crate::RegisterSpec for RSLTFRMINF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsltfrminf::R](R) reader structure"]
impl crate::Readable for RSLTFRMINF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSLTFRMINF to value 0"]
impl crate::Resettable for RSLTFRMINF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
