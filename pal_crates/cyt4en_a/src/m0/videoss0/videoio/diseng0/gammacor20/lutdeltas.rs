#[doc = "Register `LUTDELTAS` writer"]
pub struct W(crate::W<LUTDELTAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUTDELTAS_SPEC>;
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
impl From<crate::W<LUTDELTAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUTDELTAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELTABLUE` writer - Delta value for blue or chroma (V) channel. (format is signed integer)"]
pub type DELTABLUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LUTDELTAS_SPEC, u16, u16, 10, O>;
#[doc = "Field `DELTAGREEN` writer - Delta value for green or chroma (U) channel. (format is signed integer)"]
pub type DELTAGREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LUTDELTAS_SPEC, u16, u16, 10, O>;
#[doc = "Field `DELTARED` writer - Delta value for red or luma (Y) channel. (format is signed integer)"]
pub type DELTARED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUTDELTAS_SPEC, u16, u16, 10, O>;
impl W {
    #[doc = "Bits 0:9 - Delta value for blue or chroma (V) channel. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn deltablue(&mut self) -> DELTABLUE_W<0> {
        DELTABLUE_W::new(self)
    }
    #[doc = "Bits 10:19 - Delta value for green or chroma (U) channel. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn deltagreen(&mut self) -> DELTAGREEN_W<10> {
        DELTAGREEN_W::new(self)
    }
    #[doc = "Bits 20:29 - Delta value for red or luma (Y) channel. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn deltared(&mut self) -> DELTARED_W<20> {
        DELTARED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delta values for look-up table programming.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lutdeltas](index.html) module"]
pub struct LUTDELTAS_SPEC;
impl crate::RegisterSpec for LUTDELTAS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lutdeltas::W](W) writer structure"]
impl crate::Writable for LUTDELTAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUTDELTAS to value 0"]
impl crate::Resettable for LUTDELTAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
