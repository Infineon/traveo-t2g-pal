#[doc = "Register `TR_RESULT` reader"]
pub struct R(crate::R<TR_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_RESULT` writer"]
pub struct W(crate::W<TR_RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_RESULT_SPEC>;
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
impl From<crate::W<TR_RESULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA32` reader - Generated true random number. HW generates the number in the least significant bit positions (TR_CTL2.SIZE) of this field. The TR_DATA_AVAILABLE interrupt cause is activated when the number is generated. Note that SW can write this field. This functionality can be used prevent information leakage."]
pub type DATA32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA32` writer - Generated true random number. HW generates the number in the least significant bit positions (TR_CTL2.SIZE) of this field. The TR_DATA_AVAILABLE interrupt cause is activated when the number is generated. Note that SW can write this field. This functionality can be used prevent information leakage."]
pub type DATA32_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_RESULT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Generated true random number. HW generates the number in the least significant bit positions (TR_CTL2.SIZE) of this field. The TR_DATA_AVAILABLE interrupt cause is activated when the number is generated. Note that SW can write this field. This functionality can be used prevent information leakage."]
    #[inline(always)]
    pub fn data32(&self) -> DATA32_R {
        DATA32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Generated true random number. HW generates the number in the least significant bit positions (TR_CTL2.SIZE) of this field. The TR_DATA_AVAILABLE interrupt cause is activated when the number is generated. Note that SW can write this field. This functionality can be used prevent information leakage."]
    #[inline(always)]
    #[must_use]
    pub fn data32(&mut self) -> DATA32_W<0> {
        DATA32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "True random result\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_result](index.html) module"]
pub struct TR_RESULT_SPEC;
impl crate::RegisterSpec for TR_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_result::R](R) reader structure"]
impl crate::Readable for TR_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_result::W](W) writer structure"]
impl crate::Writable for TR_RESULT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_RESULT to value 0"]
impl crate::Resettable for TR_RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
