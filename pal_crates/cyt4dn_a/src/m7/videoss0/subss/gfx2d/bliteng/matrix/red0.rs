#[doc = "Register `RED0` reader"]
pub struct R(crate::R<RED0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RED0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RED0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RED0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RED0` writer"]
pub struct W(crate::W<RED0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RED0_SPEC>;
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
impl From<crate::W<RED0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RED0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A11` reader - Value for red input."]
pub type A11_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A11` writer - Value for red input."]
pub type A11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RED0_SPEC, u16, u16, 13, O>;
#[doc = "Field `A12` reader - Value for green input."]
pub type A12_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A12` writer - Value for green input."]
pub type A12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RED0_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Value for red input."]
    #[inline(always)]
    pub fn a11(&self) -> A11_R {
        A11_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Value for green input."]
    #[inline(always)]
    pub fn a12(&self) -> A12_R {
        A12_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Value for red input."]
    #[inline(always)]
    #[must_use]
    pub fn a11(&mut self) -> A11_W<0> {
        A11_W::new(self)
    }
    #[doc = "Bits 16:28 - Value for green input."]
    #[inline(always)]
    #[must_use]
    pub fn a12(&mut self) -> A12_W<16> {
        A12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Matrix values for calculation of the red output value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red0](index.html) module"]
pub struct RED0_SPEC;
impl crate::RegisterSpec for RED0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [red0::R](R) reader structure"]
impl crate::Readable for RED0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [red0::W](W) writer structure"]
impl crate::Writable for RED0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RED0 to value 0x0400"]
impl crate::Resettable for RED0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
