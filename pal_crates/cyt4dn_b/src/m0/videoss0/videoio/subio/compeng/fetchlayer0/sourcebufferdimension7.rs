#[doc = "Register `SOURCEBUFFERDIMENSION7` reader"]
pub struct R(crate::R<SOURCEBUFFERDIMENSION7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCEBUFFERDIMENSION7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCEBUFFERDIMENSION7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCEBUFFERDIMENSION7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCEBUFFERDIMENSION7` writer"]
pub struct W(crate::W<SOURCEBUFFERDIMENSION7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCEBUFFERDIMENSION7_SPEC>;
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
impl From<crate::W<SOURCEBUFFERDIMENSION7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCEBUFFERDIMENSION7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINEWIDTH7` reader - See LineWidth0."]
pub type LINEWIDTH7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINEWIDTH7` writer - See LineWidth0."]
pub type LINEWIDTH7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERDIMENSION7_SPEC, u16, u16, 14, O>;
#[doc = "Field `LINECOUNT7` reader - See LineCount0."]
pub type LINECOUNT7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINECOUNT7` writer - See LineCount0."]
pub type LINECOUNT7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERDIMENSION7_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See LineWidth0."]
    #[inline(always)]
    pub fn linewidth7(&self) -> LINEWIDTH7_R {
        LINEWIDTH7_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See LineCount0."]
    #[inline(always)]
    pub fn linecount7(&self) -> LINECOUNT7_R {
        LINECOUNT7_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See LineWidth0."]
    #[inline(always)]
    #[must_use]
    pub fn linewidth7(&mut self) -> LINEWIDTH7_W<0> {
        LINEWIDTH7_W::new(self)
    }
    #[doc = "Bits 16:29 - See LineCount0."]
    #[inline(always)]
    #[must_use]
    pub fn linecount7(&mut self) -> LINECOUNT7_W<16> {
        LINECOUNT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer dimension of layer 7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sourcebufferdimension7](index.html) module"]
pub struct SOURCEBUFFERDIMENSION7_SPEC;
impl crate::RegisterSpec for SOURCEBUFFERDIMENSION7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sourcebufferdimension7::R](R) reader structure"]
impl crate::Readable for SOURCEBUFFERDIMENSION7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sourcebufferdimension7::W](W) writer structure"]
impl crate::Writable for SOURCEBUFFERDIMENSION7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCEBUFFERDIMENSION7 to value 0x3fff_3fff"]
impl crate::Resettable for SOURCEBUFFERDIMENSION7_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff_3fff;
}
