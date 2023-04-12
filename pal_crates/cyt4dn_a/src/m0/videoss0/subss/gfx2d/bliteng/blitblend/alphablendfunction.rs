#[doc = "Register `ALPHABLENDFUNCTION` reader"]
pub struct R(crate::R<ALPHABLENDFUNCTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALPHABLENDFUNCTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALPHABLENDFUNCTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALPHABLENDFUNCTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALPHABLENDFUNCTION` writer"]
pub struct W(crate::W<ALPHABLENDFUNCTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALPHABLENDFUNCTION_SPEC>;
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
impl From<crate::W<ALPHABLENDFUNCTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALPHABLENDFUNCTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLENDFUNCALPHASRC` reader - Alpha component source blend function"]
pub type BLENDFUNCALPHASRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLENDFUNCALPHASRC` writer - Alpha component source blend function"]
pub type BLENDFUNCALPHASRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALPHABLENDFUNCTION_SPEC, u16, u16, 16, O>;
#[doc = "Field `BLENDFUNCALPHADST` reader - Alpha component destination blend function"]
pub type BLENDFUNCALPHADST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLENDFUNCALPHADST` writer - Alpha component destination blend function"]
pub type BLENDFUNCALPHADST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALPHABLENDFUNCTION_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Alpha component source blend function"]
    #[inline(always)]
    pub fn blendfuncalphasrc(&self) -> BLENDFUNCALPHASRC_R {
        BLENDFUNCALPHASRC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Alpha component destination blend function"]
    #[inline(always)]
    pub fn blendfuncalphadst(&self) -> BLENDFUNCALPHADST_R {
        BLENDFUNCALPHADST_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Alpha component source blend function"]
    #[inline(always)]
    #[must_use]
    pub fn blendfuncalphasrc(&mut self) -> BLENDFUNCALPHASRC_W<0> {
        BLENDFUNCALPHASRC_W::new(self)
    }
    #[doc = "Bits 16:31 - Alpha component destination blend function"]
    #[inline(always)]
    #[must_use]
    pub fn blendfuncalphadst(&mut self) -> BLENDFUNCALPHADST_W<16> {
        BLENDFUNCALPHADST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Open GL alpha blending factors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alphablendfunction](index.html) module"]
pub struct ALPHABLENDFUNCTION_SPEC;
impl crate::RegisterSpec for ALPHABLENDFUNCTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alphablendfunction::R](R) reader structure"]
impl crate::Readable for ALPHABLENDFUNCTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alphablendfunction::W](W) writer structure"]
impl crate::Writable for ALPHABLENDFUNCTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALPHABLENDFUNCTION to value 0x0300_0300"]
impl crate::Resettable for ALPHABLENDFUNCTION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_0300;
}
