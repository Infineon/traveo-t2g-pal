#[doc = "Register `HIDDEN_REG3` reader"]
pub struct R(crate::R<HIDDEN_REG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIDDEN_REG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIDDEN_REG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIDDEN_REG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIDDEN_REG3` writer"]
pub struct W(crate::W<HIDDEN_REG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIDDEN_REG3_SPEC>;
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
impl From<crate::W<HIDDEN_REG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIDDEN_REG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIDDEN3_FIELD` reader - default value is defined per description of register cbs_control."]
pub type HIDDEN3_FIELD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIDDEN3_FIELD` writer - default value is defined per description of register cbs_control."]
pub type HIDDEN3_FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HIDDEN_REG3_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - default value is defined per description of register cbs_control."]
    #[inline(always)]
    pub fn hidden3_field(&self) -> HIDDEN3_FIELD_R {
        HIDDEN3_FIELD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - default value is defined per description of register cbs_control."]
    #[inline(always)]
    #[must_use]
    pub fn hidden3_field(&mut self) -> HIDDEN3_FIELD_W<0> {
        HIDDEN3_FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hidden registers defined in edma_defs.v '`define gem_wd_counter 12'h4ec // *** HIDDEN Register ***'.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hidden_reg3](index.html) module"]
pub struct HIDDEN_REG3_SPEC;
impl crate::RegisterSpec for HIDDEN_REG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hidden_reg3::R](R) reader structure"]
impl crate::Readable for HIDDEN_REG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hidden_reg3::W](W) writer structure"]
impl crate::Writable for HIDDEN_REG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIDDEN_REG3 to value 0x07"]
impl crate::Resettable for HIDDEN_REG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
