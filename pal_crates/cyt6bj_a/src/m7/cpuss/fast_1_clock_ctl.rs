#[doc = "Register `FAST_1_CLOCK_CTL` reader"]
pub struct R(crate::R<FAST_1_CLOCK_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAST_1_CLOCK_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAST_1_CLOCK_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAST_1_CLOCK_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAST_1_CLOCK_CTL` writer"]
pub struct W(crate::W<FAST_1_CLOCK_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAST_1_CLOCK_CTL_SPEC>;
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
impl From<crate::W<FAST_1_CLOCK_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAST_1_CLOCK_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAC_DIV` reader - Refer FAST_0_CLOCK_CTL description."]
pub type FRAC_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAC_DIV` writer - Refer FAST_0_CLOCK_CTL description."]
pub type FRAC_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FAST_1_CLOCK_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `INT_DIV` reader - Refer FAST_0_CLOCK_CTL description."]
pub type INT_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_DIV` writer - Refer FAST_0_CLOCK_CTL description."]
pub type INT_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FAST_1_CLOCK_CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 3:7 - Refer FAST_0_CLOCK_CTL description."]
    #[inline(always)]
    pub fn frac_div(&self) -> FRAC_DIV_R {
        FRAC_DIV_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Refer FAST_0_CLOCK_CTL description."]
    #[inline(always)]
    pub fn int_div(&self) -> INT_DIV_R {
        INT_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Refer FAST_0_CLOCK_CTL description."]
    #[inline(always)]
    #[must_use]
    pub fn frac_div(&mut self) -> FRAC_DIV_W<3> {
        FRAC_DIV_W::new(self)
    }
    #[doc = "Bits 8:15 - Refer FAST_0_CLOCK_CTL description."]
    #[inline(always)]
    #[must_use]
    pub fn int_div(&mut self) -> INT_DIV_W<8> {
        INT_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast 1 clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fast_1_clock_ctl](index.html) module"]
pub struct FAST_1_CLOCK_CTL_SPEC;
impl crate::RegisterSpec for FAST_1_CLOCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fast_1_clock_ctl::R](R) reader structure"]
impl crate::Readable for FAST_1_CLOCK_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fast_1_clock_ctl::W](W) writer structure"]
impl crate::Writable for FAST_1_CLOCK_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FAST_1_CLOCK_CTL to value 0"]
impl crate::Resettable for FAST_1_CLOCK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
