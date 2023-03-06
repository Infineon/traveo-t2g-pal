#[doc = "Register `MDSTS1` reader"]
pub struct R(crate::R<MDSTS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDSTS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDSTS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDSTS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDSTS1` writer"]
pub struct W(crate::W<MDSTS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDSTS1_SPEC>;
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
impl From<crate::W<MDSTS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDSTS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POSITIONX` reader - the current X position of the capture stream."]
pub type POSITIONX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `POSITIONX` writer - the current X position of the capture stream."]
pub type POSITIONX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDSTS1_SPEC, u16, u16, 14, O>;
#[doc = "Field `POSITIONY` reader - the current Y position of the capture stream."]
pub type POSITIONY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `POSITIONY` writer - the current Y position of the capture stream."]
pub type POSITIONY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDSTS1_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - the current X position of the capture stream."]
    #[inline(always)]
    pub fn positionx(&self) -> POSITIONX_R {
        POSITIONX_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - the current Y position of the capture stream."]
    #[inline(always)]
    pub fn positiony(&self) -> POSITIONY_R {
        POSITIONY_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - the current X position of the capture stream."]
    #[inline(always)]
    #[must_use]
    pub fn positionx(&mut self) -> POSITIONX_W<0> {
        POSITIONX_W::new(self)
    }
    #[doc = "Bits 16:29 - the current Y position of the capture stream."]
    #[inline(always)]
    #[must_use]
    pub fn positiony(&mut self) -> POSITIONY_W<16> {
        POSITIONY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameCap mode detection status register. The current (x,y) position of the capture stream (with respect to the output).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdsts1](index.html) module"]
pub struct MDSTS1_SPEC;
impl crate::RegisterSpec for MDSTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdsts1::R](R) reader structure"]
impl crate::Readable for MDSTS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdsts1::W](W) writer structure"]
impl crate::Writable for MDSTS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDSTS1 to value 0"]
impl crate::Resettable for MDSTS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
