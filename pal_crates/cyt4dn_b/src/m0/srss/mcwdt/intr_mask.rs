#[doc = "Register `INTR_MASK` reader"]
pub struct R(crate::R<INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_MASK` writer"]
pub struct W(crate::W<INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_MASK_SPEC>;
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
impl From<crate::W<INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTR0_INT` reader - Interrupt Mask for sub-counter 0 for warning interrupt. The bit controls if the interrupt is forwarded to the CPU. The interrupt is blocked when the value of the bit is 0. The interrupt is forwarded if the value of the bit is 1."]
pub type CTR0_INT_R = crate::BitReader<bool>;
#[doc = "Field `CTR0_INT` writer - Interrupt Mask for sub-counter 0 for warning interrupt. The bit controls if the interrupt is forwarded to the CPU. The interrupt is blocked when the value of the bit is 0. The interrupt is forwarded if the value of the bit is 1."]
pub type CTR0_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CTR1_INT` reader - Interrupt Mask for sub-counter 1 for warning interrupt. The bit controls if the interrupt is forwarded to the CPU. The interrupt is blocked when the value of the bit is 0. The interrupt is forwarded if the value of the bit is 1."]
pub type CTR1_INT_R = crate::BitReader<bool>;
#[doc = "Field `CTR1_INT` writer - Interrupt Mask for sub-counter 1 for warning interrupt. The bit controls if the interrupt is forwarded to the CPU. The interrupt is blocked when the value of the bit is 0. The interrupt is forwarded if the value of the bit is 1."]
pub type CTR1_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CTR2_INT` reader - Interrupt Mask for sub-counter 2. The bit controls if the interrupt is forwarded to the CPU. The interrupt is blocked when the value of the bit is 0. The interrupt is forwarded if the value of the bit is 1."]
pub type CTR2_INT_R = crate::BitReader<bool>;
#[doc = "Field `CTR2_INT` writer - Interrupt Mask for sub-counter 2. The bit controls if the interrupt is forwarded to the CPU. The interrupt is blocked when the value of the bit is 0. The interrupt is forwarded if the value of the bit is 1."]
pub type CTR2_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Mask for sub-counter 0 for warning interrupt. The bit controls if the interrupt is forwarded to the CPU. The interrupt is blocked when the value of the bit is 0. The interrupt is forwarded if the value of the bit is 1."]
    #[inline(always)]
    pub fn ctr0_int(&self) -> CTR0_INT_R {
        CTR0_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask for sub-counter 1 for warning interrupt. The bit controls if the interrupt is forwarded to the CPU. The interrupt is blocked when the value of the bit is 0. The interrupt is forwarded if the value of the bit is 1."]
    #[inline(always)]
    pub fn ctr1_int(&self) -> CTR1_INT_R {
        CTR1_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask for sub-counter 2. The bit controls if the interrupt is forwarded to the CPU. The interrupt is blocked when the value of the bit is 0. The interrupt is forwarded if the value of the bit is 1."]
    #[inline(always)]
    pub fn ctr2_int(&self) -> CTR2_INT_R {
        CTR2_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask for sub-counter 0 for warning interrupt. The bit controls if the interrupt is forwarded to the CPU. The interrupt is blocked when the value of the bit is 0. The interrupt is forwarded if the value of the bit is 1."]
    #[inline(always)]
    #[must_use]
    pub fn ctr0_int(&mut self) -> CTR0_INT_W<0> {
        CTR0_INT_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Mask for sub-counter 1 for warning interrupt. The bit controls if the interrupt is forwarded to the CPU. The interrupt is blocked when the value of the bit is 0. The interrupt is forwarded if the value of the bit is 1."]
    #[inline(always)]
    #[must_use]
    pub fn ctr1_int(&mut self) -> CTR1_INT_W<1> {
        CTR1_INT_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Mask for sub-counter 2. The bit controls if the interrupt is forwarded to the CPU. The interrupt is blocked when the value of the bit is 0. The interrupt is forwarded if the value of the bit is 1."]
    #[inline(always)]
    #[must_use]
    pub fn ctr2_int(&mut self) -> CTR2_INT_W<2> {
        CTR2_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCWDT Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](index.html) module"]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_mask::R](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
