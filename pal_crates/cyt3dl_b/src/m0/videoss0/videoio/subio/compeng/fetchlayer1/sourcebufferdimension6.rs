#[doc = "Register `SOURCEBUFFERDIMENSION6` reader"]
pub struct R(crate::R<SOURCEBUFFERDIMENSION6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCEBUFFERDIMENSION6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCEBUFFERDIMENSION6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCEBUFFERDIMENSION6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCEBUFFERDIMENSION6` writer"]
pub struct W(crate::W<SOURCEBUFFERDIMENSION6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCEBUFFERDIMENSION6_SPEC>;
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
impl From<crate::W<SOURCEBUFFERDIMENSION6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCEBUFFERDIMENSION6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINEWIDTH6` reader - See LineWidth0."]
pub type LINEWIDTH6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINEWIDTH6` writer - See LineWidth0."]
pub type LINEWIDTH6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERDIMENSION6_SPEC, u16, u16, 14, O>;
#[doc = "Field `LINECOUNT6` reader - See LineCount0."]
pub type LINECOUNT6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINECOUNT6` writer - See LineCount0."]
pub type LINECOUNT6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERDIMENSION6_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See LineWidth0."]
    #[inline(always)]
    pub fn linewidth6(&self) -> LINEWIDTH6_R {
        LINEWIDTH6_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See LineCount0."]
    #[inline(always)]
    pub fn linecount6(&self) -> LINECOUNT6_R {
        LINECOUNT6_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See LineWidth0."]
    #[inline(always)]
    #[must_use]
    pub fn linewidth6(&mut self) -> LINEWIDTH6_W<0> {
        LINEWIDTH6_W::new(self)
    }
    #[doc = "Bits 16:29 - See LineCount0."]
    #[inline(always)]
    #[must_use]
    pub fn linecount6(&mut self) -> LINECOUNT6_W<16> {
        LINECOUNT6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer dimension of layer 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sourcebufferdimension6](index.html) module"]
pub struct SOURCEBUFFERDIMENSION6_SPEC;
impl crate::RegisterSpec for SOURCEBUFFERDIMENSION6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sourcebufferdimension6::R](R) reader structure"]
impl crate::Readable for SOURCEBUFFERDIMENSION6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sourcebufferdimension6::W](W) writer structure"]
impl crate::Writable for SOURCEBUFFERDIMENSION6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCEBUFFERDIMENSION6 to value 0x3fff_3fff"]
impl crate::Resettable for SOURCEBUFFERDIMENSION6_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff_3fff;
}
