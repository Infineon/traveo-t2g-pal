#[doc = "Register `FGCCR` reader"]
pub struct R(crate::R<FGCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGCCR` writer"]
pub struct W(crate::W<FGCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGCCR_SPEC>;
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
impl From<crate::W<FGCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCBLUE` reader - Constant color - blue component."]
pub type CCBLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCBLUE` writer - Constant color - blue component."]
pub type CCBLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGCCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CCGREEN` reader - Constant color - green component."]
pub type CCGREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCGREEN` writer - Constant color - green component."]
pub type CCGREEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGCCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CCRED` reader - Constant color - red component."]
pub type CCRED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCRED` writer - Constant color - red component."]
pub type CCRED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGCCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CCALPHA` reader - Constant color - alpha value. In case of blending the value of 1 will be extended to 0xff."]
pub type CCALPHA_R = crate::BitReader<bool>;
#[doc = "Field `CCALPHA` writer - Constant color - alpha value. In case of blending the value of 1 will be extended to 0xff."]
pub type CCALPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Constant color - blue component."]
    #[inline(always)]
    pub fn ccblue(&self) -> CCBLUE_R {
        CCBLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Constant color - green component."]
    #[inline(always)]
    pub fn ccgreen(&self) -> CCGREEN_R {
        CCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Constant color - red component."]
    #[inline(always)]
    pub fn ccred(&self) -> CCRED_R {
        CCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Constant color - alpha value. In case of blending the value of 1 will be extended to 0xff."]
    #[inline(always)]
    pub fn ccalpha(&self) -> CCALPHA_R {
        CCALPHA_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Constant color - blue component."]
    #[inline(always)]
    #[must_use]
    pub fn ccblue(&mut self) -> CCBLUE_W<0> {
        CCBLUE_W::new(self)
    }
    #[doc = "Bits 8:15 - Constant color - green component."]
    #[inline(always)]
    #[must_use]
    pub fn ccgreen(&mut self) -> CCGREEN_W<8> {
        CCGREEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Constant color - red component."]
    #[inline(always)]
    #[must_use]
    pub fn ccred(&mut self) -> CCRED_W<16> {
        CCRED_W::new(self)
    }
    #[doc = "Bit 24 - Constant color - alpha value. In case of blending the value of 1 will be extended to 0xff."]
    #[inline(always)]
    #[must_use]
    pub fn ccalpha(&mut self) -> CCALPHA_W<24> {
        CCALPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Constant Color Register (shadowed)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgccr](index.html) module"]
pub struct FGCCR_SPEC;
impl crate::RegisterSpec for FGCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgccr::R](R) reader structure"]
impl crate::Readable for FGCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgccr::W](W) writer structure"]
impl crate::Writable for FGCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGCCR to value 0x01ff_ffff"]
impl crate::Resettable for FGCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff_ffff;
}
