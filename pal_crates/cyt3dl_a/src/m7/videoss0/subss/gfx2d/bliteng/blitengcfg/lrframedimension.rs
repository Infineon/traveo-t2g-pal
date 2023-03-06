#[doc = "Register `LRFRAMEDIMENSION` reader"]
pub struct R(crate::R<LRFRAMEDIMENSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LRFRAMEDIMENSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LRFRAMEDIMENSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LRFRAMEDIMENSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LRFRAMEDIMENSION` writer"]
pub struct W(crate::W<LRFRAMEDIMENSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LRFRAMEDIMENSION_SPEC>;
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
impl From<crate::W<LRFRAMEDIMENSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LRFRAMEDIMENSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOTALWIDTH` reader - Frame width (programmed with -1)."]
pub type TOTALWIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOTALWIDTH` writer - Frame width (programmed with -1)."]
pub type TOTALWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LRFRAMEDIMENSION_SPEC, u16, u16, 14, O>;
#[doc = "Field `TOTALHEIGHT` reader - Frame height (programmed with -1)."]
pub type TOTALHEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOTALHEIGHT` writer - Frame height (programmed with -1)."]
pub type TOTALHEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LRFRAMEDIMENSION_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Frame width (programmed with -1)."]
    #[inline(always)]
    pub fn totalwidth(&self) -> TOTALWIDTH_R {
        TOTALWIDTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Frame height (programmed with -1)."]
    #[inline(always)]
    pub fn totalheight(&self) -> TOTALHEIGHT_R {
        TOTALHEIGHT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame width (programmed with -1)."]
    #[inline(always)]
    #[must_use]
    pub fn totalwidth(&mut self) -> TOTALWIDTH_W<0> {
        TOTALWIDTH_W::new(self)
    }
    #[doc = "Bits 16:29 - Frame height (programmed with -1)."]
    #[inline(always)]
    #[must_use]
    pub fn totalheight(&mut self) -> TOTALHEIGHT_W<16> {
        TOTALHEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dimension of final frame in memory.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lrframedimension](index.html) module"]
pub struct LRFRAMEDIMENSION_SPEC;
impl crate::RegisterSpec for LRFRAMEDIMENSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lrframedimension::R](R) reader structure"]
impl crate::Readable for LRFRAMEDIMENSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lrframedimension::W](W) writer structure"]
impl crate::Writable for LRFRAMEDIMENSION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LRFRAMEDIMENSION to value 0x00f0_0140"]
impl crate::Resettable for LRFRAMEDIMENSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x00f0_0140;
}
