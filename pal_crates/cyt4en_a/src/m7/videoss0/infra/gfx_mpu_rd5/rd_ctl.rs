#[doc = "Register `RD_CTL` reader"]
pub struct R(crate::R<RD_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_CTL` writer"]
pub struct W(crate::W<RD_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_CTL_SPEC>;
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
impl From<crate::W<RD_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC` reader - Active protection context. Modifications to this field are constrained by the associated RDx_CTL.PC_MASK_0 and RDx_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
pub type PC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC` writer - Active protection context. Modifications to this field are constrained by the associated RDx_CTL.PC_MASK_0 and RDx_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
pub type PC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD_CTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Active protection context. Modifications to this field are constrained by the associated RDx_CTL.PC_MASK_0 and RDx_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active protection context. Modifications to this field are constrained by the associated RDx_CTL.PC_MASK_0 and RDx_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PC_W<0> {
        PC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory protection context for AXI read requests.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_ctl](index.html) module"]
pub struct RD_CTL_SPEC;
impl crate::RegisterSpec for RD_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_ctl::R](R) reader structure"]
impl crate::Readable for RD_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_ctl::W](W) writer structure"]
impl crate::Writable for RD_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RD_CTL to value 0x07"]
impl crate::Resettable for RD_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
