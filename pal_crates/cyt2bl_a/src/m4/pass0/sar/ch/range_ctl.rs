#[doc = "Register `RANGE_CTL` reader"]
pub struct R(crate::R<RANGE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RANGE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RANGE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RANGE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RANGE_CTL` writer"]
pub struct W(crate::W<RANGE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RANGE_CTL_SPEC>;
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
impl From<crate::W<RANGE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RANGE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RANGE_LO` reader - Range detect low threshold (Lo)"]
pub type RANGE_LO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RANGE_LO` writer - Range detect low threshold (Lo)"]
pub type RANGE_LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RANGE_CTL_SPEC, u16, u16, 16, O>;
#[doc = "Field `RANGE_HI` reader - Range detect high threshold (Hi)"]
pub type RANGE_HI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RANGE_HI` writer - Range detect high threshold (Hi)"]
pub type RANGE_HI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RANGE_CTL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Range detect low threshold (Lo)"]
    #[inline(always)]
    pub fn range_lo(&self) -> RANGE_LO_R {
        RANGE_LO_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Range detect high threshold (Hi)"]
    #[inline(always)]
    pub fn range_hi(&self) -> RANGE_HI_R {
        RANGE_HI_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Range detect low threshold (Lo)"]
    #[inline(always)]
    #[must_use]
    pub fn range_lo(&mut self) -> RANGE_LO_W<0> {
        RANGE_LO_W::new(self)
    }
    #[doc = "Bits 16:31 - Range detect high threshold (Hi)"]
    #[inline(always)]
    #[must_use]
    pub fn range_hi(&mut self) -> RANGE_HI_W<16> {
        RANGE_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Range thresholds\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [range_ctl](index.html) module"]
pub struct RANGE_CTL_SPEC;
impl crate::RegisterSpec for RANGE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [range_ctl::R](R) reader structure"]
impl crate::Readable for RANGE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [range_ctl::W](W) writer structure"]
impl crate::Writable for RANGE_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RANGE_CTL to value 0"]
impl crate::Resettable for RANGE_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
