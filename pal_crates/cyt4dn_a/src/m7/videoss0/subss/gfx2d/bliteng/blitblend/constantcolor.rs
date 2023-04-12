#[doc = "Register `CONSTANTCOLOR` reader"]
pub struct R(crate::R<CONSTANTCOLOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANTCOLOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANTCOLOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANTCOLOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANTCOLOR` writer"]
pub struct W(crate::W<CONSTANTCOLOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANTCOLOR_SPEC>;
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
impl From<crate::W<CONSTANTCOLOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANTCOLOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANTALPHA` reader - Alpha."]
pub type CONSTANTALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTALPHA` writer - Alpha."]
pub type CONSTANTALPHA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTBLUE` reader - Blue and V (chroma)."]
pub type CONSTANTBLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTBLUE` writer - Blue and V (chroma)."]
pub type CONSTANTBLUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTGREEN` reader - Green and U (chroma)."]
pub type CONSTANTGREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTGREEN` writer - Green and U (chroma)."]
pub type CONSTANTGREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CONSTANTRED` reader - Red and Y (luma)."]
pub type CONSTANTRED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTRED` writer - Red and Y (luma)."]
pub type CONSTANTRED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLOR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    pub fn constantalpha(&self) -> CONSTANTALPHA_R {
        CONSTANTALPHA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn constantblue(&self) -> CONSTANTBLUE_R {
        CONSTANTBLUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    pub fn constantgreen(&self) -> CONSTANTGREEN_R {
        CONSTANTGREEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    pub fn constantred(&self) -> CONSTANTRED_R {
        CONSTANTRED_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn constantalpha(&mut self) -> CONSTANTALPHA_W<0> {
        CONSTANTALPHA_W::new(self)
    }
    #[doc = "Bits 8:15 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantblue(&mut self) -> CONSTANTBLUE_W<8> {
        CONSTANTBLUE_W::new(self)
    }
    #[doc = "Bits 16:23 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantgreen(&mut self) -> CONSTANTGREEN_W<16> {
        CONSTANTGREEN_W::new(self)
    }
    #[doc = "Bits 24:31 - Red and Y (luma)."]
    #[inline(always)]
    #[must_use]
    pub fn constantred(&mut self) -> CONSTANTRED_W<24> {
        CONSTANTRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Constant color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constantcolor](index.html) module"]
pub struct CONSTANTCOLOR_SPEC;
impl crate::RegisterSpec for CONSTANTCOLOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constantcolor::R](R) reader structure"]
impl crate::Readable for CONSTANTCOLOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constantcolor::W](W) writer structure"]
impl crate::Writable for CONSTANTCOLOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSTANTCOLOR to value 0"]
impl crate::Resettable for CONSTANTCOLOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
