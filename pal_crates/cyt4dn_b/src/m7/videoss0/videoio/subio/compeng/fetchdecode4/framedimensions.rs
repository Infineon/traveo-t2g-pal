#[doc = "Register `FRAMEDIMENSIONS` reader"]
pub struct R(crate::R<FRAMEDIMENSIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMEDIMENSIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMEDIMENSIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMEDIMENSIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMEDIMENSIONS` writer"]
pub struct W(crate::W<FRAMEDIMENSIONS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMEDIMENSIONS_SPEC>;
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
impl From<crate::W<FRAMEDIMENSIONS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMEDIMENSIONS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEWIDTH` reader - Frame width minus one."]
pub type FRAMEWIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMEWIDTH` writer - Frame width minus one."]
pub type FRAMEWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAMEDIMENSIONS_SPEC, u16, u16, 14, O>;
#[doc = "Field `FRAMEHEIGHT` reader - Frame height minus one."]
pub type FRAMEHEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMEHEIGHT` writer - Frame height minus one."]
pub type FRAMEHEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAMEDIMENSIONS_SPEC, u16, u16, 14, O>;
#[doc = "Field `EMPTYFRAME` reader - When enabled output frame is empty. FrameWidth/Height settings have no effect then. Can be used to load shadows or to generate synchronization signals only (frame/sequence complete). If enabled, InputSelect must be set to INACTIVE."]
pub type EMPTYFRAME_R = crate::BitReader<bool>;
#[doc = "Field `EMPTYFRAME` writer - When enabled output frame is empty. FrameWidth/Height settings have no effect then. Can be used to load shadows or to generate synchronization signals only (frame/sequence complete). If enabled, InputSelect must be set to INACTIVE."]
pub type EMPTYFRAME_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRAMEDIMENSIONS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - Frame width minus one."]
    #[inline(always)]
    pub fn framewidth(&self) -> FRAMEWIDTH_R {
        FRAMEWIDTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Frame height minus one."]
    #[inline(always)]
    pub fn frameheight(&self) -> FRAMEHEIGHT_R {
        FRAMEHEIGHT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When enabled output frame is empty. FrameWidth/Height settings have no effect then. Can be used to load shadows or to generate synchronization signals only (frame/sequence complete). If enabled, InputSelect must be set to INACTIVE."]
    #[inline(always)]
    pub fn emptyframe(&self) -> EMPTYFRAME_R {
        EMPTYFRAME_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame width minus one."]
    #[inline(always)]
    #[must_use]
    pub fn framewidth(&mut self) -> FRAMEWIDTH_W<0> {
        FRAMEWIDTH_W::new(self)
    }
    #[doc = "Bits 16:29 - Frame height minus one."]
    #[inline(always)]
    #[must_use]
    pub fn frameheight(&mut self) -> FRAMEHEIGHT_W<16> {
        FRAMEHEIGHT_W::new(self)
    }
    #[doc = "Bit 31 - When enabled output frame is empty. FrameWidth/Height settings have no effect then. Can be used to load shadows or to generate synchronization signals only (frame/sequence complete). If enabled, InputSelect must be set to INACTIVE."]
    #[inline(always)]
    #[must_use]
    pub fn emptyframe(&mut self) -> EMPTYFRAME_W<31> {
        EMPTYFRAME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output frame dimension.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framedimensions](index.html) module"]
pub struct FRAMEDIMENSIONS_SPEC;
impl crate::RegisterSpec for FRAMEDIMENSIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framedimensions::R](R) reader structure"]
impl crate::Readable for FRAMEDIMENSIONS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framedimensions::W](W) writer structure"]
impl crate::Writable for FRAMEDIMENSIONS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMEDIMENSIONS to value 0x00ef_013f"]
impl crate::Resettable for FRAMEDIMENSIONS_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ef_013f;
}
