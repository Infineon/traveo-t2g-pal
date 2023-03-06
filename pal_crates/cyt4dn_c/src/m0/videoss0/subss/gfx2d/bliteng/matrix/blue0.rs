#[doc = "Register `BLUE0` reader"]
pub struct R(crate::R<BLUE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLUE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLUE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLUE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLUE0` writer"]
pub struct W(crate::W<BLUE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLUE0_SPEC>;
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
impl From<crate::W<BLUE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLUE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A31` reader - Value for red input. (format is signed fix-point 3.10)"]
pub type A31_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A31` writer - Value for red input. (format is signed fix-point 3.10)"]
pub type A31_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BLUE0_SPEC, u16, u16, 13, O>;
#[doc = "Field `A32` reader - Value for green input. (format is signed fix-point 3.10)"]
pub type A32_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A32` writer - Value for green input. (format is signed fix-point 3.10)"]
pub type A32_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BLUE0_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Value for red input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    pub fn a31(&self) -> A31_R {
        A31_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Value for green input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    pub fn a32(&self) -> A32_R {
        A32_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Value for red input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    #[must_use]
    pub fn a31(&mut self) -> A31_W<0> {
        A31_W::new(self)
    }
    #[doc = "Bits 16:28 - Value for green input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    #[must_use]
    pub fn a32(&mut self) -> A32_W<16> {
        A32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Matrix values for calculation of the blue output value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blue0](index.html) module"]
pub struct BLUE0_SPEC;
impl crate::RegisterSpec for BLUE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blue0::R](R) reader structure"]
impl crate::Readable for BLUE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blue0::W](W) writer structure"]
impl crate::Writable for BLUE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLUE0 to value 0"]
impl crate::Resettable for BLUE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
