#[doc = "Register `COLORBLUEBLENDFUNCTION` reader"]
pub struct R(crate::R<COLORBLUEBLENDFUNCTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORBLUEBLENDFUNCTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORBLUEBLENDFUNCTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORBLUEBLENDFUNCTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORBLUEBLENDFUNCTION` writer"]
pub struct W(crate::W<COLORBLUEBLENDFUNCTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORBLUEBLENDFUNCTION_SPEC>;
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
impl From<crate::W<COLORBLUEBLENDFUNCTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORBLUEBLENDFUNCTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLENDFUNCCOLORBLUESRC` reader - Blue component source blend function"]
pub type BLENDFUNCCOLORBLUESRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLENDFUNCCOLORBLUESRC` writer - Blue component source blend function"]
pub type BLENDFUNCCOLORBLUESRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORBLUEBLENDFUNCTION_SPEC, u16, u16, 16, O>;
#[doc = "Field `BLENDFUNCCOLORBLUEDST` reader - Blue component destination blend function"]
pub type BLENDFUNCCOLORBLUEDST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLENDFUNCCOLORBLUEDST` writer - Blue component destination blend function"]
pub type BLENDFUNCCOLORBLUEDST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORBLUEBLENDFUNCTION_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Blue component source blend function"]
    #[inline(always)]
    pub fn blendfunccolorbluesrc(&self) -> BLENDFUNCCOLORBLUESRC_R {
        BLENDFUNCCOLORBLUESRC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Blue component destination blend function"]
    #[inline(always)]
    pub fn blendfunccolorbluedst(&self) -> BLENDFUNCCOLORBLUEDST_R {
        BLENDFUNCCOLORBLUEDST_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Blue component source blend function"]
    #[inline(always)]
    #[must_use]
    pub fn blendfunccolorbluesrc(&mut self) -> BLENDFUNCCOLORBLUESRC_W<0> {
        BLENDFUNCCOLORBLUESRC_W::new(self)
    }
    #[doc = "Bits 16:31 - Blue component destination blend function"]
    #[inline(always)]
    #[must_use]
    pub fn blendfunccolorbluedst(&mut self) -> BLENDFUNCCOLORBLUEDST_W<16> {
        BLENDFUNCCOLORBLUEDST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Open GL RGB blending factors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorblueblendfunction](index.html) module"]
pub struct COLORBLUEBLENDFUNCTION_SPEC;
impl crate::RegisterSpec for COLORBLUEBLENDFUNCTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorblueblendfunction::R](R) reader structure"]
impl crate::Readable for COLORBLUEBLENDFUNCTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorblueblendfunction::W](W) writer structure"]
impl crate::Writable for COLORBLUEBLENDFUNCTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORBLUEBLENDFUNCTION to value 0x0300_0300"]
impl crate::Resettable for COLORBLUEBLENDFUNCTION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_0300;
}
