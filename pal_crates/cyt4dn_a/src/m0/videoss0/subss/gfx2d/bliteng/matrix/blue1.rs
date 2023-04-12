#[doc = "Register `BLUE1` reader"]
pub struct R(crate::R<BLUE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLUE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLUE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLUE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLUE1` writer"]
pub struct W(crate::W<BLUE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLUE1_SPEC>;
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
impl From<crate::W<BLUE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLUE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A33` reader - Value for blue input."]
pub type A33_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A33` writer - Value for blue input."]
pub type A33_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BLUE1_SPEC, u16, u16, 13, O>;
#[doc = "Field `A34` reader - Value for alpha input."]
pub type A34_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A34` writer - Value for alpha input."]
pub type A34_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BLUE1_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Value for blue input."]
    #[inline(always)]
    pub fn a33(&self) -> A33_R {
        A33_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Value for alpha input."]
    #[inline(always)]
    pub fn a34(&self) -> A34_R {
        A34_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Value for blue input."]
    #[inline(always)]
    #[must_use]
    pub fn a33(&mut self) -> A33_W<0> {
        A33_W::new(self)
    }
    #[doc = "Bits 16:28 - Value for alpha input."]
    #[inline(always)]
    #[must_use]
    pub fn a34(&mut self) -> A34_W<16> {
        A34_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Matrix values for calculation of the blue output value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blue1](index.html) module"]
pub struct BLUE1_SPEC;
impl crate::RegisterSpec for BLUE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blue1::R](R) reader structure"]
impl crate::Readable for BLUE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blue1::W](W) writer structure"]
impl crate::Writable for BLUE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLUE1 to value 0x0400"]
impl crate::Resettable for BLUE1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
