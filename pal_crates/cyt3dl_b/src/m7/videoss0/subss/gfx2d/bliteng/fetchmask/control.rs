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
#[doc = "Field `RASTERMODE` reader - Selects a method how to generate source buffer sample points."]
pub type RASTERMODE_R = crate::FieldReader<u8, RASTERMODE_A>;
#[doc = "Selects a method how to generate source buffer sample points.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RASTERMODE_A {
    #[doc = "0: \\[IBO only, for LBO use PERSPECTIVE with corresponding Geometry\\]
First sample at StartX/Y relative to origin. Hor/ver increments using DeltaX/Y and DeltaSwap setup."]
    NORMAL = 0,
    #[doc = "1: \\[FetchDecode/FetchBlit only\\]
Source buffer is an encoded bit stream. First sample at origin (0,0). Hor/ver increments = (1,0)/(0,1)."]
    DECODE = 1,
    #[doc = "2: \\[FetchBlit/Warp only\\]
Arbitrary warping (filter is active). Coordinates are read from frame input port. InputSelect must be set to COORDINATE. ArbStartX/Y and ArbDeltaXX/XY/YX/YY must be setup."]
    ARBITRARY = 2,
    #[doc = "3: \\[FetchBlit only\\]
Affine/Perspective warping (filter is active). First sample at PerspStartX/Y/W. Hor/ver increments using PerspDeltaXX/XY/YX/YY/WX/WY. Homogeneous coordinates. See also field Geometry to reduce coordinate amount."]
    PERSPECTIVE = 3,
    #[doc = "4: \\[FetchBlit/Decode only\\]
Source buffer is packed YUV 422. First sample at origin (0,0). Hor/ver increments = (1,0)/(0,1). All correllated window widths and horizontal offsets must be even."]
    YUV422 = 4,
}
impl From<RASTERMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: RASTERMODE_A) -> Self {
        variant as _
    }
}
impl RASTERMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RASTERMODE_A> {
        match self.bits {
            0 => Some(RASTERMODE_A::NORMAL),
            1 => Some(RASTERMODE_A::DECODE),
            2 => Some(RASTERMODE_A::ARBITRARY),
            3 => Some(RASTERMODE_A::PERSPECTIVE),
            4 => Some(RASTERMODE_A::YUV422),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RASTERMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `DECODE`"]
    #[inline(always)]
    pub fn is_decode(&self) -> bool {
        *self == RASTERMODE_A::DECODE
    }
    #[doc = "Checks if the value of the field is `ARBITRARY`"]
    #[inline(always)]
    pub fn is_arbitrary(&self) -> bool {
        *self == RASTERMODE_A::ARBITRARY
    }
    #[doc = "Checks if the value of the field is `PERSPECTIVE`"]
    #[inline(always)]
    pub fn is_perspective(&self) -> bool {
        *self == RASTERMODE_A::PERSPECTIVE
    }
    #[doc = "Checks if the value of the field is `YUV422`"]
    #[inline(always)]
    pub fn is_yuv422(&self) -> bool {
        *self == RASTERMODE_A::YUV422
    }
}
#[doc = "Field `RASTERMODE` writer - Selects a method how to generate source buffer sample points."]
pub type RASTERMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, RASTERMODE_A, 3, O>;
impl<'a, const O: u8> RASTERMODE_W<'a, O> {
    #[doc = "IBO only, for LBO use PERSPECTIVE with corresponding Geometry\\]
First sample at StartX/Y relative to origin. Hor/ver increments using DeltaX/Y and DeltaSwap setup."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RASTERMODE_A::NORMAL)
    }
    #[doc = "FetchDecode/FetchBlit only\\]
Source buffer is an encoded bit stream. First sample at origin (0,0). Hor/ver increments = (1,0)/(0,1)."]
    #[inline(always)]
    pub fn decode(self) -> &'a mut W {
        self.variant(RASTERMODE_A::DECODE)
    }
    #[doc = "FetchBlit/Warp only\\]
Arbitrary warping (filter is active). Coordinates are read from frame input port. InputSelect must be set to COORDINATE. ArbStartX/Y and ArbDeltaXX/XY/YX/YY must be setup."]
    #[inline(always)]
    pub fn arbitrary(self) -> &'a mut W {
        self.variant(RASTERMODE_A::ARBITRARY)
    }
    #[doc = "FetchBlit only\\]
Affine/Perspective warping (filter is active). First sample at PerspStartX/Y/W. Hor/ver increments using PerspDeltaXX/XY/YX/YY/WX/WY. Homogeneous coordinates. See also field Geometry to reduce coordinate amount."]
    #[inline(always)]
    pub fn perspective(self) -> &'a mut W {
        self.variant(RASTERMODE_A::PERSPECTIVE)
    }
    #[doc = "FetchBlit/Decode only\\]
Source buffer is packed YUV 422. First sample at origin (0,0). Hor/ver increments = (1,0)/(0,1). All correllated window widths and horizontal offsets must be even."]
    #[inline(always)]
    pub fn yuv422(self) -> &'a mut W {
        self.variant(RASTERMODE_A::YUV422)
    }
}
#[doc = "Field `INPUTSELECT` reader - Selects function for the frame input port."]
pub type INPUTSELECT_R = crate::FieldReader<u8, INPUTSELECT_A>;
#[doc = "Selects function for the frame input port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUTSELECT_A {
    #[doc = "0: Not used."]
    INACTIVE = 0,
    #[doc = "1: Used for component packing (e.g. UV or source alpha buffer)."]
    COMPPACK = 1,
    #[doc = "2: Used for RGB and alpha pre-multiply stage (mask alpha buffer)."]
    ALPHAMASK = 2,
    #[doc = "3: Used for arbitrary warping (coordinate buffer)."]
    COORDINATE = 3,
}
impl From<INPUTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUTSELECT_A) -> Self {
        variant as _
    }
}
impl INPUTSELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUTSELECT_A {
        match self.bits {
            0 => INPUTSELECT_A::INACTIVE,
            1 => INPUTSELECT_A::COMPPACK,
            2 => INPUTSELECT_A::ALPHAMASK,
            3 => INPUTSELECT_A::COORDINATE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == INPUTSELECT_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `COMPPACK`"]
    #[inline(always)]
    pub fn is_comppack(&self) -> bool {
        *self == INPUTSELECT_A::COMPPACK
    }
    #[doc = "Checks if the value of the field is `ALPHAMASK`"]
    #[inline(always)]
    pub fn is_alphamask(&self) -> bool {
        *self == INPUTSELECT_A::ALPHAMASK
    }
    #[doc = "Checks if the value of the field is `COORDINATE`"]
    #[inline(always)]
    pub fn is_coordinate(&self) -> bool {
        *self == INPUTSELECT_A::COORDINATE
    }
}
#[doc = "Field `INPUTSELECT` writer - Selects function for the frame input port."]
pub type INPUTSELECT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONTROL_SPEC, u8, INPUTSELECT_A, 2, O>;
impl<'a, const O: u8> INPUTSELECT_W<'a, O> {
    #[doc = "Not used."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(INPUTSELECT_A::INACTIVE)
    }
    #[doc = "Used for component packing (e.g. UV or source alpha buffer)."]
    #[inline(always)]
    pub fn comppack(self) -> &'a mut W {
        self.variant(INPUTSELECT_A::COMPPACK)
    }
    #[doc = "Used for RGB and alpha pre-multiply stage (mask alpha buffer)."]
    #[inline(always)]
    pub fn alphamask(self) -> &'a mut W {
        self.variant(INPUTSELECT_A::ALPHAMASK)
    }
    #[doc = "Used for arbitrary warping (coordinate buffer)."]
    #[inline(always)]
    pub fn coordinate(self) -> &'a mut W {
        self.variant(INPUTSELECT_A::COORDINATE)
    }
}
#[doc = "Field `YUV422UPSAMPLINGMODE` reader - Selects a method for horizontal up-sampling of YUV 422/420 input data."]
pub type YUV422UPSAMPLINGMODE_R = crate::BitReader<YUV422UPSAMPLINGMODE_A>;
#[doc = "Selects a method for horizontal up-sampling of YUV 422/420 input data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YUV422UPSAMPLINGMODE_A {
    #[doc = "0: Replicate mode for interspersed samples (UV samples between Y samples)."]
    REPLICATE = 0,
    #[doc = "1: Interpolate mode for coaligned samples (UV samples at Y sample positions)."]
    INTERPOLATE = 1,
}
impl From<YUV422UPSAMPLINGMODE_A> for bool {
    #[inline(always)]
    fn from(variant: YUV422UPSAMPLINGMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl YUV422UPSAMPLINGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUV422UPSAMPLINGMODE_A {
        match self.bits {
            false => YUV422UPSAMPLINGMODE_A::REPLICATE,
            true => YUV422UPSAMPLINGMODE_A::INTERPOLATE,
        }
    }
    #[doc = "Checks if the value of the field is `REPLICATE`"]
    #[inline(always)]
    pub fn is_replicate(&self) -> bool {
        *self == YUV422UPSAMPLINGMODE_A::REPLICATE
    }
    #[doc = "Checks if the value of the field is `INTERPOLATE`"]
    #[inline(always)]
    pub fn is_interpolate(&self) -> bool {
        *self == YUV422UPSAMPLINGMODE_A::INTERPOLATE
    }
}
#[doc = "Field `YUV422UPSAMPLINGMODE` writer - Selects a method for horizontal up-sampling of YUV 422/420 input data."]
pub type YUV422UPSAMPLINGMODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, YUV422UPSAMPLINGMODE_A, O>;
impl<'a, const O: u8> YUV422UPSAMPLINGMODE_W<'a, O> {
    #[doc = "Replicate mode for interspersed samples (UV samples between Y samples)."]
    #[inline(always)]
    pub fn replicate(self) -> &'a mut W {
        self.variant(YUV422UPSAMPLINGMODE_A::REPLICATE)
    }
    #[doc = "Interpolate mode for coaligned samples (UV samples at Y sample positions)."]
    #[inline(always)]
    pub fn interpolate(self) -> &'a mut W {
        self.variant(YUV422UPSAMPLINGMODE_A::INTERPOLATE)
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
#[doc = "Field `FILTERMODE` reader - Use this to select between nearest and bilinear filtering. Only has an effect if rastermode == ARBITRARY or rastermode == PERSPECTIVE or rastermode == AFFINE."]
pub type FILTERMODE_R = crate::FieldReader<u8, FILTERMODE_A>;
#[doc = "Use this to select between nearest and bilinear filtering. Only has an effect if rastermode == ARBITRARY or rastermode == PERSPECTIVE or rastermode == AFFINE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTERMODE_A {
    #[doc = "0: Chooses pixel closest to sample point"]
    NEAREST = 0,
    #[doc = "1: Calculates result from 4 pixels closest to sample point"]
    BILINEAR = 1,
    #[doc = "2: FIR mode with 2 programmable pixel positions and coefficients"]
    FIR2 = 2,
    #[doc = "3: FIR mode with 4 programmable pixel positions and coefficients"]
    FIR4 = 3,
    #[doc = "4: Calculates result from 2 pixels closest to the sample point and on the same line"]
    HOR_LINEAR = 4,
}
impl From<FILTERMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTERMODE_A) -> Self {
        variant as _
    }
}
impl FILTERMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FILTERMODE_A> {
        match self.bits {
            0 => Some(FILTERMODE_A::NEAREST),
            1 => Some(FILTERMODE_A::BILINEAR),
            2 => Some(FILTERMODE_A::FIR2),
            3 => Some(FILTERMODE_A::FIR4),
            4 => Some(FILTERMODE_A::HOR_LINEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NEAREST`"]
    #[inline(always)]
    pub fn is_nearest(&self) -> bool {
        *self == FILTERMODE_A::NEAREST
    }
    #[doc = "Checks if the value of the field is `BILINEAR`"]
    #[inline(always)]
    pub fn is_bilinear(&self) -> bool {
        *self == FILTERMODE_A::BILINEAR
    }
    #[doc = "Checks if the value of the field is `FIR2`"]
    #[inline(always)]
    pub fn is_fir2(&self) -> bool {
        *self == FILTERMODE_A::FIR2
    }
    #[doc = "Checks if the value of the field is `FIR4`"]
    #[inline(always)]
    pub fn is_fir4(&self) -> bool {
        *self == FILTERMODE_A::FIR4
    }
    #[doc = "Checks if the value of the field is `HOR_LINEAR`"]
    #[inline(always)]
    pub fn is_hor_linear(&self) -> bool {
        *self == FILTERMODE_A::HOR_LINEAR
    }
}
#[doc = "Field `FILTERMODE` writer - Use this to select between nearest and bilinear filtering. Only has an effect if rastermode == ARBITRARY or rastermode == PERSPECTIVE or rastermode == AFFINE."]
pub type FILTERMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, FILTERMODE_A, 3, O>;
impl<'a, const O: u8> FILTERMODE_W<'a, O> {
    #[doc = "Chooses pixel closest to sample point"]
    #[inline(always)]
    pub fn nearest(self) -> &'a mut W {
        self.variant(FILTERMODE_A::NEAREST)
    }
    #[doc = "Calculates result from 4 pixels closest to sample point"]
    #[inline(always)]
    pub fn bilinear(self) -> &'a mut W {
        self.variant(FILTERMODE_A::BILINEAR)
    }
    #[doc = "FIR mode with 2 programmable pixel positions and coefficients"]
    #[inline(always)]
    pub fn fir2(self) -> &'a mut W {
        self.variant(FILTERMODE_A::FIR2)
    }
    #[doc = "FIR mode with 4 programmable pixel positions and coefficients"]
    #[inline(always)]
    pub fn fir4(self) -> &'a mut W {
        self.variant(FILTERMODE_A::FIR4)
    }
    #[doc = "Calculates result from 2 pixels closest to the sample point and on the same line"]
    #[inline(always)]
    pub fn hor_linear(self) -> &'a mut W {
        self.variant(FILTERMODE_A::HOR_LINEAR)
    }
}
#[doc = "Field `GEOMETRY` reader - Control the geomety settings (PerspStartX/Y/W and PerspDeltaXX/XY/YX/YY/XW/YW usage) for the raster mode PERSPECTIVE."]
pub type GEOMETRY_R = crate::FieldReader<u8, GEOMETRY_A>;
#[doc = "Control the geomety settings (PerspStartX/Y/W and PerspDeltaXX/XY/YX/YY/XW/YW usage) for the raster mode PERSPECTIVE.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GEOMETRY_A {
    #[doc = "0: PerspStartX/Y are used. (PerspDeltaXY/YX/XW/YW are 0. PerspDeltaXX/YY and PerspStartW are 1.)"]
    TRANSLATE = 0,
    #[doc = "1: PerspStartX/Y and PerspDeltaXX/YY are used. (PerspDeltaXY/YX/XW/YW are 0. PerspStartW is 1.)"]
    SCALE = 1,
    #[doc = "2: PerspStartX/Y and PerspDeltaXX/XY/YX/YY are used. (PerspDeltaXW/YW are 0. PerspStartW is 1.)"]
    ROTATION = 2,
    #[doc = "3: All values PerspStartX/Y/W and PerspDeltaXX/XY/YX/YY/XW/YW are used"]
    PERSPECTIVE = 3,
}
impl From<GEOMETRY_A> for u8 {
    #[inline(always)]
    fn from(variant: GEOMETRY_A) -> Self {
        variant as _
    }
}
impl GEOMETRY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GEOMETRY_A {
        match self.bits {
            0 => GEOMETRY_A::TRANSLATE,
            1 => GEOMETRY_A::SCALE,
            2 => GEOMETRY_A::ROTATION,
            3 => GEOMETRY_A::PERSPECTIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRANSLATE`"]
    #[inline(always)]
    pub fn is_translate(&self) -> bool {
        *self == GEOMETRY_A::TRANSLATE
    }
    #[doc = "Checks if the value of the field is `SCALE`"]
    #[inline(always)]
    pub fn is_scale(&self) -> bool {
        *self == GEOMETRY_A::SCALE
    }
    #[doc = "Checks if the value of the field is `ROTATION`"]
    #[inline(always)]
    pub fn is_rotation(&self) -> bool {
        *self == GEOMETRY_A::ROTATION
    }
    #[doc = "Checks if the value of the field is `PERSPECTIVE`"]
    #[inline(always)]
    pub fn is_perspective(&self) -> bool {
        *self == GEOMETRY_A::PERSPECTIVE
    }
}
#[doc = "Field `GEOMETRY` writer - Control the geomety settings (PerspStartX/Y/W and PerspDeltaXX/XY/YX/YY/XW/YW usage) for the raster mode PERSPECTIVE."]
pub type GEOMETRY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONTROL_SPEC, u8, GEOMETRY_A, 2, O>;
impl<'a, const O: u8> GEOMETRY_W<'a, O> {
    #[doc = "PerspStartX/Y are used. (PerspDeltaXY/YX/XW/YW are 0. PerspDeltaXX/YY and PerspStartW are 1.)"]
    #[inline(always)]
    pub fn translate(self) -> &'a mut W {
        self.variant(GEOMETRY_A::TRANSLATE)
    }
    #[doc = "PerspStartX/Y and PerspDeltaXX/YY are used. (PerspDeltaXY/YX/XW/YW are 0. PerspStartW is 1.)"]
    #[inline(always)]
    pub fn scale(self) -> &'a mut W {
        self.variant(GEOMETRY_A::SCALE)
    }
    #[doc = "PerspStartX/Y and PerspDeltaXX/XY/YX/YY are used. (PerspDeltaXW/YW are 0. PerspStartW is 1.)"]
    #[inline(always)]
    pub fn rotation(self) -> &'a mut W {
        self.variant(GEOMETRY_A::ROTATION)
    }
    #[doc = "All values PerspStartX/Y/W and PerspDeltaXX/XY/YX/YY/XW/YW are used"]
    #[inline(always)]
    pub fn perspective(self) -> &'a mut W {
        self.variant(GEOMETRY_A::PERSPECTIVE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects a method how to generate source buffer sample points."]
    #[inline(always)]
    pub fn rastermode(&self) -> RASTERMODE_R {
        RASTERMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Selects function for the frame input port."]
    #[inline(always)]
    pub fn inputselect(&self) -> INPUTSELECT_R {
        INPUTSELECT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Selects a method for horizontal up-sampling of YUV 422/420 input data."]
    #[inline(always)]
    pub fn yuv422upsamplingmode(&self) -> YUV422UPSAMPLINGMODE_R {
        YUV422UPSAMPLINGMODE_R::new(((self.bits >> 5) & 1) != 0)
    }
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
    #[doc = "Bits 20:22 - Use this to select between nearest and bilinear filtering. Only has an effect if rastermode == ARBITRARY or rastermode == PERSPECTIVE or rastermode == AFFINE."]
    #[inline(always)]
    pub fn filtermode(&self) -> FILTERMODE_R {
        FILTERMODE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Control the geomety settings (PerspStartX/Y/W and PerspDeltaXX/XY/YX/YY/XW/YW usage) for the raster mode PERSPECTIVE."]
    #[inline(always)]
    pub fn geometry(&self) -> GEOMETRY_R {
        GEOMETRY_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects a method how to generate source buffer sample points."]
    #[inline(always)]
    #[must_use]
    pub fn rastermode(&mut self) -> RASTERMODE_W<0> {
        RASTERMODE_W::new(self)
    }
    #[doc = "Bits 3:4 - Selects function for the frame input port."]
    #[inline(always)]
    #[must_use]
    pub fn inputselect(&mut self) -> INPUTSELECT_W<3> {
        INPUTSELECT_W::new(self)
    }
    #[doc = "Bit 5 - Selects a method for horizontal up-sampling of YUV 422/420 input data."]
    #[inline(always)]
    #[must_use]
    pub fn yuv422upsamplingmode(&mut self) -> YUV422UPSAMPLINGMODE_W<5> {
        YUV422UPSAMPLINGMODE_W::new(self)
    }
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
    #[doc = "Bits 20:22 - Use this to select between nearest and bilinear filtering. Only has an effect if rastermode == ARBITRARY or rastermode == PERSPECTIVE or rastermode == AFFINE."]
    #[inline(always)]
    #[must_use]
    pub fn filtermode(&mut self) -> FILTERMODE_W<20> {
        FILTERMODE_W::new(self)
    }
    #[doc = "Bits 28:29 - Control the geomety settings (PerspStartX/Y/W and PerspDeltaXX/XY/YX/YY/XW/YW usage) for the raster mode PERSPECTIVE."]
    #[inline(always)]
    #[must_use]
    pub fn geometry(&mut self) -> GEOMETRY_W<28> {
        GEOMETRY_W::new(self)
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
#[doc = "`reset()` method sets CONTROL to value 0x3001_0000"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3001_0000;
}
