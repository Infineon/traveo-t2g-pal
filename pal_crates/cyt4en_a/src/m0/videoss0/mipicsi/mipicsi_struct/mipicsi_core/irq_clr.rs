#[doc = "Register `IRQ_CLR` reader"]
pub struct R(crate::R<IRQ_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_CLR` writer"]
pub struct W(crate::W<IRQ_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_CLR_SPEC>;
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
impl From<crate::W<IRQ_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ_CLR` reader - IRQ status clear. Writing a '1' value to each bit clears the corresponding bit in IRQ_STATUS. A '0' value must be written after the '1'. No time requirement between the '1' and '0' write operations."]
pub type IRQ_CLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQ_CLR` writer - IRQ status clear. Writing a '1' value to each bit clears the corresponding bit in IRQ_STATUS. A '0' value must be written after the '1'. No time requirement between the '1' and '0' write operations."]
pub type IRQ_CLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQ_CLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - IRQ status clear. Writing a '1' value to each bit clears the corresponding bit in IRQ_STATUS. A '0' value must be written after the '1'. No time requirement between the '1' and '0' write operations."]
    #[inline(always)]
    pub fn irq_clr(&self) -> IRQ_CLR_R {
        IRQ_CLR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IRQ status clear. Writing a '1' value to each bit clears the corresponding bit in IRQ_STATUS. A '0' value must be written after the '1'. No time requirement between the '1' and '0' write operations."]
    #[inline(always)]
    #[must_use]
    pub fn irq_clr(&mut self) -> IRQ_CLR_W<0> {
        IRQ_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRQ_CLR is a register within the CSI-2 RX Controller Core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_clr](index.html) module"]
pub struct IRQ_CLR_SPEC;
impl crate::RegisterSpec for IRQ_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_clr::R](R) reader structure"]
impl crate::Readable for IRQ_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_clr::W](W) writer structure"]
impl crate::Writable for IRQ_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ_CLR to value 0"]
impl crate::Resettable for IRQ_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
