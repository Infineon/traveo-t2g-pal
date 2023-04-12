#[doc = "Register `ERROR_STATUS1` reader"]
pub struct R(crate::R<ERROR_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR_STATUS1` writer"]
pub struct W(crate::W<ERROR_STATUS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_STATUS1_SPEC>;
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
impl From<crate::W<ERROR_STATUS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_STATUS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA24` reader - Specifies error description information. - For BUS_ERROR: - Violating transfer, read attribute (DATA\\[0\\]). - Violating transfer, size attribute (DATA\\[5:4\\]). '0': 8-bit transfer, '1': 16 bits transfer, '2': 32-bit transfer."]
pub type DATA24_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IDX` reader - Error source: '0': INSTR_OPC_ERROR (instruction FIFO decoder error). '1': INSTR_CC_ERROR (instruction FIFO decoder, VU CC error). '2': BUS_ERROR (bus master interface AHB-Lite bus error). '3': TR_AP_DETECT_ERROR. '4': TR_RC_DETECT_ERROR. '5': INSTR_DEV_KEY_ERROR. '6'-'7': Undefined."]
pub type IDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VALID` reader - Specifies if ERROR_STATUS0 and ERROR_STATUS1 specify valid error information. No new error information is captured as long as VALID is '1'; i.e. the error information of the first detected error is NOT overwritten."]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - Specifies if ERROR_STATUS0 and ERROR_STATUS1 specify valid error information. No new error information is captured as long as VALID is '1'; i.e. the error information of the first detected error is NOT overwritten."]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERROR_STATUS1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - Specifies error description information. - For BUS_ERROR: - Violating transfer, read attribute (DATA\\[0\\]). - Violating transfer, size attribute (DATA\\[5:4\\]). '0': 8-bit transfer, '1': 16 bits transfer, '2': 32-bit transfer."]
    #[inline(always)]
    pub fn data24(&self) -> DATA24_R {
        DATA24_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:26 - Error source: '0': INSTR_OPC_ERROR (instruction FIFO decoder error). '1': INSTR_CC_ERROR (instruction FIFO decoder, VU CC error). '2': BUS_ERROR (bus master interface AHB-Lite bus error). '3': TR_AP_DETECT_ERROR. '4': TR_RC_DETECT_ERROR. '5': INSTR_DEV_KEY_ERROR. '6'-'7': Undefined."]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Specifies if ERROR_STATUS0 and ERROR_STATUS1 specify valid error information. No new error information is captured as long as VALID is '1'; i.e. the error information of the first detected error is NOT overwritten."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Specifies if ERROR_STATUS0 and ERROR_STATUS1 specify valid error information. No new error information is captured as long as VALID is '1'; i.e. the error information of the first detected error is NOT overwritten."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<31> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error status 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_status1](index.html) module"]
pub struct ERROR_STATUS1_SPEC;
impl crate::RegisterSpec for ERROR_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error_status1::R](R) reader structure"]
impl crate::Readable for ERROR_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error_status1::W](W) writer structure"]
impl crate::Writable for ERROR_STATUS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERROR_STATUS1 to value 0"]
impl crate::Resettable for ERROR_STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
