#[doc = "Register `TYPE2_COMPARE_1_WORD_0` reader"]
pub struct R(crate::R<TYPE2_COMPARE_1_WORD_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TYPE2_COMPARE_1_WORD_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TYPE2_COMPARE_1_WORD_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TYPE2_COMPARE_1_WORD_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TYPE2_COMPARE_1_WORD_0` writer"]
pub struct W(crate::W<TYPE2_COMPARE_1_WORD_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TYPE2_COMPARE_1_WORD_0_SPEC>;
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
impl From<crate::W<TYPE2_COMPARE_1_WORD_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TYPE2_COMPARE_1_WORD_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK_VALUE` reader - These bits can be either a 2 byte mask field or an additional 2 byte Compare Value. If bit 9 of the associated compare_word1 register is set, then the byte stored in bits \\[7:0\\]
is compared against the byte in the received frame from the selected offset+0 and the byte stored in bits \\[15:8\\]
is compared against the byte in the received frame from the selected offset+1. If bit 9 of the associated compare_word1 register is clear, these bits become a direct 2-byte mask for the 2-byte compare register in bits \\[31:16\\]."]
pub type MASK_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MASK_VALUE` writer - These bits can be either a 2 byte mask field or an additional 2 byte Compare Value. If bit 9 of the associated compare_word1 register is set, then the byte stored in bits \\[7:0\\]
is compared against the byte in the received frame from the selected offset+0 and the byte stored in bits \\[15:8\\]
is compared against the byte in the received frame from the selected offset+1. If bit 9 of the associated compare_word1 register is clear, these bits become a direct 2-byte mask for the 2-byte compare register in bits \\[31:16\\]."]
pub type MASK_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TYPE2_COMPARE_1_WORD_0_SPEC, u16, u16, 16, O>;
#[doc = "Field `COMPARE_VALUE_TYPE2` reader - 2 byte Compare Value. If bit 9 of the associated compare_word1 register is set, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+2 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+3. If bit 9 of the associated compare_word1 register is clear, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+0 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+1."]
pub type COMPARE_VALUE_TYPE2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMPARE_VALUE_TYPE2` writer - 2 byte Compare Value. If bit 9 of the associated compare_word1 register is set, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+2 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+3. If bit 9 of the associated compare_word1 register is clear, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+0 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+1."]
pub type COMPARE_VALUE_TYPE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TYPE2_COMPARE_1_WORD_0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - These bits can be either a 2 byte mask field or an additional 2 byte Compare Value. If bit 9 of the associated compare_word1 register is set, then the byte stored in bits \\[7:0\\]
is compared against the byte in the received frame from the selected offset+0 and the byte stored in bits \\[15:8\\]
is compared against the byte in the received frame from the selected offset+1. If bit 9 of the associated compare_word1 register is clear, these bits become a direct 2-byte mask for the 2-byte compare register in bits \\[31:16\\]."]
    #[inline(always)]
    pub fn mask_value(&self) -> MASK_VALUE_R {
        MASK_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 2 byte Compare Value. If bit 9 of the associated compare_word1 register is set, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+2 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+3. If bit 9 of the associated compare_word1 register is clear, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+0 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+1."]
    #[inline(always)]
    pub fn compare_value_type2(&self) -> COMPARE_VALUE_TYPE2_R {
        COMPARE_VALUE_TYPE2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - These bits can be either a 2 byte mask field or an additional 2 byte Compare Value. If bit 9 of the associated compare_word1 register is set, then the byte stored in bits \\[7:0\\]
is compared against the byte in the received frame from the selected offset+0 and the byte stored in bits \\[15:8\\]
is compared against the byte in the received frame from the selected offset+1. If bit 9 of the associated compare_word1 register is clear, these bits become a direct 2-byte mask for the 2-byte compare register in bits \\[31:16\\]."]
    #[inline(always)]
    #[must_use]
    pub fn mask_value(&mut self) -> MASK_VALUE_W<0> {
        MASK_VALUE_W::new(self)
    }
    #[doc = "Bits 16:31 - 2 byte Compare Value. If bit 9 of the associated compare_word1 register is set, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+2 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+3. If bit 9 of the associated compare_word1 register is clear, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+0 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+1."]
    #[inline(always)]
    #[must_use]
    pub fn compare_value_type2(&mut self) -> COMPARE_VALUE_TYPE2_W<16> {
        COMPARE_VALUE_TYPE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "same as type2_compare_0_word_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type2_compare_1_word_0](index.html) module"]
pub struct TYPE2_COMPARE_1_WORD_0_SPEC;
impl crate::RegisterSpec for TYPE2_COMPARE_1_WORD_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [type2_compare_1_word_0::R](R) reader structure"]
impl crate::Readable for TYPE2_COMPARE_1_WORD_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [type2_compare_1_word_0::W](W) writer structure"]
impl crate::Writable for TYPE2_COMPARE_1_WORD_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TYPE2_COMPARE_1_WORD_0 to value 0"]
impl crate::Resettable for TYPE2_COMPARE_1_WORD_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
