#[doc = "Register `HIDDEN_REG2` reader"]
pub struct R(crate::R<HIDDEN_REG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIDDEN_REG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIDDEN_REG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIDDEN_REG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIDDEN_REG2` writer"]
pub struct W(crate::W<HIDDEN_REG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIDDEN_REG2_SPEC>;
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
impl From<crate::W<HIDDEN_REG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIDDEN_REG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIDDEN2_FIELD` reader - default value is defined per description of register cbs_control."]
pub type HIDDEN2_FIELD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HIDDEN2_FIELD` writer - default value is defined per description of register cbs_control."]
pub type HIDDEN2_FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HIDDEN_REG2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - default value is defined per description of register cbs_control."]
    #[inline(always)]
    pub fn hidden2_field(&self) -> HIDDEN2_FIELD_R {
        HIDDEN2_FIELD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - default value is defined per description of register cbs_control."]
    #[inline(always)]
    #[must_use]
    pub fn hidden2_field(&mut self) -> HIDDEN2_FIELD_W<0> {
        HIDDEN2_FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hidden registers defined in edma_defs.v '`define gem_cbs_port_tx_rate_1g 12'h4e8 // 1G Port TX Rate *** HIDDEN Register ***'\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hidden_reg2](index.html) module"]
pub struct HIDDEN_REG2_SPEC;
impl crate::RegisterSpec for HIDDEN_REG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hidden_reg2::R](R) reader structure"]
impl crate::Readable for HIDDEN_REG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hidden_reg2::W](W) writer structure"]
impl crate::Writable for HIDDEN_REG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIDDEN_REG2 to value 0x0773_5940"]
impl crate::Resettable for HIDDEN_REG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0773_5940;
}
