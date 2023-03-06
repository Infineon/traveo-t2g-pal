#[doc = "Register `VU_CTL2` reader"]
pub struct R(crate::R<VU_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VU_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VU_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VU_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VU_CTL2` writer"]
pub struct W(crate::W<VU_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VU_CTL2_SPEC>;
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
impl From<crate::W<VU_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VU_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Specifies the size of the vector operand memory region. Legal values: '0b0000000': 32 KB memory region (VU_VTL1.ADDR\\[14:8\\]
ignored). '0b1000000': 16 KB memory region (VU_VTL1.ADDR\\[13:8\\]
ignored). '0b1100000': 8 KB memory region (VU_VTL1.ADDR\\[12:8\\]
ignored). '0b1110000': 4 KB memory region (VU_VTL1.ADDR\\[11:8\\]
ignored). '0b1111000': 2 KB memory region (VU_VTL1.ADDR\\[10:8\\]
ignored). '0b1111100': 1 KB memory region (VU_VTL1.ADDR\\[9:8\\]
ignored). '0b1111110': 512 B memory region (VU_VTL1.ADDR\\[8\\]
ignored). '0b1111111': 256 B memory region. Note: the default specifies a 256 B memory region."]
pub type MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK` writer - Specifies the size of the vector operand memory region. Legal values: '0b0000000': 32 KB memory region (VU_VTL1.ADDR\\[14:8\\]
ignored). '0b1000000': 16 KB memory region (VU_VTL1.ADDR\\[13:8\\]
ignored). '0b1100000': 8 KB memory region (VU_VTL1.ADDR\\[12:8\\]
ignored). '0b1110000': 4 KB memory region (VU_VTL1.ADDR\\[11:8\\]
ignored). '0b1111000': 2 KB memory region (VU_VTL1.ADDR\\[10:8\\]
ignored). '0b1111100': 1 KB memory region (VU_VTL1.ADDR\\[9:8\\]
ignored). '0b1111110': 512 B memory region (VU_VTL1.ADDR\\[8\\]
ignored). '0b1111111': 256 B memory region. Note: the default specifies a 256 B memory region."]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VU_CTL2_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 8:14 - Specifies the size of the vector operand memory region. Legal values: '0b0000000': 32 KB memory region (VU_VTL1.ADDR\\[14:8\\]
ignored). '0b1000000': 16 KB memory region (VU_VTL1.ADDR\\[13:8\\]
ignored). '0b1100000': 8 KB memory region (VU_VTL1.ADDR\\[12:8\\]
ignored). '0b1110000': 4 KB memory region (VU_VTL1.ADDR\\[11:8\\]
ignored). '0b1111000': 2 KB memory region (VU_VTL1.ADDR\\[10:8\\]
ignored). '0b1111100': 1 KB memory region (VU_VTL1.ADDR\\[9:8\\]
ignored). '0b1111110': 512 B memory region (VU_VTL1.ADDR\\[8\\]
ignored). '0b1111111': 256 B memory region. Note: the default specifies a 256 B memory region."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - Specifies the size of the vector operand memory region. Legal values: '0b0000000': 32 KB memory region (VU_VTL1.ADDR\\[14:8\\]
ignored). '0b1000000': 16 KB memory region (VU_VTL1.ADDR\\[13:8\\]
ignored). '0b1100000': 8 KB memory region (VU_VTL1.ADDR\\[12:8\\]
ignored). '0b1110000': 4 KB memory region (VU_VTL1.ADDR\\[11:8\\]
ignored). '0b1111000': 2 KB memory region (VU_VTL1.ADDR\\[10:8\\]
ignored). '0b1111100': 1 KB memory region (VU_VTL1.ADDR\\[9:8\\]
ignored). '0b1111110': 512 B memory region (VU_VTL1.ADDR\\[8\\]
ignored). '0b1111111': 256 B memory region. Note: the default specifies a 256 B memory region."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<8> {
        MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vector unit control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vu_ctl2](index.html) module"]
pub struct VU_CTL2_SPEC;
impl crate::RegisterSpec for VU_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vu_ctl2::R](R) reader structure"]
impl crate::Readable for VU_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vu_ctl2::W](W) writer structure"]
impl crate::Writable for VU_CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VU_CTL2 to value 0x7f00"]
impl crate::Resettable for VU_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f00;
}
