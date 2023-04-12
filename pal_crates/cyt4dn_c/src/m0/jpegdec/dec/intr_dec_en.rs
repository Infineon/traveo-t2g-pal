#[doc = "Register `INTR_DEC_EN` reader"]
pub struct R(crate::R<INTR_DEC_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_DEC_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_DEC_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_DEC_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_DEC_EN` writer"]
pub struct W(crate::W<INTR_DEC_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_DEC_EN_SPEC>;
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
impl From<crate::W<INTR_DEC_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_DEC_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APPMARKER` reader - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
pub type APPMARKER_R = crate::BitReader<bool>;
#[doc = "Field `APPMARKER` writer - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
pub type APPMARKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_EN_SPEC, bool, O>;
#[doc = "Field `COMMARKER` reader - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
pub type COMMARKER_R = crate::BitReader<bool>;
#[doc = "Field `COMMARKER` writer - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
pub type COMMARKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_EN_SPEC, bool, O>;
#[doc = "Field `UNKNOWNMARKER` reader - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
pub type UNKNOWNMARKER_R = crate::BitReader<bool>;
#[doc = "Field `UNKNOWNMARKER` writer - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
pub type UNKNOWNMARKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_EN_SPEC, bool, O>;
#[doc = "Field `SIZEAVAILABLE` reader - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
pub type SIZEAVAILABLE_R = crate::BitReader<bool>;
#[doc = "Field `SIZEAVAILABLE` writer - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
pub type SIZEAVAILABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_EN_SPEC, bool, O>;
#[doc = "Field `ERRORINTERVAL` reader - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
pub type ERRORINTERVAL_R = crate::BitReader<bool>;
#[doc = "Field `ERRORINTERVAL` writer - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
pub type ERRORINTERVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_EN_SPEC, bool, O>;
#[doc = "Field `ERRORTOTALDATA` reader - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
pub type ERRORTOTALDATA_R = crate::BitReader<bool>;
#[doc = "Field `ERRORTOTALDATA` writer - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
pub type ERRORTOTALDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_EN_SPEC, bool, O>;
#[doc = "Field `CORRECTION` reader - This bit enables Correction Function."]
pub type CORRECTION_R = crate::BitReader<bool>;
#[doc = "Field `CORRECTION` writer - This bit enables Correction Function."]
pub type CORRECTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_EN_SPEC, bool, O>;
#[doc = "Field `INTMASK` reader - This field is a global mask for the propagation of INTR_DEC bits to INTR.JPEG_CORE. This field has following polarity: 0: enable propagation 1: disable propagation"]
pub type INTMASK_R = crate::BitReader<bool>;
#[doc = "Field `INTMASK` writer - This field is a global mask for the propagation of INTR_DEC bits to INTR.JPEG_CORE. This field has following polarity: 0: enable propagation 1: disable propagation"]
pub type INTMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
    #[inline(always)]
    pub fn appmarker(&self) -> APPMARKER_R {
        APPMARKER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
    #[inline(always)]
    pub fn commarker(&self) -> COMMARKER_R {
        COMMARKER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
    #[inline(always)]
    pub fn unknownmarker(&self) -> UNKNOWNMARKER_R {
        UNKNOWNMARKER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
    #[inline(always)]
    pub fn sizeavailable(&self) -> SIZEAVAILABLE_R {
        SIZEAVAILABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
    #[inline(always)]
    pub fn errorinterval(&self) -> ERRORINTERVAL_R {
        ERRORINTERVAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
    #[inline(always)]
    pub fn errortotaldata(&self) -> ERRORTOTALDATA_R {
        ERRORTOTALDATA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit enables Correction Function."]
    #[inline(always)]
    pub fn correction(&self) -> CORRECTION_R {
        CORRECTION_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 31 - This field is a global mask for the propagation of INTR_DEC bits to INTR.JPEG_CORE. This field has following polarity: 0: enable propagation 1: disable propagation"]
    #[inline(always)]
    pub fn intmask(&self) -> INTMASK_R {
        INTMASK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
    #[inline(always)]
    #[must_use]
    pub fn appmarker(&mut self) -> APPMARKER_W<0> {
        APPMARKER_W::new(self)
    }
    #[doc = "Bit 1 - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
    #[inline(always)]
    #[must_use]
    pub fn commarker(&mut self) -> COMMARKER_W<1> {
        COMMARKER_W::new(self)
    }
    #[doc = "Bit 2 - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
    #[inline(always)]
    #[must_use]
    pub fn unknownmarker(&mut self) -> UNKNOWNMARKER_W<2> {
        UNKNOWNMARKER_W::new(self)
    }
    #[doc = "Bit 3 - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
    #[inline(always)]
    #[must_use]
    pub fn sizeavailable(&mut self) -> SIZEAVAILABLE_W<3> {
        SIZEAVAILABLE_W::new(self)
    }
    #[doc = "Bit 8 - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
    #[inline(always)]
    #[must_use]
    pub fn errorinterval(&mut self) -> ERRORINTERVAL_W<8> {
        ERRORINTERVAL_W::new(self)
    }
    #[doc = "Bit 9 - Setting this bit to '1' propagates the corresponding INTR_DEC bit to INTR.JPEG_CORE."]
    #[inline(always)]
    #[must_use]
    pub fn errortotaldata(&mut self) -> ERRORTOTALDATA_W<9> {
        ERRORTOTALDATA_W::new(self)
    }
    #[doc = "Bit 10 - This bit enables Correction Function."]
    #[inline(always)]
    #[must_use]
    pub fn correction(&mut self) -> CORRECTION_W<10> {
        CORRECTION_W::new(self)
    }
    #[doc = "Bit 31 - This field is a global mask for the propagation of INTR_DEC bits to INTR.JPEG_CORE. This field has following polarity: 0: enable propagation 1: disable propagation"]
    #[inline(always)]
    #[must_use]
    pub fn intmask(&mut self) -> INTMASK_W<31> {
        INTMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_dec_en](index.html) module"]
pub struct INTR_DEC_EN_SPEC;
impl crate::RegisterSpec for INTR_DEC_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_dec_en::R](R) reader structure"]
impl crate::Readable for INTR_DEC_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_dec_en::W](W) writer structure"]
impl crate::Writable for INTR_DEC_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_DEC_EN to value 0"]
impl crate::Resettable for INTR_DEC_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
