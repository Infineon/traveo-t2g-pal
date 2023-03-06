#[doc = "Register `TYPE2_COMPARE_19_WORD_1` reader"]
pub struct R(crate::R<TYPE2_COMPARE_19_WORD_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TYPE2_COMPARE_19_WORD_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TYPE2_COMPARE_19_WORD_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TYPE2_COMPARE_19_WORD_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TYPE2_COMPARE_19_WORD_1` writer"]
pub struct W(crate::W<TYPE2_COMPARE_19_WORD_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TYPE2_COMPARE_19_WORD_1_SPEC>;
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
impl From<crate::W<TYPE2_COMPARE_19_WORD_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TYPE2_COMPARE_19_WORD_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET_VALUE` reader - 'Offset value in bytes'"]
pub type OFFSET_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET_VALUE` writer - 'Offset value in bytes'"]
pub type OFFSET_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TYPE2_COMPARE_19_WORD_1_SPEC, u8, u8, 7, O>;
#[doc = "Field `COMPARE_OFFSET` reader - 'Compare byte offset. 00 : Offset from beginning of the frame. 01 : Offset from byte after Ether Type. 10 : Offset from byte following end of IP header. 11 : Offset from byte following end of TCP/UDP header' If bit 9 of the associated compare_word1 register is set, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+2 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+3. If bit 9 of the associated compare_word1 register is clear, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+0 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+1."]
pub type COMPARE_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPARE_OFFSET` writer - 'Compare byte offset. 00 : Offset from beginning of the frame. 01 : Offset from byte after Ether Type. 10 : Offset from byte following end of IP header. 11 : Offset from byte following end of TCP/UDP header' If bit 9 of the associated compare_word1 register is set, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+2 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+3. If bit 9 of the associated compare_word1 register is clear, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+0 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+1."]
pub type COMPARE_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TYPE2_COMPARE_19_WORD_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `DISABLE_MASK` reader - 'This bit is used to control whether the compare register word_0 contains a 4-byte compare value, or a 2-byte compare value with a 2-byte mask value. 1 - 4-byte compare value 0 - 2-byte compare, 2-byte mask '"]
pub type DISABLE_MASK_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE_MASK` writer - 'This bit is used to control whether the compare register word_0 contains a 4-byte compare value, or a 2-byte compare value with a 2-byte mask value. 1 - 4-byte compare value 0 - 2-byte compare, 2-byte mask '"]
pub type DISABLE_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TYPE2_COMPARE_19_WORD_1_SPEC, bool, O>;
#[doc = "Field `RSVD_31_10` reader - N/A"]
pub type RSVD_31_10_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:6 - 'Offset value in bytes'"]
    #[inline(always)]
    pub fn offset_value(&self) -> OFFSET_VALUE_R {
        OFFSET_VALUE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - 'Compare byte offset. 00 : Offset from beginning of the frame. 01 : Offset from byte after Ether Type. 10 : Offset from byte following end of IP header. 11 : Offset from byte following end of TCP/UDP header' If bit 9 of the associated compare_word1 register is set, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+2 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+3. If bit 9 of the associated compare_word1 register is clear, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+0 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+1."]
    #[inline(always)]
    pub fn compare_offset(&self) -> COMPARE_OFFSET_R {
        COMPARE_OFFSET_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - 'This bit is used to control whether the compare register word_0 contains a 4-byte compare value, or a 2-byte compare value with a 2-byte mask value. 1 - 4-byte compare value 0 - 2-byte compare, 2-byte mask '"]
    #[inline(always)]
    pub fn disable_mask(&self) -> DISABLE_MASK_R {
        DISABLE_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - N/A"]
    #[inline(always)]
    pub fn rsvd_31_10(&self) -> RSVD_31_10_R {
        RSVD_31_10_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 'Offset value in bytes'"]
    #[inline(always)]
    #[must_use]
    pub fn offset_value(&mut self) -> OFFSET_VALUE_W<0> {
        OFFSET_VALUE_W::new(self)
    }
    #[doc = "Bits 7:8 - 'Compare byte offset. 00 : Offset from beginning of the frame. 01 : Offset from byte after Ether Type. 10 : Offset from byte following end of IP header. 11 : Offset from byte following end of TCP/UDP header' If bit 9 of the associated compare_word1 register is set, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+2 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+3. If bit 9 of the associated compare_word1 register is clear, then the byte stored in bits \\[23:16\\]
is compared against the byte in the received frame from the selected offset+0 and the byte stored in bits \\[31:24\\]
is compared against the byte in the received frame from the selected offset+1."]
    #[inline(always)]
    #[must_use]
    pub fn compare_offset(&mut self) -> COMPARE_OFFSET_W<7> {
        COMPARE_OFFSET_W::new(self)
    }
    #[doc = "Bit 9 - 'This bit is used to control whether the compare register word_0 contains a 4-byte compare value, or a 2-byte compare value with a 2-byte mask value. 1 - 4-byte compare value 0 - 2-byte compare, 2-byte mask '"]
    #[inline(always)]
    #[must_use]
    pub fn disable_mask(&mut self) -> DISABLE_MASK_W<9> {
        DISABLE_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "same as type2_compare_0_word_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type2_compare_19_word_1](index.html) module"]
pub struct TYPE2_COMPARE_19_WORD_1_SPEC;
impl crate::RegisterSpec for TYPE2_COMPARE_19_WORD_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [type2_compare_19_word_1::R](R) reader structure"]
impl crate::Readable for TYPE2_COMPARE_19_WORD_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [type2_compare_19_word_1::W](W) writer structure"]
impl crate::Writable for TYPE2_COMPARE_19_WORD_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TYPE2_COMPARE_19_WORD_1 to value 0"]
impl crate::Resettable for TYPE2_COMPARE_19_WORD_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
