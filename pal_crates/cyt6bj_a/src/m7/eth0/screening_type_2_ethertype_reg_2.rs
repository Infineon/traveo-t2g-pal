#[doc = "Register `SCREENING_TYPE_2_ETHERTYPE_REG_2` reader"]
pub struct R(crate::R<SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCREENING_TYPE_2_ETHERTYPE_REG_2` writer"]
pub struct W(crate::W<SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC>;
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
impl From<crate::W<SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPARE_VALUE` reader - 'EtherType compare value'"]
pub type COMPARE_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMPARE_VALUE` writer - 'EtherType compare value'"]
pub type COMPARE_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSVD_31_16` reader - N/A"]
pub type RSVD_31_16_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - 'EtherType compare value'"]
    #[inline(always)]
    pub fn compare_value(&self) -> COMPARE_VALUE_R {
        COMPARE_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn rsvd_31_16(&self) -> RSVD_31_16_R {
        RSVD_31_16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 'EtherType compare value'"]
    #[inline(always)]
    #[must_use]
    pub fn compare_value(&mut self) -> COMPARE_VALUE_W<0> {
        COMPARE_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethertype Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [screening_type_2_ethertype_reg_2](index.html) module"]
pub struct SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC;
impl crate::RegisterSpec for SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [screening_type_2_ethertype_reg_2::R](R) reader structure"]
impl crate::Readable for SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [screening_type_2_ethertype_reg_2::W](W) writer structure"]
impl crate::Writable for SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCREENING_TYPE_2_ETHERTYPE_REG_2 to value 0"]
impl crate::Resettable for SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
