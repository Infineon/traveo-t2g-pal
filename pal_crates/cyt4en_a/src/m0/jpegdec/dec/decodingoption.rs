#[doc = "Register `DECODINGOPTION` reader"]
pub struct R(crate::R<DECODINGOPTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECODINGOPTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECODINGOPTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECODINGOPTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DECODINGOPTION` writer"]
pub struct W(crate::W<DECODINGOPTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DECODINGOPTION_SPEC>;
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
impl From<crate::W<DECODINGOPTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DECODINGOPTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MARKERSKIP` reader - This bit enables Marker Skip Function. 0: disable 1: enable"]
pub type MARKERSKIP_R = crate::BitReader<bool>;
#[doc = "Field `MARKERSKIP` writer - This bit enables Marker Skip Function. 0: disable 1: enable"]
pub type MARKERSKIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECODINGOPTION_SPEC, bool, O>;
#[doc = "Field `CROP` reader - This bit enables Crop Function. 0: disable 1: enable"]
pub type CROP_R = crate::BitReader<bool>;
#[doc = "Field `CROP` writer - This bit enables Crop Function. 0: disable 1: enable"]
pub type CROP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECODINGOPTION_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - This bit enables Marker Skip Function. 0: disable 1: enable"]
    #[inline(always)]
    pub fn markerskip(&self) -> MARKERSKIP_R {
        MARKERSKIP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit enables Crop Function. 0: disable 1: enable"]
    #[inline(always)]
    pub fn crop(&self) -> CROP_R {
        CROP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit enables Marker Skip Function. 0: disable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn markerskip(&mut self) -> MARKERSKIP_W<1> {
        MARKERSKIP_W::new(self)
    }
    #[doc = "Bit 3 - This bit enables Crop Function. 0: disable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn crop(&mut self) -> CROP_W<3> {
        CROP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Decoding option settings.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decodingoption](index.html) module"]
pub struct DECODINGOPTION_SPEC;
impl crate::RegisterSpec for DECODINGOPTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decodingoption::R](R) reader structure"]
impl crate::Readable for DECODINGOPTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [decodingoption::W](W) writer structure"]
impl crate::Writable for DECODINGOPTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DECODINGOPTION to value 0"]
impl crate::Resettable for DECODINGOPTION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
