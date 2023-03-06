#[doc = "Register `ALPHA0` reader"]
pub struct R(crate::R<ALPHA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALPHA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALPHA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALPHA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALPHA0` writer"]
pub struct W(crate::W<ALPHA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALPHA0_SPEC>;
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
impl From<crate::W<ALPHA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALPHA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A41` reader - Value for red input. (format is signed fix-point 3.10)"]
pub type A41_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A41` writer - Value for red input. (format is signed fix-point 3.10)"]
pub type A41_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALPHA0_SPEC, u16, u16, 13, O>;
#[doc = "Field `A42` reader - Value for green input. (format is signed fix-point 3.10)"]
pub type A42_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A42` writer - Value for green input. (format is signed fix-point 3.10)"]
pub type A42_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALPHA0_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Value for red input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    pub fn a41(&self) -> A41_R {
        A41_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Value for green input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    pub fn a42(&self) -> A42_R {
        A42_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Value for red input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    #[must_use]
    pub fn a41(&mut self) -> A41_W<0> {
        A41_W::new(self)
    }
    #[doc = "Bits 16:28 - Value for green input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    #[must_use]
    pub fn a42(&mut self) -> A42_W<16> {
        A42_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Matrix values for calculation of the alpha output value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alpha0](index.html) module"]
pub struct ALPHA0_SPEC;
impl crate::RegisterSpec for ALPHA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alpha0::R](R) reader structure"]
impl crate::Readable for ALPHA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alpha0::W](W) writer structure"]
impl crate::Writable for ALPHA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALPHA0 to value 0"]
impl crate::Resettable for ALPHA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
