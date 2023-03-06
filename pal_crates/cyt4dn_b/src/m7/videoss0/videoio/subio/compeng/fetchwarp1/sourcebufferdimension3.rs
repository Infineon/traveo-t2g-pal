#[doc = "Register `SOURCEBUFFERDIMENSION3` reader"]
pub struct R(crate::R<SOURCEBUFFERDIMENSION3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCEBUFFERDIMENSION3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCEBUFFERDIMENSION3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCEBUFFERDIMENSION3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCEBUFFERDIMENSION3` writer"]
pub struct W(crate::W<SOURCEBUFFERDIMENSION3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCEBUFFERDIMENSION3_SPEC>;
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
impl From<crate::W<SOURCEBUFFERDIMENSION3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCEBUFFERDIMENSION3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINEWIDTH3` reader - See LineWidth0."]
pub type LINEWIDTH3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINEWIDTH3` writer - See LineWidth0."]
pub type LINEWIDTH3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERDIMENSION3_SPEC, u16, u16, 14, O>;
#[doc = "Field `LINECOUNT3` reader - See LineCount0."]
pub type LINECOUNT3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINECOUNT3` writer - See LineCount0."]
pub type LINECOUNT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERDIMENSION3_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See LineWidth0."]
    #[inline(always)]
    pub fn linewidth3(&self) -> LINEWIDTH3_R {
        LINEWIDTH3_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See LineCount0."]
    #[inline(always)]
    pub fn linecount3(&self) -> LINECOUNT3_R {
        LINECOUNT3_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See LineWidth0."]
    #[inline(always)]
    #[must_use]
    pub fn linewidth3(&mut self) -> LINEWIDTH3_W<0> {
        LINEWIDTH3_W::new(self)
    }
    #[doc = "Bits 16:29 - See LineCount0."]
    #[inline(always)]
    #[must_use]
    pub fn linecount3(&mut self) -> LINECOUNT3_W<16> {
        LINECOUNT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer dimension of layer 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sourcebufferdimension3](index.html) module"]
pub struct SOURCEBUFFERDIMENSION3_SPEC;
impl crate::RegisterSpec for SOURCEBUFFERDIMENSION3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sourcebufferdimension3::R](R) reader structure"]
impl crate::Readable for SOURCEBUFFERDIMENSION3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sourcebufferdimension3::W](W) writer structure"]
impl crate::Writable for SOURCEBUFFERDIMENSION3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCEBUFFERDIMENSION3 to value 0x3fff_3fff"]
impl crate::Resettable for SOURCEBUFFERDIMENSION3_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff_3fff;
}
