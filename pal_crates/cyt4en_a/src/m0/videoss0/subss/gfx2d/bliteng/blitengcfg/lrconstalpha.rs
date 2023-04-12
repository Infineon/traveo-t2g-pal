#[doc = "Register `LRCONSTALPHA` reader"]
pub struct R(crate::R<LRCONSTALPHA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LRCONSTALPHA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LRCONSTALPHA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LRCONSTALPHA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LRCONSTALPHA` writer"]
pub struct W(crate::W<LRCONSTALPHA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LRCONSTALPHA_SPEC>;
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
impl From<crate::W<LRCONSTALPHA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LRCONSTALPHA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANTALPHA` reader - Alpha to be filled to RGBA or Alpha buffer."]
pub type CONSTANTALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTALPHA` writer - Alpha to be filled to RGBA or Alpha buffer."]
pub type CONSTANTALPHA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LRCONSTALPHA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alpha to be filled to RGBA or Alpha buffer."]
    #[inline(always)]
    pub fn constantalpha(&self) -> CONSTANTALPHA_R {
        CONSTANTALPHA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alpha to be filled to RGBA or Alpha buffer."]
    #[inline(always)]
    #[must_use]
    pub fn constantalpha(&mut self) -> CONSTANTALPHA_W<0> {
        CONSTANTALPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Constant alpha used to fill buffer w/o fetch unit. Has affect only if ConstantColorFill is set. Only possible in LBO mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lrconstalpha](index.html) module"]
pub struct LRCONSTALPHA_SPEC;
impl crate::RegisterSpec for LRCONSTALPHA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lrconstalpha::R](R) reader structure"]
impl crate::Readable for LRCONSTALPHA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lrconstalpha::W](W) writer structure"]
impl crate::Writable for LRCONSTALPHA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LRCONSTALPHA to value 0"]
impl crate::Resettable for LRCONSTALPHA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
