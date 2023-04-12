#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAWPIXEL` reader - Raw pixel mode. If enabled (value = 1), the ComponentBits/Shift settings are replaced for all layers by fixed values that allow passing the pixel data read from memory unchanged to subsequent units (e.g. for reading coordinate layers). Multiply stages and transparent color are deactivated. Skip and Tile pixels are not affected by this setting."]
pub type RAWPIXEL_R = crate::BitReader<bool>;
#[doc = "Field `RAWPIXEL` writer - Raw pixel mode. If enabled (value = 1), the ComponentBits/Shift settings are replaced for all layers by fixed values that allow passing the pixel data read from memory unchanged to subsequent units (e.g. for reading coordinate layers). Multiply stages and transparent color are deactivated. Skip and Tile pixels are not affected by this setting."]
pub type RAWPIXEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `CLIPCOLOR` reader - Selects which color to take for pixels that do not lie inside the clip window of any layer."]
pub type CLIPCOLOR_R = crate::BitReader<CLIPCOLOR_A>;
#[doc = "Selects which color to take for pixels that do not lie inside the clip window of any layer.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLIPCOLOR_A {
    #[doc = "0: Null color."]
    NULL = 0,
    #[doc = "1: Color of layer number given by ClipLayer (or layer 0 when Fetch unit has one layer only). The color is then the layer's source or tiling color."]
    LAYER = 1,
}
impl From<CLIPCOLOR_A> for bool {
    #[inline(always)]
    fn from(variant: CLIPCOLOR_A) -> Self {
        variant as u8 != 0
    }
}
impl CLIPCOLOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLIPCOLOR_A {
        match self.bits {
            false => CLIPCOLOR_A::NULL,
            true => CLIPCOLOR_A::LAYER,
        }
    }
    #[doc = "Checks if the value of the field is `NULL`"]
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        *self == CLIPCOLOR_A::NULL
    }
    #[doc = "Checks if the value of the field is `LAYER`"]
    #[inline(always)]
    pub fn is_layer(&self) -> bool {
        *self == CLIPCOLOR_A::LAYER
    }
}
#[doc = "Field `CLIPCOLOR` writer - Selects which color to take for pixels that do not lie inside the clip window of any layer."]
pub type CLIPCOLOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, CLIPCOLOR_A, O>;
impl<'a, const O: u8> CLIPCOLOR_W<'a, O> {
    #[doc = "Null color."]
    #[inline(always)]
    pub fn null(self) -> &'a mut W {
        self.variant(CLIPCOLOR_A::NULL)
    }
    #[doc = "Color of layer number given by ClipLayer (or layer 0 when Fetch unit has one layer only). The color is then the layer's source or tiling color."]
    #[inline(always)]
    pub fn layer(self) -> &'a mut W {
        self.variant(CLIPCOLOR_A::LAYER)
    }
}
impl R {
    #[doc = "Bit 7 - Raw pixel mode. If enabled (value = 1), the ComponentBits/Shift settings are replaced for all layers by fixed values that allow passing the pixel data read from memory unchanged to subsequent units (e.g. for reading coordinate layers). Multiply stages and transparent color are deactivated. Skip and Tile pixels are not affected by this setting."]
    #[inline(always)]
    pub fn rawpixel(&self) -> RAWPIXEL_R {
        RAWPIXEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Selects which color to take for pixels that do not lie inside the clip window of any layer."]
    #[inline(always)]
    pub fn clipcolor(&self) -> CLIPCOLOR_R {
        CLIPCOLOR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Raw pixel mode. If enabled (value = 1), the ComponentBits/Shift settings are replaced for all layers by fixed values that allow passing the pixel data read from memory unchanged to subsequent units (e.g. for reading coordinate layers). Multiply stages and transparent color are deactivated. Skip and Tile pixels are not affected by this setting."]
    #[inline(always)]
    #[must_use]
    pub fn rawpixel(&mut self) -> RAWPIXEL_W<7> {
        RAWPIXEL_W::new(self)
    }
    #[doc = "Bit 16 - Selects which color to take for pixels that do not lie inside the clip window of any layer."]
    #[inline(always)]
    #[must_use]
    pub fn clipcolor(&mut self) -> CLIPCOLOR_W<16> {
        CLIPCOLOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shared common control settings for all layers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0x0001_0000"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
