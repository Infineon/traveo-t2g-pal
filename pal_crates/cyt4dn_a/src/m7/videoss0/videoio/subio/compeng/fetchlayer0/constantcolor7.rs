#[doc = "Register `CONSTANTCOLOR7` reader"]
pub struct R(crate::R<CONSTANTCOLOR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANTCOLOR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANTCOLOR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANTCOLOR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANTCOLOR7` writer"]
pub struct W(crate::W<CONSTANTCOLOR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANTCOLOR7_SPEC>;
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
impl From<crate::W<CONSTANTCOLOR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANTCOLOR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANTALPHA7` reader - Alpha."]
pub type CONSTANTALPHA7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTALPHA7` writer - Alpha."]
pub type CONSTANTALPHA7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR7_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTBLUE7` reader - Blue and V (chroma)."]
pub type CONSTANTBLUE7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTBLUE7` writer - Blue and V (chroma)."]
pub type CONSTANTBLUE7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR7_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTGREEN7` reader - Green and U (chroma)."]
pub type CONSTANTGREEN7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTGREEN7` writer - Green and U (chroma)."]
pub type CONSTANTGREEN7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR7_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTRED7` reader - Red and Y (luma)."]
pub type CONSTANTRED7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTRED7` writer - Red and Y (luma)."]
pub type CONSTANTRED7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR7_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    pub fn constantalpha7(&self) -> CONSTANTALPHA7_R {
        CONSTANTALPHA7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn constantblue7(&self) -> CONSTANTBLUE7_R {
        CONSTANTBLUE7_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    pub fn constantgreen7(&self) -> CONSTANTGREEN7_R {
        CONSTANTGREEN7_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    pub fn constantred7(&self) -> CONSTANTRED7_R {
        CONSTANTRED7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn constantalpha7(&mut self) -> CONSTANTALPHA7_W<0> {
        CONSTANTALPHA7_W::new(self)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantblue7(&mut self) -> CONSTANTBLUE7_W<8> {
        CONSTANTBLUE7_W::new(self)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantgreen7(&mut self) -> CONSTANTGREEN7_W<16> {
        CONSTANTGREEN7_W::new(self)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantred7(&mut self) -> CONSTANTRED7_W<24> {
        CONSTANTRED7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Constant color for layer 7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constantcolor7](index.html) module"]
pub struct CONSTANTCOLOR7_SPEC;
impl crate::RegisterSpec for CONSTANTCOLOR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constantcolor7::R](R) reader structure"]
impl crate::Readable for CONSTANTCOLOR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constantcolor7::W](W) writer structure"]
impl crate::Writable for CONSTANTCOLOR7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSTANTCOLOR7 to value 0"]
impl crate::Resettable for CONSTANTCOLOR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
