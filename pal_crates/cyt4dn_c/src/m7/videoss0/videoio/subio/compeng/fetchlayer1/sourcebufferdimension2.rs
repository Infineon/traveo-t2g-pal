#[doc = "Register `SOURCEBUFFERDIMENSION2` reader"]
pub struct R(crate::R<SOURCEBUFFERDIMENSION2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCEBUFFERDIMENSION2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCEBUFFERDIMENSION2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCEBUFFERDIMENSION2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCEBUFFERDIMENSION2` writer"]
pub struct W(crate::W<SOURCEBUFFERDIMENSION2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCEBUFFERDIMENSION2_SPEC>;
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
impl From<crate::W<SOURCEBUFFERDIMENSION2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCEBUFFERDIMENSION2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINEWIDTH2` reader - See LineWidth0."]
pub type LINEWIDTH2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINEWIDTH2` writer - See LineWidth0."]
pub type LINEWIDTH2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERDIMENSION2_SPEC, u16, u16, 14, O>;
#[doc = "Field `LINECOUNT2` reader - See LineCount0."]
pub type LINECOUNT2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINECOUNT2` writer - See LineCount0."]
pub type LINECOUNT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERDIMENSION2_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See LineWidth0."]
    #[inline(always)]
    pub fn linewidth2(&self) -> LINEWIDTH2_R {
        LINEWIDTH2_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See LineCount0."]
    #[inline(always)]
    pub fn linecount2(&self) -> LINECOUNT2_R {
        LINECOUNT2_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See LineWidth0."]
    #[inline(always)]
    #[must_use]
    pub fn linewidth2(&mut self) -> LINEWIDTH2_W<0> {
        LINEWIDTH2_W::new(self)
    }
    #[doc = "Bits 16:29 - See LineCount0."]
    #[inline(always)]
    #[must_use]
    pub fn linecount2(&mut self) -> LINECOUNT2_W<16> {
        LINECOUNT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer dimension of layer 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sourcebufferdimension2](index.html) module"]
pub struct SOURCEBUFFERDIMENSION2_SPEC;
impl crate::RegisterSpec for SOURCEBUFFERDIMENSION2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sourcebufferdimension2::R](R) reader structure"]
impl crate::Readable for SOURCEBUFFERDIMENSION2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sourcebufferdimension2::W](W) writer structure"]
impl crate::Writable for SOURCEBUFFERDIMENSION2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCEBUFFERDIMENSION2 to value 0x3fff_3fff"]
impl crate::Resettable for SOURCEBUFFERDIMENSION2_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff_3fff;
}
