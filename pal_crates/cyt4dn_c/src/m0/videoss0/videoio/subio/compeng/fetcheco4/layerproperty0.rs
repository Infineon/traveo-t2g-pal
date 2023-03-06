#[doc = "Register `LAYERPROPERTY0` reader"]
pub struct R(crate::R<LAYERPROPERTY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYERPROPERTY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYERPROPERTY0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYERPROPERTY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYERPROPERTY0` writer"]
pub struct W(crate::W<LAYERPROPERTY0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYERPROPERTY0_SPEC>;
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
impl From<crate::W<LAYERPROPERTY0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYERPROPERTY0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TILEMODE0` reader - Select the tile mode for pixels outside the source buffer. Clip color (0,0,0,0) takes precedence if a pixel becomes subject to both tiling and clipping."]
pub type TILEMODE0_R = crate::FieldReader<u8, TILEMODE0_A>;
#[doc = "Select the tile mode for pixels outside the source buffer. Clip color (0,0,0,0) takes precedence if a pixel becomes subject to both tiling and clipping.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TILEMODE0_A {
    #[doc = "0: Use zero value"]
    TILE_FILL_ZERO = 0,
    #[doc = "1: Use constant color register value"]
    TILE_FILL_CONSTANT = 1,
    #[doc = "2: Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD = 2,
    #[doc = "3: Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD_ZERO = 3,
}
impl From<TILEMODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: TILEMODE0_A) -> Self {
        variant as _
    }
}
impl TILEMODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TILEMODE0_A {
        match self.bits {
            0 => TILEMODE0_A::TILE_FILL_ZERO,
            1 => TILEMODE0_A::TILE_FILL_CONSTANT,
            2 => TILEMODE0_A::TILE_PAD,
            3 => TILEMODE0_A::TILE_PAD_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_ZERO`"]
    #[inline(always)]
    pub fn is_tile_fill_zero(&self) -> bool {
        *self == TILEMODE0_A::TILE_FILL_ZERO
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_CONSTANT`"]
    #[inline(always)]
    pub fn is_tile_fill_constant(&self) -> bool {
        *self == TILEMODE0_A::TILE_FILL_CONSTANT
    }
    #[doc = "Checks if the value of the field is `TILE_PAD`"]
    #[inline(always)]
    pub fn is_tile_pad(&self) -> bool {
        *self == TILEMODE0_A::TILE_PAD
    }
    #[doc = "Checks if the value of the field is `TILE_PAD_ZERO`"]
    #[inline(always)]
    pub fn is_tile_pad_zero(&self) -> bool {
        *self == TILEMODE0_A::TILE_PAD_ZERO
    }
}
#[doc = "Field `TILEMODE0` writer - Select the tile mode for pixels outside the source buffer. Clip color (0,0,0,0) takes precedence if a pixel becomes subject to both tiling and clipping."]
pub type TILEMODE0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY0_SPEC, u8, TILEMODE0_A, 2, O>;
impl<'a, const O: u8> TILEMODE0_W<'a, O> {
    #[doc = "Use zero value"]
    #[inline(always)]
    pub fn tile_fill_zero(self) -> &'a mut W {
        self.variant(TILEMODE0_A::TILE_FILL_ZERO)
    }
    #[doc = "Use constant color register value"]
    #[inline(always)]
    pub fn tile_fill_constant(self) -> &'a mut W {
        self.variant(TILEMODE0_A::TILE_FILL_CONSTANT)
    }
    #[doc = "Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad(self) -> &'a mut W {
        self.variant(TILEMODE0_A::TILE_PAD)
    }
    #[doc = "Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad_zero(self) -> &'a mut W {
        self.variant(TILEMODE0_A::TILE_PAD_ZERO)
    }
}
#[doc = "Field `CLIPWINDOWENABLE0` reader - Value 1 enables the clip window for this layer. Pixels outside the clip window get the clip color, pixels inside the source or tiling color."]
pub type CLIPWINDOWENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `CLIPWINDOWENABLE0` writer - Value 1 enables the clip window for this layer. Pixels outside the clip window get the clip color, pixels inside the source or tiling color."]
pub type CLIPWINDOWENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `SOURCEBUFFERENABLE0` reader - Value 1 enables the source buffer for this layer. When disabled the tiling color is used only (TileMode TILE_PAD not allowed)."]
pub type SOURCEBUFFERENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `SOURCEBUFFERENABLE0` writer - Value 1 enables the source buffer for this layer. When disabled the tiling color is used only (TileMode TILE_PAD not allowed)."]
pub type SOURCEBUFFERENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 1:2 - Select the tile mode for pixels outside the source buffer. Clip color (0,0,0,0) takes precedence if a pixel becomes subject to both tiling and clipping."]
    #[inline(always)]
    pub fn tilemode0(&self) -> TILEMODE0_R {
        TILEMODE0_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 30 - Value 1 enables the clip window for this layer. Pixels outside the clip window get the clip color, pixels inside the source or tiling color."]
    #[inline(always)]
    pub fn clipwindowenable0(&self) -> CLIPWINDOWENABLE0_R {
        CLIPWINDOWENABLE0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Value 1 enables the source buffer for this layer. When disabled the tiling color is used only (TileMode TILE_PAD not allowed)."]
    #[inline(always)]
    pub fn sourcebufferenable0(&self) -> SOURCEBUFFERENABLE0_R {
        SOURCEBUFFERENABLE0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - Select the tile mode for pixels outside the source buffer. Clip color (0,0,0,0) takes precedence if a pixel becomes subject to both tiling and clipping."]
    #[inline(always)]
    #[must_use]
    pub fn tilemode0(&mut self) -> TILEMODE0_W<1> {
        TILEMODE0_W::new(self)
    }
    #[doc = "Bit 30 - Value 1 enables the clip window for this layer. Pixels outside the clip window get the clip color, pixels inside the source or tiling color."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowenable0(&mut self) -> CLIPWINDOWENABLE0_W<30> {
        CLIPWINDOWENABLE0_W::new(self)
    }
    #[doc = "Bit 31 - Value 1 enables the source buffer for this layer. When disabled the tiling color is used only (TileMode TILE_PAD not allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn sourcebufferenable0(&mut self) -> SOURCEBUFFERENABLE0_W<31> {
        SOURCEBUFFERENABLE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common properties of layer 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layerproperty0](index.html) module"]
pub struct LAYERPROPERTY0_SPEC;
impl crate::RegisterSpec for LAYERPROPERTY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layerproperty0::R](R) reader structure"]
impl crate::Readable for LAYERPROPERTY0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layerproperty0::W](W) writer structure"]
impl crate::Writable for LAYERPROPERTY0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYERPROPERTY0 to value 0x8000_0000"]
impl crate::Resettable for LAYERPROPERTY0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
