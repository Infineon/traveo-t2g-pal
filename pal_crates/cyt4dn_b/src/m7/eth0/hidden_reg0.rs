#[doc = "Register `HIDDEN_REG0` reader"]
pub struct R(crate::R<HIDDEN_REG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIDDEN_REG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIDDEN_REG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIDDEN_REG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIDDEN_REG0` writer"]
pub struct W(crate::W<HIDDEN_REG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIDDEN_REG0_SPEC>;
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
impl From<crate::W<HIDDEN_REG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIDDEN_REG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIDDEN0_FIELD` reader - default value is defined per description of register cbs_control."]
pub type HIDDEN0_FIELD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HIDDEN0_FIELD` writer - default value is defined per description of register cbs_control."]
pub type HIDDEN0_FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HIDDEN_REG0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - default value is defined per description of register cbs_control."]
    #[inline(always)]
    pub fn hidden0_field(&self) -> HIDDEN0_FIELD_R {
        HIDDEN0_FIELD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - default value is defined per description of register cbs_control."]
    #[inline(always)]
    #[must_use]
    pub fn hidden0_field(&mut self) -> HIDDEN0_FIELD_W<0> {
        HIDDEN0_FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hidden registers defined in edma_defs.v '`define gem_cbs_port_tx_rate_10m 12'h4e0 // 10M Port TX Rate *** HIDDEN Register ***'. Default value of cbs related hidden registers (0x14E0~0x14E8) are depicted in cbs_control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hidden_reg0](index.html) module"]
pub struct HIDDEN_REG0_SPEC;
impl crate::RegisterSpec for HIDDEN_REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hidden_reg0::R](R) reader structure"]
impl crate::Readable for HIDDEN_REG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hidden_reg0::W](W) writer structure"]
impl crate::Writable for HIDDEN_REG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIDDEN_REG0 to value 0x0026_25a0"]
impl crate::Resettable for HIDDEN_REG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0026_25a0;
}
