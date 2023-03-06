#[doc = "Register `TILECOLORRGB` reader"]
pub struct R(crate::R<TILECOLORRGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TILECOLORRGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TILECOLORRGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TILECOLORRGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TILECOLORRGB` writer"]
pub struct W(crate::W<TILECOLORRGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TILECOLORRGB_SPEC>;
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
impl From<crate::W<TILECOLORRGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TILECOLORRGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TILE_COLOR_RED` reader - Sets the red color value for pixels outside of the slice. (format is unsigned integer)"]
pub type TILE_COLOR_RED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TILE_COLOR_RED` writer - Sets the red color value for pixels outside of the slice. (format is unsigned integer)"]
pub type TILE_COLOR_RED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TILECOLORRGB_SPEC, u16, u16, 10, O>;
#[doc = "Field `TILE_COLOR_GREEN` reader - Sets the green color value for pixels outside of the slice. (format is unsigned integer)"]
pub type TILE_COLOR_GREEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TILE_COLOR_GREEN` writer - Sets the green color value for pixels outside of the slice. (format is unsigned integer)"]
pub type TILE_COLOR_GREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TILECOLORRGB_SPEC, u16, u16, 10, O>;
#[doc = "Field `TILE_COLOR_BLUE` reader - Sets the blue color value for pixels outside of the slice. (format is unsigned integer)"]
pub type TILE_COLOR_BLUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TILE_COLOR_BLUE` writer - Sets the blue color value for pixels outside of the slice. (format is unsigned integer)"]
pub type TILE_COLOR_BLUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TILECOLORRGB_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sets the red color value for pixels outside of the slice. (format is unsigned integer)"]
    #[inline(always)]
    pub fn tile_color_red(&self) -> TILE_COLOR_RED_R {
        TILE_COLOR_RED_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Sets the green color value for pixels outside of the slice. (format is unsigned integer)"]
    #[inline(always)]
    pub fn tile_color_green(&self) -> TILE_COLOR_GREEN_R {
        TILE_COLOR_GREEN_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Sets the blue color value for pixels outside of the slice. (format is unsigned integer)"]
    #[inline(always)]
    pub fn tile_color_blue(&self) -> TILE_COLOR_BLUE_R {
        TILE_COLOR_BLUE_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sets the red color value for pixels outside of the slice. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn tile_color_red(&mut self) -> TILE_COLOR_RED_W<0> {
        TILE_COLOR_RED_W::new(self)
    }
    #[doc = "Bits 10:19 - Sets the green color value for pixels outside of the slice. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn tile_color_green(&mut self) -> TILE_COLOR_GREEN_W<10> {
        TILE_COLOR_GREEN_W::new(self)
    }
    #[doc = "Bits 20:29 - Sets the blue color value for pixels outside of the slice. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn tile_color_blue(&mut self) -> TILE_COLOR_BLUE_W<20> {
        TILE_COLOR_BLUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Color for outside of the slice.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tilecolorrgb](index.html) module"]
pub struct TILECOLORRGB_SPEC;
impl crate::RegisterSpec for TILECOLORRGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tilecolorrgb::R](R) reader structure"]
impl crate::Readable for TILECOLORRGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tilecolorrgb::W](W) writer structure"]
impl crate::Writable for TILECOLORRGB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TILECOLORRGB to value 0"]
impl crate::Resettable for TILECOLORRGB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
