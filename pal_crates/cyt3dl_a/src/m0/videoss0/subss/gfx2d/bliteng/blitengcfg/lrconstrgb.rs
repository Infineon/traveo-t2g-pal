#[doc = "Register `LRCONSTRGB` reader"]
pub struct R(crate::R<LRCONSTRGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LRCONSTRGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LRCONSTRGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LRCONSTRGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LRCONSTRGB` writer"]
pub struct W(crate::W<LRCONSTRGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LRCONSTRGB_SPEC>;
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
impl From<crate::W<LRCONSTRGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LRCONSTRGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANTCOLORRED` reader - Constant red color to be filled into RGBA buffer."]
pub type CONSTANTCOLORRED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONSTANTCOLORRED` writer - Constant red color to be filled into RGBA buffer."]
pub type CONSTANTCOLORRED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LRCONSTRGB_SPEC, u16, u16, 10, O>;
#[doc = "Field `CONSTANTCOLORGREEN` reader - Constant green color to be filled into RGBA buffer."]
pub type CONSTANTCOLORGREEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONSTANTCOLORGREEN` writer - Constant green color to be filled into RGBA buffer."]
pub type CONSTANTCOLORGREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LRCONSTRGB_SPEC, u16, u16, 10, O>;
#[doc = "Field `CONSTANTCOLORBLUE` reader - Constant blue color to be filled into RGBA buffer."]
pub type CONSTANTCOLORBLUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONSTANTCOLORBLUE` writer - Constant blue color to be filled into RGBA buffer."]
pub type CONSTANTCOLORBLUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LRCONSTRGB_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Constant red color to be filled into RGBA buffer."]
    #[inline(always)]
    pub fn constantcolorred(&self) -> CONSTANTCOLORRED_R {
        CONSTANTCOLORRED_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Constant green color to be filled into RGBA buffer."]
    #[inline(always)]
    pub fn constantcolorgreen(&self) -> CONSTANTCOLORGREEN_R {
        CONSTANTCOLORGREEN_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Constant blue color to be filled into RGBA buffer."]
    #[inline(always)]
    pub fn constantcolorblue(&self) -> CONSTANTCOLORBLUE_R {
        CONSTANTCOLORBLUE_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Constant red color to be filled into RGBA buffer."]
    #[inline(always)]
    #[must_use]
    pub fn constantcolorred(&mut self) -> CONSTANTCOLORRED_W<0> {
        CONSTANTCOLORRED_W::new(self)
    }
    #[doc = "Bits 10:19 - Constant green color to be filled into RGBA buffer."]
    #[inline(always)]
    #[must_use]
    pub fn constantcolorgreen(&mut self) -> CONSTANTCOLORGREEN_W<10> {
        CONSTANTCOLORGREEN_W::new(self)
    }
    #[doc = "Bits 20:29 - Constant blue color to be filled into RGBA buffer."]
    #[inline(always)]
    #[must_use]
    pub fn constantcolorblue(&mut self) -> CONSTANTCOLORBLUE_W<20> {
        CONSTANTCOLORBLUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Constant color used to fill buffer w/o fetch unit. Has affect only if ConstantColorFill is set and BufferSelect==RGBA. Only possible in LBO mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lrconstrgb](index.html) module"]
pub struct LRCONSTRGB_SPEC;
impl crate::RegisterSpec for LRCONSTRGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lrconstrgb::R](R) reader structure"]
impl crate::Readable for LRCONSTRGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lrconstrgb::W](W) writer structure"]
impl crate::Writable for LRCONSTRGB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LRCONSTRGB to value 0"]
impl crate::Resettable for LRCONSTRGB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
