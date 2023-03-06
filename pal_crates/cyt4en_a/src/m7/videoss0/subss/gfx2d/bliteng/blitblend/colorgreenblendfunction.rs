#[doc = "Register `COLORGREENBLENDFUNCTION` reader"]
pub struct R(crate::R<COLORGREENBLENDFUNCTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORGREENBLENDFUNCTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORGREENBLENDFUNCTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORGREENBLENDFUNCTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORGREENBLENDFUNCTION` writer"]
pub struct W(crate::W<COLORGREENBLENDFUNCTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORGREENBLENDFUNCTION_SPEC>;
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
impl From<crate::W<COLORGREENBLENDFUNCTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORGREENBLENDFUNCTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLENDFUNCCOLORGREENSRC` reader - Green component source blend function"]
pub type BLENDFUNCCOLORGREENSRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLENDFUNCCOLORGREENSRC` writer - Green component source blend function"]
pub type BLENDFUNCCOLORGREENSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORGREENBLENDFUNCTION_SPEC, u16, u16, 16, O>;
#[doc = "Field `BLENDFUNCCOLORGREENDST` reader - Green component destination blend function"]
pub type BLENDFUNCCOLORGREENDST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLENDFUNCCOLORGREENDST` writer - Green component destination blend function"]
pub type BLENDFUNCCOLORGREENDST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORGREENBLENDFUNCTION_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Green component source blend function"]
    #[inline(always)]
    pub fn blendfunccolorgreensrc(&self) -> BLENDFUNCCOLORGREENSRC_R {
        BLENDFUNCCOLORGREENSRC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Green component destination blend function"]
    #[inline(always)]
    pub fn blendfunccolorgreendst(&self) -> BLENDFUNCCOLORGREENDST_R {
        BLENDFUNCCOLORGREENDST_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Green component source blend function"]
    #[inline(always)]
    #[must_use]
    pub fn blendfunccolorgreensrc(&mut self) -> BLENDFUNCCOLORGREENSRC_W<0> {
        BLENDFUNCCOLORGREENSRC_W::new(self)
    }
    #[doc = "Bits 16:31 - Green component destination blend function"]
    #[inline(always)]
    #[must_use]
    pub fn blendfunccolorgreendst(&mut self) -> BLENDFUNCCOLORGREENDST_W<16> {
        BLENDFUNCCOLORGREENDST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Open GL RGB blending factors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorgreenblendfunction](index.html) module"]
pub struct COLORGREENBLENDFUNCTION_SPEC;
impl crate::RegisterSpec for COLORGREENBLENDFUNCTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorgreenblendfunction::R](R) reader structure"]
impl crate::Readable for COLORGREENBLENDFUNCTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorgreenblendfunction::W](W) writer structure"]
impl crate::Writable for COLORGREENBLENDFUNCTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORGREENBLENDFUNCTION to value 0x0300_0300"]
impl crate::Resettable for COLORGREENBLENDFUNCTION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_0300;
}
