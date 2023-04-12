#[doc = "Register `FRAMERESAMPLING` reader"]
pub struct R(crate::R<FRAMERESAMPLING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMERESAMPLING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMERESAMPLING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMERESAMPLING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMERESAMPLING` writer"]
pub struct W(crate::W<FRAMERESAMPLING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMERESAMPLING_SPEC>;
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
impl From<crate::W<FRAMERESAMPLING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMERESAMPLING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTX` reader - X coordinate of first sample point relative to origin."]
pub type STARTX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTX` writer - X coordinate of first sample point relative to origin."]
pub type STARTX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAMERESAMPLING_SPEC, u8, u8, 6, O>;
#[doc = "Field `STARTY` reader - Y coordinate of first sample point relative to origin."]
pub type STARTY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTY` writer - Y coordinate of first sample point relative to origin."]
pub type STARTY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAMERESAMPLING_SPEC, u8, u8, 6, O>;
#[doc = "Field `DELTAX` reader - Increment of X coordinate for horizontal step in destination frame."]
pub type DELTAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTAX` writer - Increment of X coordinate for horizontal step in destination frame."]
pub type DELTAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAMERESAMPLING_SPEC, u8, u8, 6, O>;
#[doc = "Field `DELTAY` reader - Increment of Y coordinate for vertical step in destination frame."]
pub type DELTAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTAY` writer - Increment of Y coordinate for vertical step in destination frame."]
pub type DELTAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAMERESAMPLING_SPEC, u8, u8, 6, O>;
#[doc = "Field `SWAPDIRECTION` reader - Swaps X and Y directions for re-sampling. When enabled (= 1) DeltaY is applied for horizontal and DeltaX for vertical step on destination frame."]
pub type SWAPDIRECTION_R = crate::BitReader<bool>;
#[doc = "Field `SWAPDIRECTION` writer - Swaps X and Y directions for re-sampling. When enabled (= 1) DeltaY is applied for horizontal and DeltaX for vertical step on destination frame."]
pub type SWAPDIRECTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRAMERESAMPLING_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - X coordinate of first sample point relative to origin."]
    #[inline(always)]
    pub fn startx(&self) -> STARTX_R {
        STARTX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Y coordinate of first sample point relative to origin."]
    #[inline(always)]
    pub fn starty(&self) -> STARTY_R {
        STARTY_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Increment of X coordinate for horizontal step in destination frame."]
    #[inline(always)]
    pub fn deltax(&self) -> DELTAX_R {
        DELTAX_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Increment of Y coordinate for vertical step in destination frame."]
    #[inline(always)]
    pub fn deltay(&self) -> DELTAY_R {
        DELTAY_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Swaps X and Y directions for re-sampling. When enabled (= 1) DeltaY is applied for horizontal and DeltaX for vertical step on destination frame."]
    #[inline(always)]
    pub fn swapdirection(&self) -> SWAPDIRECTION_R {
        SWAPDIRECTION_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - X coordinate of first sample point relative to origin."]
    #[inline(always)]
    #[must_use]
    pub fn startx(&mut self) -> STARTX_W<0> {
        STARTX_W::new(self)
    }
    #[doc = "Bits 6:11 - Y coordinate of first sample point relative to origin."]
    #[inline(always)]
    #[must_use]
    pub fn starty(&mut self) -> STARTY_W<6> {
        STARTY_W::new(self)
    }
    #[doc = "Bits 12:17 - Increment of X coordinate for horizontal step in destination frame."]
    #[inline(always)]
    #[must_use]
    pub fn deltax(&mut self) -> DELTAX_W<12> {
        DELTAX_W::new(self)
    }
    #[doc = "Bits 18:23 - Increment of Y coordinate for vertical step in destination frame."]
    #[inline(always)]
    #[must_use]
    pub fn deltay(&mut self) -> DELTAY_W<18> {
        DELTAY_W::new(self)
    }
    #[doc = "Bit 24 - Swaps X and Y directions for re-sampling. When enabled (= 1) DeltaY is applied for horizontal and DeltaX for vertical step on destination frame."]
    #[inline(always)]
    #[must_use]
    pub fn swapdirection(&mut self) -> SWAPDIRECTION_W<24> {
        SWAPDIRECTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Resampling options for output frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frameresampling](index.html) module"]
pub struct FRAMERESAMPLING_SPEC;
impl crate::RegisterSpec for FRAMERESAMPLING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frameresampling::R](R) reader structure"]
impl crate::Readable for FRAMERESAMPLING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frameresampling::W](W) writer structure"]
impl crate::Writable for FRAMERESAMPLING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMERESAMPLING to value 0x0010_4000"]
impl crate::Resettable for FRAMERESAMPLING_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_4000;
}
