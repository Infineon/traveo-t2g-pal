#[doc = "Register `LROBJECTDIMENSION` reader"]
pub struct R(crate::R<LROBJECTDIMENSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LROBJECTDIMENSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LROBJECTDIMENSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LROBJECTDIMENSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LROBJECTDIMENSION` writer"]
pub struct W(crate::W<LROBJECTDIMENSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LROBJECTDIMENSION_SPEC>;
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
impl From<crate::W<LROBJECTDIMENSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LROBJECTDIMENSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OBJECTWIDTH` reader - Object width (programmed with -1)."]
pub type OBJECTWIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OBJECTWIDTH` writer - Object width (programmed with -1)."]
pub type OBJECTWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LROBJECTDIMENSION_SPEC, u16, u16, 14, O>;
#[doc = "Field `OBJECTHEIGHT` reader - Object height (programmed with -1)."]
pub type OBJECTHEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OBJECTHEIGHT` writer - Object height (programmed with -1)."]
pub type OBJECTHEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LROBJECTDIMENSION_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Object width (programmed with -1)."]
    #[inline(always)]
    pub fn objectwidth(&self) -> OBJECTWIDTH_R {
        OBJECTWIDTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Object height (programmed with -1)."]
    #[inline(always)]
    pub fn objectheight(&self) -> OBJECTHEIGHT_R {
        OBJECTHEIGHT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Object width (programmed with -1)."]
    #[inline(always)]
    #[must_use]
    pub fn objectwidth(&mut self) -> OBJECTWIDTH_W<0> {
        OBJECTWIDTH_W::new(self)
    }
    #[doc = "Bits 16:29 - Object height (programmed with -1)."]
    #[inline(always)]
    #[must_use]
    pub fn objectheight(&mut self) -> OBJECTHEIGHT_W<16> {
        OBJECTHEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dimension of object. Only possible in LBO mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lrobjectdimension](index.html) module"]
pub struct LROBJECTDIMENSION_SPEC;
impl crate::RegisterSpec for LROBJECTDIMENSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lrobjectdimension::R](R) reader structure"]
impl crate::Readable for LROBJECTDIMENSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lrobjectdimension::W](W) writer structure"]
impl crate::Writable for LROBJECTDIMENSION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LROBJECTDIMENSION to value 0x00f0_0140"]
impl crate::Resettable for LROBJECTDIMENSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x00f0_0140;
}
