#[doc = "Register `GREEN1` reader"]
pub struct R(crate::R<GREEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GREEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GREEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GREEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GREEN1` writer"]
pub struct W(crate::W<GREEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GREEN1_SPEC>;
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
impl From<crate::W<GREEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GREEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A23` reader - Value for blue input. (format is signed fix-point 3.10)"]
pub type A23_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A23` writer - Value for blue input. (format is signed fix-point 3.10)"]
pub type A23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GREEN1_SPEC, u16, u16, 13, O>;
#[doc = "Field `A24` reader - Value for alpha input. (format is signed fix-point 3.10)"]
pub type A24_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A24` writer - Value for alpha input. (format is signed fix-point 3.10)"]
pub type A24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GREEN1_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Value for blue input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    pub fn a23(&self) -> A23_R {
        A23_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Value for alpha input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    pub fn a24(&self) -> A24_R {
        A24_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Value for blue input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    #[must_use]
    pub fn a23(&mut self) -> A23_W<0> {
        A23_W::new(self)
    }
    #[doc = "Bits 16:28 - Value for alpha input. (format is signed fix-point 3.10)"]
    #[inline(always)]
    #[must_use]
    pub fn a24(&mut self) -> A24_W<16> {
        A24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Matrix values for calculation of the green output value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [green1](index.html) module"]
pub struct GREEN1_SPEC;
impl crate::RegisterSpec for GREEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [green1::R](R) reader structure"]
impl crate::Readable for GREEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [green1::W](W) writer structure"]
impl crate::Writable for GREEN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GREEN1 to value 0"]
impl crate::Resettable for GREEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
