#[doc = "Register `SOURCEBUFFERDIMENSION1` reader"]
pub struct R(crate::R<SOURCEBUFFERDIMENSION1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCEBUFFERDIMENSION1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCEBUFFERDIMENSION1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCEBUFFERDIMENSION1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCEBUFFERDIMENSION1` writer"]
pub struct W(crate::W<SOURCEBUFFERDIMENSION1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCEBUFFERDIMENSION1_SPEC>;
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
impl From<crate::W<SOURCEBUFFERDIMENSION1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCEBUFFERDIMENSION1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINEWIDTH1` reader - See LineWidth0."]
pub type LINEWIDTH1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINEWIDTH1` writer - See LineWidth0."]
pub type LINEWIDTH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERDIMENSION1_SPEC, u16, u16, 14, O>;
#[doc = "Field `LINECOUNT1` reader - See LineCount0."]
pub type LINECOUNT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINECOUNT1` writer - See LineCount0."]
pub type LINECOUNT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERDIMENSION1_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See LineWidth0."]
    #[inline(always)]
    pub fn linewidth1(&self) -> LINEWIDTH1_R {
        LINEWIDTH1_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See LineCount0."]
    #[inline(always)]
    pub fn linecount1(&self) -> LINECOUNT1_R {
        LINECOUNT1_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See LineWidth0."]
    #[inline(always)]
    #[must_use]
    pub fn linewidth1(&mut self) -> LINEWIDTH1_W<0> {
        LINEWIDTH1_W::new(self)
    }
    #[doc = "Bits 16:29 - See LineCount0."]
    #[inline(always)]
    #[must_use]
    pub fn linecount1(&mut self) -> LINECOUNT1_W<16> {
        LINECOUNT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer dimensions of layer 1,\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sourcebufferdimension1](index.html) module"]
pub struct SOURCEBUFFERDIMENSION1_SPEC;
impl crate::RegisterSpec for SOURCEBUFFERDIMENSION1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sourcebufferdimension1::R](R) reader structure"]
impl crate::Readable for SOURCEBUFFERDIMENSION1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sourcebufferdimension1::W](W) writer structure"]
impl crate::Writable for SOURCEBUFFERDIMENSION1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCEBUFFERDIMENSION1 to value 0x3fff_3fff"]
impl crate::Resettable for SOURCEBUFFERDIMENSION1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff_3fff;
}
