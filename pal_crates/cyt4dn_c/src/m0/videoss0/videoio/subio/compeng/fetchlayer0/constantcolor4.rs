#[doc = "Register `CONSTANTCOLOR4` reader"]
pub struct R(crate::R<CONSTANTCOLOR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANTCOLOR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANTCOLOR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANTCOLOR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANTCOLOR4` writer"]
pub struct W(crate::W<CONSTANTCOLOR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANTCOLOR4_SPEC>;
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
impl From<crate::W<CONSTANTCOLOR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANTCOLOR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANTALPHA4` reader - Alpha."]
pub type CONSTANTALPHA4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTALPHA4` writer - Alpha."]
pub type CONSTANTALPHA4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTBLUE4` reader - Blue and V (chroma)."]
pub type CONSTANTBLUE4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTBLUE4` writer - Blue and V (chroma)."]
pub type CONSTANTBLUE4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTGREEN4` reader - Green and U (chroma)."]
pub type CONSTANTGREEN4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTGREEN4` writer - Green and U (chroma)."]
pub type CONSTANTGREEN4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTRED4` reader - Red and Y (luma)."]
pub type CONSTANTRED4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTRED4` writer - Red and Y (luma)."]
pub type CONSTANTRED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    pub fn constantalpha4(&self) -> CONSTANTALPHA4_R {
        CONSTANTALPHA4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn constantblue4(&self) -> CONSTANTBLUE4_R {
        CONSTANTBLUE4_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    pub fn constantgreen4(&self) -> CONSTANTGREEN4_R {
        CONSTANTGREEN4_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    pub fn constantred4(&self) -> CONSTANTRED4_R {
        CONSTANTRED4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn constantalpha4(&mut self) -> CONSTANTALPHA4_W<0> {
        CONSTANTALPHA4_W::new(self)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantblue4(&mut self) -> CONSTANTBLUE4_W<8> {
        CONSTANTBLUE4_W::new(self)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantgreen4(&mut self) -> CONSTANTGREEN4_W<16> {
        CONSTANTGREEN4_W::new(self)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantred4(&mut self) -> CONSTANTRED4_W<24> {
        CONSTANTRED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Constant color for layer 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constantcolor4](index.html) module"]
pub struct CONSTANTCOLOR4_SPEC;
impl crate::RegisterSpec for CONSTANTCOLOR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constantcolor4::R](R) reader structure"]
impl crate::Readable for CONSTANTCOLOR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constantcolor4::W](W) writer structure"]
impl crate::Writable for CONSTANTCOLOR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSTANTCOLOR4 to value 0"]
impl crate::Resettable for CONSTANTCOLOR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
