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
#[doc = "Field `STARTBLUE` writer - Start value for blue or chroma (V) channel. (format is unsigned integer)"]
pub type STARTBLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUTSTART_SPEC, u16, u16, 10, O>;
#[doc = "Field `STARTGREEN` writer - Start value for green or chroma (U) channel. (format is unsigned integer)"]
pub type STARTGREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LUTSTART_SPEC, u16, u16, 10, O>;
#[doc = "Field `STARTRED` writer - Start value for red or luma (Y) channel. (format is unsigned integer)"]
pub type STARTRED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUTSTART_SPEC, u16, u16, 10, O>;
impl W {
    #[doc = "Bits 0:9 - Start value for blue or chroma (V) channel. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn startblue(&mut self) -> STARTBLUE_W<0> {
        STARTBLUE_W::new(self)
    }
    #[doc = "Bits 10:19 - Start value for green or chroma (U) channel. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn startgreen(&mut self) -> STARTGREEN_W<10> {
        STARTGREEN_W::new(self)
    }
    #[doc = "Bits 20:29 - Start value for red or luma (Y) channel. (format is unsigned integer)"]
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
#[doc = "Start values for look-up table programming.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lutstart](index.html) module"]
pub struct LUTSTART_SPEC;
impl crate::RegisterSpec for LUTSTART_SPEC {
    type Ux = u32;
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
