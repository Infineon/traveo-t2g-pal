#[doc = "Register `ALPHA1` reader"]
pub struct R(crate::R<ALPHA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALPHA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALPHA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALPHA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALPHA1` writer"]
pub struct W(crate::W<ALPHA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALPHA1_SPEC>;
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
impl From<crate::W<ALPHA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALPHA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A43` reader - Value for blue input."]
pub type A43_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A43` writer - Value for blue input."]
pub type A43_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALPHA1_SPEC, u16, u16, 13, O>;
#[doc = "Field `A44` reader - Value for alpha input."]
pub type A44_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A44` writer - Value for alpha input."]
pub type A44_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALPHA1_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Value for blue input."]
    #[inline(always)]
    pub fn a43(&self) -> A43_R {
        A43_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Value for alpha input."]
    #[inline(always)]
    pub fn a44(&self) -> A44_R {
        A44_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Value for blue input."]
    #[inline(always)]
    #[must_use]
    pub fn a43(&mut self) -> A43_W<0> {
        A43_W::new(self)
    }
    #[doc = "Bits 16:28 - Value for alpha input."]
    #[inline(always)]
    #[must_use]
    pub fn a44(&mut self) -> A44_W<16> {
        A44_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Matrix values for calculation of the alpha output value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alpha1](index.html) module"]
pub struct ALPHA1_SPEC;
impl crate::RegisterSpec for ALPHA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alpha1::R](R) reader structure"]
impl crate::Readable for ALPHA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alpha1::W](W) writer structure"]
impl crate::Writable for ALPHA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALPHA1 to value 0x0400_0000"]
impl crate::Resettable for ALPHA1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0000;
}
