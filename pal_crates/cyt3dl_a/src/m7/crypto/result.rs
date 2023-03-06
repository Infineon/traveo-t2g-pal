#[doc = "Register `RESULT` reader"]
pub struct R(crate::R<RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESULT` writer"]
pub struct W(crate::W<RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESULT_SPEC>;
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
impl From<crate::W<RESULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - BLOCK_CMP operation (DATA\\[0\\]): '0': source 0 equals source 1. '1': source 0 does NOT equal source 1. CRC operation (DATA\\[31:0\\]). State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage."]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - BLOCK_CMP operation (DATA\\[0\\]): '0': source 0 equals source 1. '1': source 0 does NOT equal source 1. CRC operation (DATA\\[31:0\\]). State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESULT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - BLOCK_CMP operation (DATA\\[0\\]): '0': source 0 equals source 1. '1': source 0 does NOT equal source 1. CRC operation (DATA\\[31:0\\]). State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BLOCK_CMP operation (DATA\\[0\\]): '0': source 0 equals source 1. '1': source 0 does NOT equal source 1. CRC operation (DATA\\[31:0\\]). State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Result\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](index.html) module"]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result::R](R) reader structure"]
impl crate::Readable for RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [result::W](W) writer structure"]
impl crate::Writable for RESULT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
