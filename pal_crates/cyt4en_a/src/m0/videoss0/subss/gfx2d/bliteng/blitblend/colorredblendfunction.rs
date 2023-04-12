#[doc = "Register `COLORREDBLENDFUNCTION` reader"]
pub struct R(crate::R<COLORREDBLENDFUNCTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORREDBLENDFUNCTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORREDBLENDFUNCTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORREDBLENDFUNCTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORREDBLENDFUNCTION` writer"]
pub struct W(crate::W<COLORREDBLENDFUNCTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORREDBLENDFUNCTION_SPEC>;
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
impl From<crate::W<COLORREDBLENDFUNCTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORREDBLENDFUNCTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLENDFUNCCOLORREDSRC` reader - Red component source blend function"]
pub type BLENDFUNCCOLORREDSRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLENDFUNCCOLORREDSRC` writer - Red component source blend function"]
pub type BLENDFUNCCOLORREDSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORREDBLENDFUNCTION_SPEC, u16, u16, 16, O>;
#[doc = "Field `BLENDFUNCCOLORREDDST` reader - Red component destination blend function"]
pub type BLENDFUNCCOLORREDDST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLENDFUNCCOLORREDDST` writer - Red component destination blend function"]
pub type BLENDFUNCCOLORREDDST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORREDBLENDFUNCTION_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Red component source blend function"]
    #[inline(always)]
    pub fn blendfunccolorredsrc(&self) -> BLENDFUNCCOLORREDSRC_R {
        BLENDFUNCCOLORREDSRC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Red component destination blend function"]
    #[inline(always)]
    pub fn blendfunccolorreddst(&self) -> BLENDFUNCCOLORREDDST_R {
        BLENDFUNCCOLORREDDST_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Red component source blend function"]
    #[inline(always)]
    #[must_use]
    pub fn blendfunccolorredsrc(&mut self) -> BLENDFUNCCOLORREDSRC_W<0> {
        BLENDFUNCCOLORREDSRC_W::new(self)
    }
    #[doc = "Bits 16:31 - Red component destination blend function"]
    #[inline(always)]
    #[must_use]
    pub fn blendfunccolorreddst(&mut self) -> BLENDFUNCCOLORREDDST_W<16> {
        BLENDFUNCCOLORREDDST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Open GL RGB blending factors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorredblendfunction](index.html) module"]
pub struct COLORREDBLENDFUNCTION_SPEC;
impl crate::RegisterSpec for COLORREDBLENDFUNCTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorredblendfunction::R](R) reader structure"]
impl crate::Readable for COLORREDBLENDFUNCTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorredblendfunction::W](W) writer structure"]
impl crate::Writable for COLORREDBLENDFUNCTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORREDBLENDFUNCTION to value 0x0300_0300"]
impl crate::Resettable for COLORREDBLENDFUNCTION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_0300;
}
