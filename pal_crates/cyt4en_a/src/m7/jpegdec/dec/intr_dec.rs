#[doc = "Register `INTR_DEC` reader"]
pub struct R(crate::R<INTR_DEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_DEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_DEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_DEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_DEC` writer"]
pub struct W(crate::W<INTR_DEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_DEC_SPEC>;
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
impl From<crate::W<INTR_DEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_DEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APPMARKER` reader - HW sets this field to '1' when an APP marker is detected in the JPEG image."]
pub type APPMARKER_R = crate::BitReader<bool>;
#[doc = "Field `APPMARKER` writer - HW sets this field to '1' when an APP marker is detected in the JPEG image."]
pub type APPMARKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_SPEC, bool, O>;
#[doc = "Field `COMMARKER` reader - HW sets this field to '1' when a COM marker is detected in the JPEG image."]
pub type COMMARKER_R = crate::BitReader<bool>;
#[doc = "Field `COMMARKER` writer - HW sets this field to '1' when a COM marker is detected in the JPEG image."]
pub type COMMARKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_SPEC, bool, O>;
#[doc = "Field `UNKNOWNMARKER` reader - HW sets this field to '1' when a marker that cannot be processed by the JPEG decoder core (markers other than SOI/SOS/DQT/DHT/DRI/RSTm/EOI/SOF0 to SOFF/APPn/COM/TEM) is detected in the JPEG image."]
pub type UNKNOWNMARKER_R = crate::BitReader<bool>;
#[doc = "Field `UNKNOWNMARKER` writer - HW sets this field to '1' when a marker that cannot be processed by the JPEG decoder core (markers other than SOI/SOS/DQT/DHT/DRI/RSTm/EOI/SOF0 to SOFF/APPn/COM/TEM) is detected in the JPEG image."]
pub type UNKNOWNMARKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_SPEC, bool, O>;
#[doc = "Field `SIZEAVAILABLE` reader - HW sets this field to '1' when the JPEG decoder core has stored the x/y size and the sub-sampling mode of the JPEG image into SizeX/SizeY registers and ImageProperty register."]
pub type SIZEAVAILABLE_R = crate::BitReader<bool>;
#[doc = "Field `SIZEAVAILABLE` writer - HW sets this field to '1' when the JPEG decoder core has stored the x/y size and the sub-sampling mode of the JPEG image into SizeX/SizeY registers and ImageProperty register."]
pub type SIZEAVAILABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_SPEC, bool, O>;
#[doc = "Field `ERRORINTERVAL` reader - HW sets this field to '1' when the interval between RSTm markers is different from the value defined in the DRI marker segment."]
pub type ERRORINTERVAL_R = crate::BitReader<bool>;
#[doc = "Field `ERRORINTERVAL` writer - HW sets this field to '1' when the interval between RSTm markers is different from the value defined in the DRI marker segment."]
pub type ERRORINTERVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_SPEC, bool, O>;
#[doc = "Field `ERRORTOTALDATA` reader - HW sets this field to '1' when the number of data in Huffman coding segment from SOS to EOI is different from the value defined in the SOF marker segment."]
pub type ERRORTOTALDATA_R = crate::BitReader<bool>;
#[doc = "Field `ERRORTOTALDATA` writer - HW sets this field to '1' when the number of data in Huffman coding segment from SOS to EOI is different from the value defined in the SOF marker segment."]
pub type ERRORTOTALDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_SPEC, bool, O>;
#[doc = "Field `ERRORMARKER` reader - HW sets this field to '1' when the JPEG decoder core has stopped decoding due to a marker-related problem. Read ERRORCODE register to find what caused this bit to be set."]
pub type ERRORMARKER_R = crate::BitReader<bool>;
#[doc = "Field `ERRORMARKER` writer - HW sets this field to '1' when the JPEG decoder core has stopped decoding due to a marker-related problem. Read ERRORCODE register to find what caused this bit to be set."]
pub type ERRORMARKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_SPEC, bool, O>;
#[doc = "Field `CORRECTEDEND` reader - HW sets this field to '1' when the JPEG decoder core has finished decoding with Correction Mode taking effect. Register CORRECTIONCROPSTATUS shows more detailed status of how Correction Mode has corrected the original image."]
pub type CORRECTEDEND_R = crate::BitReader<bool>;
#[doc = "Field `CORRECTEDEND` writer - HW sets this field to '1' when the JPEG decoder core has finished decoding with Correction Mode taking effect. Register CORRECTIONCROPSTATUS shows more detailed status of how Correction Mode has corrected the original image."]
pub type CORRECTEDEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_SPEC, bool, O>;
#[doc = "Field `NORMALEND` reader - HW sets this field to '1' when the JPEG decoder core has finished normally."]
pub type NORMALEND_R = crate::BitReader<bool>;
#[doc = "Field `NORMALEND` writer - HW sets this field to '1' when the JPEG decoder core has finished normally."]
pub type NORMALEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_DEC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HW sets this field to '1' when an APP marker is detected in the JPEG image."]
    #[inline(always)]
    pub fn appmarker(&self) -> APPMARKER_R {
        APPMARKER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HW sets this field to '1' when a COM marker is detected in the JPEG image."]
    #[inline(always)]
    pub fn commarker(&self) -> COMMARKER_R {
        COMMARKER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HW sets this field to '1' when a marker that cannot be processed by the JPEG decoder core (markers other than SOI/SOS/DQT/DHT/DRI/RSTm/EOI/SOF0 to SOFF/APPn/COM/TEM) is detected in the JPEG image."]
    #[inline(always)]
    pub fn unknownmarker(&self) -> UNKNOWNMARKER_R {
        UNKNOWNMARKER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HW sets this field to '1' when the JPEG decoder core has stored the x/y size and the sub-sampling mode of the JPEG image into SizeX/SizeY registers and ImageProperty register."]
    #[inline(always)]
    pub fn sizeavailable(&self) -> SIZEAVAILABLE_R {
        SIZEAVAILABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - HW sets this field to '1' when the interval between RSTm markers is different from the value defined in the DRI marker segment."]
    #[inline(always)]
    pub fn errorinterval(&self) -> ERRORINTERVAL_R {
        ERRORINTERVAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HW sets this field to '1' when the number of data in Huffman coding segment from SOS to EOI is different from the value defined in the SOF marker segment."]
    #[inline(always)]
    pub fn errortotaldata(&self) -> ERRORTOTALDATA_R {
        ERRORTOTALDATA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HW sets this field to '1' when the JPEG decoder core has stopped decoding due to a marker-related problem. Read ERRORCODE register to find what caused this bit to be set."]
    #[inline(always)]
    pub fn errormarker(&self) -> ERRORMARKER_R {
        ERRORMARKER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 30 - HW sets this field to '1' when the JPEG decoder core has finished decoding with Correction Mode taking effect. Register CORRECTIONCROPSTATUS shows more detailed status of how Correction Mode has corrected the original image."]
    #[inline(always)]
    pub fn correctedend(&self) -> CORRECTEDEND_R {
        CORRECTEDEND_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - HW sets this field to '1' when the JPEG decoder core has finished normally."]
    #[inline(always)]
    pub fn normalend(&self) -> NORMALEND_R {
        NORMALEND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HW sets this field to '1' when an APP marker is detected in the JPEG image."]
    #[inline(always)]
    #[must_use]
    pub fn appmarker(&mut self) -> APPMARKER_W<0> {
        APPMARKER_W::new(self)
    }
    #[doc = "Bit 1 - HW sets this field to '1' when a COM marker is detected in the JPEG image."]
    #[inline(always)]
    #[must_use]
    pub fn commarker(&mut self) -> COMMARKER_W<1> {
        COMMARKER_W::new(self)
    }
    #[doc = "Bit 2 - HW sets this field to '1' when a marker that cannot be processed by the JPEG decoder core (markers other than SOI/SOS/DQT/DHT/DRI/RSTm/EOI/SOF0 to SOFF/APPn/COM/TEM) is detected in the JPEG image."]
    #[inline(always)]
    #[must_use]
    pub fn unknownmarker(&mut self) -> UNKNOWNMARKER_W<2> {
        UNKNOWNMARKER_W::new(self)
    }
    #[doc = "Bit 3 - HW sets this field to '1' when the JPEG decoder core has stored the x/y size and the sub-sampling mode of the JPEG image into SizeX/SizeY registers and ImageProperty register."]
    #[inline(always)]
    #[must_use]
    pub fn sizeavailable(&mut self) -> SIZEAVAILABLE_W<3> {
        SIZEAVAILABLE_W::new(self)
    }
    #[doc = "Bit 8 - HW sets this field to '1' when the interval between RSTm markers is different from the value defined in the DRI marker segment."]
    #[inline(always)]
    #[must_use]
    pub fn errorinterval(&mut self) -> ERRORINTERVAL_W<8> {
        ERRORINTERVAL_W::new(self)
    }
    #[doc = "Bit 9 - HW sets this field to '1' when the number of data in Huffman coding segment from SOS to EOI is different from the value defined in the SOF marker segment."]
    #[inline(always)]
    #[must_use]
    pub fn errortotaldata(&mut self) -> ERRORTOTALDATA_W<9> {
        ERRORTOTALDATA_W::new(self)
    }
    #[doc = "Bit 10 - HW sets this field to '1' when the JPEG decoder core has stopped decoding due to a marker-related problem. Read ERRORCODE register to find what caused this bit to be set."]
    #[inline(always)]
    #[must_use]
    pub fn errormarker(&mut self) -> ERRORMARKER_W<10> {
        ERRORMARKER_W::new(self)
    }
    #[doc = "Bit 30 - HW sets this field to '1' when the JPEG decoder core has finished decoding with Correction Mode taking effect. Register CORRECTIONCROPSTATUS shows more detailed status of how Correction Mode has corrected the original image."]
    #[inline(always)]
    #[must_use]
    pub fn correctedend(&mut self) -> CORRECTEDEND_W<30> {
        CORRECTEDEND_W::new(self)
    }
    #[doc = "Bit 31 - HW sets this field to '1' when the JPEG decoder core has finished normally."]
    #[inline(always)]
    #[must_use]
    pub fn normalend(&mut self) -> NORMALEND_W<31> {
        NORMALEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_dec](index.html) module"]
pub struct INTR_DEC_SPEC;
impl crate::RegisterSpec for INTR_DEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_dec::R](R) reader structure"]
impl crate::Readable for INTR_DEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_dec::W](W) writer structure"]
impl crate::Writable for INTR_DEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_DEC to value 0"]
impl crate::Resettable for INTR_DEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
