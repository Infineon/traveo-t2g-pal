#[doc = "Register `RD10_CTL` reader"]
pub struct R(crate::R<RD10_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD10_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD10_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD10_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD10_CTL` writer"]
pub struct W(crate::W<RD10_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD10_CTL_SPEC>;
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
impl From<crate::W<RD10_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD10_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD10_PC` reader - Active protection context. Modifications to this field are constrained by the associated RDx_CTL.PC_MASK_0 and RDx_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
pub type RD10_PC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD10_PC` writer - Active protection context. Modifications to this field are constrained by the associated RDx_CTL.PC_MASK_0 and RDx_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
pub type RD10_PC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD10_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RD10_PC_SAVED` reader - Saved protection context. Modifications to this field are constrained by the associated RDx_CTL.PC_MASK_0 and RDx_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
pub type RD10_PC_SAVED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD10_PC_SAVED` writer - Saved protection context. Modifications to this field are constrained by the associated RDx_CTL.PC_MASK_0 and RDx_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
pub type RD10_PC_SAVED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RD10_CTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Active protection context. Modifications to this field are constrained by the associated RDx_CTL.PC_MASK_0 and RDx_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
    #[inline(always)]
    pub fn rd10_pc(&self) -> RD10_PC_R {
        RD10_PC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Saved protection context. Modifications to this field are constrained by the associated RDx_CTL.PC_MASK_0 and RDx_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
    #[inline(always)]
    pub fn rd10_pc_saved(&self) -> RD10_PC_SAVED_R {
        RD10_PC_SAVED_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active protection context. Modifications to this field are constrained by the associated RDx_CTL.PC_MASK_0 and RDx_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
    #[inline(always)]
    #[must_use]
    pub fn rd10_pc(&mut self) -> RD10_PC_W<0> {
        RD10_PC_W::new(self)
    }
    #[doc = "Bits 16:19 - Saved protection context. Modifications to this field are constrained by the associated RDx_CTL.PC_MASK_0 and RDx_CTL.PC_MASK_15_TO_1\\[\\]
fields."]
    #[inline(always)]
    #[must_use]
    pub fn rd10_pc_saved(&mut self) -> RD10_PC_SAVED_W<16> {
        RD10_PC_SAVED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory protection7context for AXI read master with ID=10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd10_ctl](index.html) module"]
pub struct RD10_CTL_SPEC;
impl crate::RegisterSpec for RD10_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd10_ctl::R](R) reader structure"]
impl crate::Readable for RD10_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd10_ctl::W](W) writer structure"]
impl crate::Writable for RD10_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RD10_CTL to value 0"]
impl crate::Resettable for RD10_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}