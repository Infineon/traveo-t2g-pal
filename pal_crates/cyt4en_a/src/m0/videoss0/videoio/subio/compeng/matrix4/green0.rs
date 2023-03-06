#[doc = "Register `GREEN0` reader"]
pub struct R(crate::R<GREEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GREEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GREEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GREEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GREEN0` writer"]
pub struct W(crate::W<GREEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GREEN0_SPEC>;
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
impl From<crate::W<GREEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GREEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A21` reader - Value for red input. (format is signed fix-point 3.10)"]
pub type A21_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A21` writer - Value for red input. (format is signed fix-point 3.10)"]
pub type A21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GREEN0_SPEC, u16, u16, 13, O>;
#[doc = "Field `A22` reader - Value for green input. (format is signed fix-point 3.10)"]
pub type A22_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A22` writer - Value for green input. (format is signed fix-point 3.10)"]
pub type A22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GREEN0_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Value for red input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    pub fn a21(&self) -> A21_R {
        A21_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Value for green input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    pub fn a22(&self) -> A22_R {
        A22_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Value for red input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    #[must_use]
    pub fn a21(&mut self) -> A21_W<0> {
        A21_W::new(self)
    }
    #[doc = "Bits 16:28 - Value for green input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    #[must_use]
    pub fn a22(&mut self) -> A22_W<16> {
        A22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Matrix values for calculation of the green output value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [green0](index.html) module"]
pub struct GREEN0_SPEC;
impl crate::RegisterSpec for GREEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [green0::R](R) reader structure"]
impl crate::Readable for GREEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [green0::W](W) writer structure"]
impl crate::Writable for GREEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GREEN0 to value 0x0400_0000"]
impl crate::Resettable for GREEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0000;
}
