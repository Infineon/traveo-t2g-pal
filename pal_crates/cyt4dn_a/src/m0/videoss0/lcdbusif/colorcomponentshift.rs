#[doc = "Register `COLORCOMPONENTSHIFT` reader"]
pub struct R(crate::R<COLORCOMPONENTSHIFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTSHIFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTSHIFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTSHIFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTSHIFT` writer"]
pub struct W(crate::W<COLORCOMPONENTSHIFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTSHIFT_SPEC>;
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
impl From<crate::W<COLORCOMPONENTSHIFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTSHIFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTSHIFTBLUE` reader - Blue component shift."]
pub type COMPONENTSHIFTBLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTBLUE` writer - Blue component shift."]
pub type COMPONENTSHIFTBLUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTGREEN` reader - Green component shift."]
pub type COMPONENTSHIFTGREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTGREEN` writer - Green component shift."]
pub type COMPONENTSHIFTGREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTRED` reader - Red component shift."]
pub type COMPONENTSHIFTRED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTRED` writer - Red component shift."]
pub type COMPONENTSHIFTRED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 8:12 - Blue component shift."]
    #[inline(always)]
    pub fn componentshiftblue(&self) -> COMPONENTSHIFTBLUE_R {
        COMPONENTSHIFTBLUE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Green component shift."]
    #[inline(always)]
    pub fn componentshiftgreen(&self) -> COMPONENTSHIFTGREEN_R {
        COMPONENTSHIFTGREEN_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Red component shift."]
    #[inline(always)]
    pub fn componentshiftred(&self) -> COMPONENTSHIFTRED_R {
        COMPONENTSHIFTRED_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - Blue component shift."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftblue(&mut self) -> COMPONENTSHIFTBLUE_W<8> {
        COMPONENTSHIFTBLUE_W::new(self)
    }
    #[doc = "Bits 16:20 - Green component shift."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftgreen(&mut self) -> COMPONENTSHIFTGREEN_W<16> {
        COMPONENTSHIFTGREEN_W::new(self)
    }
    #[doc = "Bits 24:28 - Red component shift."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftred(&mut self) -> COMPONENTSHIFTRED_W<24> {
        COMPONENTSHIFTRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Color component offset for color format of LCD controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentshift](index.html) module"]
pub struct COLORCOMPONENTSHIFT_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTSHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentshift::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTSHIFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentshift::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTSHIFT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTSHIFT to value 0x0b05_0000"]
impl crate::Resettable for COLORCOMPONENTSHIFT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b05_0000;
}
