#[doc = "Register `DIRECTSETUP` reader"]
pub struct R(crate::R<DIRECTSETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRECTSETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRECTSETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRECTSETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIRECTSETUP` writer"]
pub struct W(crate::W<DIRECTSETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRECTSETUP_SPEC>;
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
impl From<crate::W<DIRECTSETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRECTSETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COLORDEBUG` reader - Sets the multiplexers of the color datapath directly, do not change"]
pub type COLORDEBUG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COLORDEBUG` writer - Sets the multiplexers of the color datapath directly, do not change"]
pub type COLORDEBUG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIRECTSETUP_SPEC, u16, u16, 10, O>;
#[doc = "Field `ALPHADEBUG` reader - Sets the multiplexers of the alpha datapath directly, do not change"]
pub type ALPHADEBUG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ALPHADEBUG` writer - Sets the multiplexers of the alpha datapath directly, do not change"]
pub type ALPHADEBUG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIRECTSETUP_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sets the multiplexers of the color datapath directly, do not change"]
    #[inline(always)]
    pub fn colordebug(&self) -> COLORDEBUG_R {
        COLORDEBUG_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sets the multiplexers of the alpha datapath directly, do not change"]
    #[inline(always)]
    pub fn alphadebug(&self) -> ALPHADEBUG_R {
        ALPHADEBUG_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sets the multiplexers of the color datapath directly, do not change"]
    #[inline(always)]
    #[must_use]
    pub fn colordebug(&mut self) -> COLORDEBUG_W<0> {
        COLORDEBUG_W::new(self)
    }
    #[doc = "Bits 16:25 - Sets the multiplexers of the alpha datapath directly, do not change"]
    #[inline(always)]
    #[must_use]
    pub fn alphadebug(&mut self) -> ALPHADEBUG_W<16> {
        ALPHADEBUG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direct Control of the BlitBlend Datapath multiplexers, do not change\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [directsetup](index.html) module"]
pub struct DIRECTSETUP_SPEC;
impl crate::RegisterSpec for DIRECTSETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [directsetup::R](R) reader structure"]
impl crate::Readable for DIRECTSETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [directsetup::W](W) writer structure"]
impl crate::Writable for DIRECTSETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRECTSETUP to value 0"]
impl crate::Resettable for DIRECTSETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
