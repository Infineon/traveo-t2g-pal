#[doc = "Register `LUTSTART` reader"]
pub struct R(crate::R<LUTSTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUTSTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUTSTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUTSTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUTSTART` writer"]
pub struct W(crate::W<LUTSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUTSTART_SPEC>;
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
impl From<crate::W<LUTSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUTSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTBLUE` reader - Start value for blue or chroma (V) channel."]
pub type STARTBLUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STARTBLUE` writer - Start value for blue or chroma (V) channel."]
pub type STARTBLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUTSTART_SPEC, u16, u16, 10, O>;
#[doc = "Field `STARTGREEN` reader - Start value for green or chroma (U) channel."]
pub type STARTGREEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STARTGREEN` writer - Start value for green or chroma (U) channel."]
pub type STARTGREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LUTSTART_SPEC, u16, u16, 10, O>;
#[doc = "Field `STARTRED` reader - Start value for red or luma (Y) channel."]
pub type STARTRED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STARTRED` writer - Start value for red or luma (Y) channel."]
pub type STARTRED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUTSTART_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Start value for blue or chroma (V) channel."]
    #[inline(always)]
    pub fn startblue(&self) -> STARTBLUE_R {
        STARTBLUE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Start value for green or chroma (U) channel."]
    #[inline(always)]
    pub fn startgreen(&self) -> STARTGREEN_R {
        STARTGREEN_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Start value for red or luma (Y) channel."]
    #[inline(always)]
    pub fn startred(&self) -> STARTRED_R {
        STARTRED_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Start value for blue or chroma (V) channel."]
    #[inline(always)]
    #[must_use]
    pub fn startblue(&mut self) -> STARTBLUE_W<0> {
        STARTBLUE_W::new(self)
    }
    #[doc = "Bits 10:19 - Start value for green or chroma (U) channel."]
    #[inline(always)]
    #[must_use]
    pub fn startgreen(&mut self) -> STARTGREEN_W<10> {
        STARTGREEN_W::new(self)
    }
    #[doc = "Bits 20:29 - Start value for red or luma (Y) channel."]
    #[inline(always)]
    #[must_use]
    pub fn startred(&mut self) -> STARTRED_W<20> {
        STARTRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start values for look-up table programming.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lutstart](index.html) module"]
pub struct LUTSTART_SPEC;
impl crate::RegisterSpec for LUTSTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lutstart::R](R) reader structure"]
impl crate::Readable for LUTSTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lutstart::W](W) writer structure"]
impl crate::Writable for LUTSTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUTSTART to value 0"]
impl crate::Resettable for LUTSTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
