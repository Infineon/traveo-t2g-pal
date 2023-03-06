#[doc = "Register `CONSTANTCOLOR2` reader"]
pub struct R(crate::R<CONSTANTCOLOR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANTCOLOR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANTCOLOR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANTCOLOR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANTCOLOR2` writer"]
pub struct W(crate::W<CONSTANTCOLOR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANTCOLOR2_SPEC>;
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
impl From<crate::W<CONSTANTCOLOR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANTCOLOR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANTALPHA2` reader - Alpha."]
pub type CONSTANTALPHA2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTALPHA2` writer - Alpha."]
pub type CONSTANTALPHA2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTBLUE2` reader - Blue and V (chroma)."]
pub type CONSTANTBLUE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTBLUE2` writer - Blue and V (chroma)."]
pub type CONSTANTBLUE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTGREEN2` reader - Green and U (chroma)."]
pub type CONSTANTGREEN2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTGREEN2` writer - Green and U (chroma)."]
pub type CONSTANTGREEN2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTRED2` reader - Red and Y (luma)."]
pub type CONSTANTRED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTRED2` writer - Red and Y (luma)."]
pub type CONSTANTRED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    pub fn constantalpha2(&self) -> CONSTANTALPHA2_R {
        CONSTANTALPHA2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn constantblue2(&self) -> CONSTANTBLUE2_R {
        CONSTANTBLUE2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    pub fn constantgreen2(&self) -> CONSTANTGREEN2_R {
        CONSTANTGREEN2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    pub fn constantred2(&self) -> CONSTANTRED2_R {
        CONSTANTRED2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn constantalpha2(&mut self) -> CONSTANTALPHA2_W<0> {
        CONSTANTALPHA2_W::new(self)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantblue2(&mut self) -> CONSTANTBLUE2_W<8> {
        CONSTANTBLUE2_W::new(self)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantgreen2(&mut self) -> CONSTANTGREEN2_W<16> {
        CONSTANTGREEN2_W::new(self)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantred2(&mut self) -> CONSTANTRED2_W<24> {
        CONSTANTRED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Constant color for layer 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constantcolor2](index.html) module"]
pub struct CONSTANTCOLOR2_SPEC;
impl crate::RegisterSpec for CONSTANTCOLOR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constantcolor2::R](R) reader structure"]
impl crate::Readable for CONSTANTCOLOR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constantcolor2::W](W) writer structure"]
impl crate::Writable for CONSTANTCOLOR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSTANTCOLOR2 to value 0"]
impl crate::Resettable for CONSTANTCOLOR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
