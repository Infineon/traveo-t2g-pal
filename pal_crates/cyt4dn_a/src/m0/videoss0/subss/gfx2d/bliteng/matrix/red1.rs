#[doc = "Register `RED1` reader"]
pub struct R(crate::R<RED1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RED1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RED1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RED1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RED1` writer"]
pub struct W(crate::W<RED1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RED1_SPEC>;
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
impl From<crate::W<RED1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RED1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A13` reader - Value for blue input."]
pub type A13_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A13` writer - Value for blue input."]
pub type A13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RED1_SPEC, u16, u16, 13, O>;
#[doc = "Field `A14` reader - Value for alpha input."]
pub type A14_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A14` writer - Value for alpha input."]
pub type A14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RED1_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Value for blue input."]
    #[inline(always)]
    pub fn a13(&self) -> A13_R {
        A13_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Value for alpha input."]
    #[inline(always)]
    pub fn a14(&self) -> A14_R {
        A14_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Value for blue input."]
    #[inline(always)]
    #[must_use]
    pub fn a13(&mut self) -> A13_W<0> {
        A13_W::new(self)
    }
    #[doc = "Bits 16:28 - Value for alpha input."]
    #[inline(always)]
    #[must_use]
    pub fn a14(&mut self) -> A14_W<16> {
        A14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Matrix values for calculation of the red output value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red1](index.html) module"]
pub struct RED1_SPEC;
impl crate::RegisterSpec for RED1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [red1::R](R) reader structure"]
impl crate::Readable for RED1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [red1::W](W) writer structure"]
impl crate::Writable for RED1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RED1 to value 0"]
impl crate::Resettable for RED1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
