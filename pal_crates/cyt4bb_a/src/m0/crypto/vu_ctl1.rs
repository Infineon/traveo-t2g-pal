#[doc = "Register `VU_CTL1` reader"]
pub struct R(crate::R<VU_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VU_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VU_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VU_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VU_CTL1` writer"]
pub struct W(crate::W<VU_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VU_CTL1_SPEC>;
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
impl From<crate::W<VU_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VU_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR24` reader - Specifies the memory address for the vector unit operand memory region. The register-file registers provide 13-bit word offsets within this memory region. Given ADDR\\[31:8\\], VU_VTL2.MASK\\[14:8\\]
and a 13-bit word offset offset\\[14:2\\], a vector operand memory address VU_OPERAND_ADDR\\[31:0\\]
is calculated as follows: - VU_OPERAND_ADDR\\[31:15\\]
= ADDR\\[31:15\\]
- VU_OPERAND_ADDR\\[14:8\\]
= (ADDR\\[14:8\\]
&amp; MASK\\[14:8\\]) | (offset\\[14:8\\]
&amp; ~MASK\\[14:8\\]) - VU_OPERAND_ADDR\\[7:2\\]
= offset\\[7:2\\]
- VU_OPERAND_ADDR\\[1:0\\]
= 0 (always word aligned) The vector unit operand memory region uses either the IP's memory buffer or system memory. For best performance, the IP's memory buffer should be used and ADDR should be set to MEM_BUFF and MASK should specify the IP memory buffer size. If a vector operand memory address is mapped on a memory hole, read accesses return a '0' and write accesses are ignored."]
pub type ADDR24_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR24` writer - Specifies the memory address for the vector unit operand memory region. The register-file registers provide 13-bit word offsets within this memory region. Given ADDR\\[31:8\\], VU_VTL2.MASK\\[14:8\\]
and a 13-bit word offset offset\\[14:2\\], a vector operand memory address VU_OPERAND_ADDR\\[31:0\\]
is calculated as follows: - VU_OPERAND_ADDR\\[31:15\\]
= ADDR\\[31:15\\]
- VU_OPERAND_ADDR\\[14:8\\]
= (ADDR\\[14:8\\]
&amp; MASK\\[14:8\\]) | (offset\\[14:8\\]
&amp; ~MASK\\[14:8\\]) - VU_OPERAND_ADDR\\[7:2\\]
= offset\\[7:2\\]
- VU_OPERAND_ADDR\\[1:0\\]
= 0 (always word aligned) The vector unit operand memory region uses either the IP's memory buffer or system memory. For best performance, the IP's memory buffer should be used and ADDR should be set to MEM_BUFF and MASK should specify the IP memory buffer size. If a vector operand memory address is mapped on a memory hole, read accesses return a '0' and write accesses are ignored."]
pub type ADDR24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VU_CTL1_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 8:31 - Specifies the memory address for the vector unit operand memory region. The register-file registers provide 13-bit word offsets within this memory region. Given ADDR\\[31:8\\], VU_VTL2.MASK\\[14:8\\]
and a 13-bit word offset offset\\[14:2\\], a vector operand memory address VU_OPERAND_ADDR\\[31:0\\]
is calculated as follows: - VU_OPERAND_ADDR\\[31:15\\]
= ADDR\\[31:15\\]
- VU_OPERAND_ADDR\\[14:8\\]
= (ADDR\\[14:8\\]
&amp; MASK\\[14:8\\]) | (offset\\[14:8\\]
&amp; ~MASK\\[14:8\\]) - VU_OPERAND_ADDR\\[7:2\\]
= offset\\[7:2\\]
- VU_OPERAND_ADDR\\[1:0\\]
= 0 (always word aligned) The vector unit operand memory region uses either the IP's memory buffer or system memory. For best performance, the IP's memory buffer should be used and ADDR should be set to MEM_BUFF and MASK should specify the IP memory buffer size. If a vector operand memory address is mapped on a memory hole, read accesses return a '0' and write accesses are ignored."]
    #[inline(always)]
    pub fn addr24(&self) -> ADDR24_R {
        ADDR24_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Specifies the memory address for the vector unit operand memory region. The register-file registers provide 13-bit word offsets within this memory region. Given ADDR\\[31:8\\], VU_VTL2.MASK\\[14:8\\]
and a 13-bit word offset offset\\[14:2\\], a vector operand memory address VU_OPERAND_ADDR\\[31:0\\]
is calculated as follows: - VU_OPERAND_ADDR\\[31:15\\]
= ADDR\\[31:15\\]
- VU_OPERAND_ADDR\\[14:8\\]
= (ADDR\\[14:8\\]
&amp; MASK\\[14:8\\]) | (offset\\[14:8\\]
&amp; ~MASK\\[14:8\\]) - VU_OPERAND_ADDR\\[7:2\\]
= offset\\[7:2\\]
- VU_OPERAND_ADDR\\[1:0\\]
= 0 (always word aligned) The vector unit operand memory region uses either the IP's memory buffer or system memory. For best performance, the IP's memory buffer should be used and ADDR should be set to MEM_BUFF and MASK should specify the IP memory buffer size. If a vector operand memory address is mapped on a memory hole, read accesses return a '0' and write accesses are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn addr24(&mut self) -> ADDR24_W<8> {
        ADDR24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vector unit control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vu_ctl1](index.html) module"]
pub struct VU_CTL1_SPEC;
impl crate::RegisterSpec for VU_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vu_ctl1::R](R) reader structure"]
impl crate::Readable for VU_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vu_ctl1::W](W) writer structure"]
impl crate::Writable for VU_CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VU_CTL1 to value 0"]
impl crate::Resettable for VU_CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
