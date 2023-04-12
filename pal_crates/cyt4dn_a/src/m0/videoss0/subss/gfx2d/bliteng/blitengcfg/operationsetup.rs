#[doc = "Register `OPERATIONSETUP` reader"]
pub struct R(crate::R<OPERATIONSETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPERATIONSETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPERATIONSETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPERATIONSETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPERATIONSETUP` writer"]
pub struct W(crate::W<OPERATIONSETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPERATIONSETUP_SPEC>;
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
impl From<crate::W<OPERATIONSETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPERATIONSETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DESTINATIONSELECT` reader - Select operation for destination buffer. In IBO the buffer is provided via the FETCHDST in LBO the buffer is provided via internal RGBA slice buffer."]
pub type DESTINATIONSELECT_R = crate::FieldReader<u8, DESTINATIONSELECT_A>;
#[doc = "Select operation for destination buffer. In IBO the buffer is provided via the FETCHDST in LBO the buffer is provided via internal RGBA slice buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DESTINATIONSELECT_A {
    #[doc = "0: Destination buffer is not read, used for FILL or COPY operations."]
    OFF = 0,
    #[doc = "1: Destination is connected to raster operation block (tertary). Only possible in IBO mode."]
    ROP = 1,
    #[doc = "2: Destination is connected to blend operation block."]
    BLEND = 2,
    #[doc = "3: Destination fetch is connected to the external input of the fetch MASK, for planar YUV. Only possible in IBO mode."]
    FETCHMASK = 3,
}
impl From<DESTINATIONSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DESTINATIONSELECT_A) -> Self {
        variant as _
    }
}
impl DESTINATIONSELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DESTINATIONSELECT_A {
        match self.bits {
            0 => DESTINATIONSELECT_A::OFF,
            1 => DESTINATIONSELECT_A::ROP,
            2 => DESTINATIONSELECT_A::BLEND,
            3 => DESTINATIONSELECT_A::FETCHMASK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DESTINATIONSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ROP`"]
    #[inline(always)]
    pub fn is_rop(&self) -> bool {
        *self == DESTINATIONSELECT_A::ROP
    }
    #[doc = "Checks if the value of the field is `BLEND`"]
    #[inline(always)]
    pub fn is_blend(&self) -> bool {
        *self == DESTINATIONSELECT_A::BLEND
    }
    #[doc = "Checks if the value of the field is `FETCHMASK`"]
    #[inline(always)]
    pub fn is_fetchmask(&self) -> bool {
        *self == DESTINATIONSELECT_A::FETCHMASK
    }
}
#[doc = "Field `DESTINATIONSELECT` writer - Select operation for destination buffer. In IBO the buffer is provided via the FETCHDST in LBO the buffer is provided via internal RGBA slice buffer."]
pub type DESTINATIONSELECT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPERATIONSETUP_SPEC, u8, DESTINATIONSELECT_A, 2, O>;
impl<'a, const O: u8> DESTINATIONSELECT_W<'a, O> {
    #[doc = "Destination buffer is not read, used for FILL or COPY operations."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DESTINATIONSELECT_A::OFF)
    }
    #[doc = "Destination is connected to raster operation block (tertary). Only possible in IBO mode."]
    #[inline(always)]
    pub fn rop(self) -> &'a mut W {
        self.variant(DESTINATIONSELECT_A::ROP)
    }
    #[doc = "Destination is connected to blend operation block."]
    #[inline(always)]
    pub fn blend(self) -> &'a mut W {
        self.variant(DESTINATIONSELECT_A::BLEND)
    }
    #[doc = "Destination fetch is connected to the external input of the fetch MASK, for planar YUV. Only possible in IBO mode."]
    #[inline(always)]
    pub fn fetchmask(self) -> &'a mut W {
        self.variant(DESTINATIONSELECT_A::FETCHMASK)
    }
}
#[doc = "Field `MASKSELECT` reader - Select operation for mask buffer. In IBO the buffer is provided via the FETCHMASK; in LBO the buffer is provided via internal ALPHA slice buffer. If the value of both fields this and DestinationSelect is BLEND, OFF is used for this field."]
pub type MASKSELECT_R = crate::FieldReader<u8, MASKSELECT_A>;
#[doc = "Select operation for mask buffer. In IBO the buffer is provided via the FETCHMASK; in LBO the buffer is provided via internal ALPHA slice buffer. If the value of both fields this and DestinationSelect is BLEND, OFF is used for this field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASKSELECT_A {
    #[doc = "0: Mask buffer is not read."]
    OFF = 0,
    #[doc = "1: Mask is connected to raster operation block (secondary). Only possible in IBO mode."]
    ROP = 1,
    #[doc = "2: Mask is connected to blend operation block. Only possible in LBO mode."]
    BLEND = 2,
    #[doc = "3: Mask is connected to the fetch SRC."]
    FETCHSRC = 3,
}
impl From<MASKSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKSELECT_A) -> Self {
        variant as _
    }
}
impl MASKSELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKSELECT_A {
        match self.bits {
            0 => MASKSELECT_A::OFF,
            1 => MASKSELECT_A::ROP,
            2 => MASKSELECT_A::BLEND,
            3 => MASKSELECT_A::FETCHSRC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MASKSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ROP`"]
    #[inline(always)]
    pub fn is_rop(&self) -> bool {
        *self == MASKSELECT_A::ROP
    }
    #[doc = "Checks if the value of the field is `BLEND`"]
    #[inline(always)]
    pub fn is_blend(&self) -> bool {
        *self == MASKSELECT_A::BLEND
    }
    #[doc = "Checks if the value of the field is `FETCHSRC`"]
    #[inline(always)]
    pub fn is_fetchsrc(&self) -> bool {
        *self == MASKSELECT_A::FETCHSRC
    }
}
#[doc = "Field `MASKSELECT` writer - Select operation for mask buffer. In IBO the buffer is provided via the FETCHMASK; in LBO the buffer is provided via internal ALPHA slice buffer. If the value of both fields this and DestinationSelect is BLEND, OFF is used for this field."]
pub type MASKSELECT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPERATIONSETUP_SPEC, u8, MASKSELECT_A, 2, O>;
impl<'a, const O: u8> MASKSELECT_W<'a, O> {
    #[doc = "Mask buffer is not read."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MASKSELECT_A::OFF)
    }
    #[doc = "Mask is connected to raster operation block (secondary). Only possible in IBO mode."]
    #[inline(always)]
    pub fn rop(self) -> &'a mut W {
        self.variant(MASKSELECT_A::ROP)
    }
    #[doc = "Mask is connected to blend operation block. Only possible in LBO mode."]
    #[inline(always)]
    pub fn blend(self) -> &'a mut W {
        self.variant(MASKSELECT_A::BLEND)
    }
    #[doc = "Mask is connected to the fetch SRC."]
    #[inline(always)]
    pub fn fetchsrc(self) -> &'a mut W {
        self.variant(MASKSELECT_A::FETCHSRC)
    }
}
#[doc = "Field `BUFFERSELECT` reader - Line buffer to render into. This field is ignored for IBO render mode."]
pub type BUFFERSELECT_R = crate::BitReader<BUFFERSELECT_A>;
#[doc = "Line buffer to render into. This field is ignored for IBO render mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFFERSELECT_A {
    #[doc = "0: RGBA buffer."]
    RGBA = 0,
    #[doc = "1: MASK buffer."]
    MASK = 1,
}
impl From<BUFFERSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BUFFERSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFFERSELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFFERSELECT_A {
        match self.bits {
            false => BUFFERSELECT_A::RGBA,
            true => BUFFERSELECT_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `RGBA`"]
    #[inline(always)]
    pub fn is_rgba(&self) -> bool {
        *self == BUFFERSELECT_A::RGBA
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == BUFFERSELECT_A::MASK
    }
}
#[doc = "Field `BUFFERSELECT` writer - Line buffer to render into. This field is ignored for IBO render mode."]
pub type BUFFERSELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OPERATIONSETUP_SPEC, BUFFERSELECT_A, O>;
impl<'a, const O: u8> BUFFERSELECT_W<'a, O> {
    #[doc = "RGBA buffer."]
    #[inline(always)]
    pub fn rgba(self) -> &'a mut W {
        self.variant(BUFFERSELECT_A::RGBA)
    }
    #[doc = "MASK buffer."]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(BUFFERSELECT_A::MASK)
    }
}
#[doc = "Field `ENABLEROP` reader - Set 1 to enable ROP module. This field is ignored for LBO render mode."]
pub type ENABLEROP_R = crate::BitReader<bool>;
#[doc = "Field `ENABLEROP` writer - Set 1 to enable ROP module. This field is ignored for LBO render mode."]
pub type ENABLEROP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPERATIONSETUP_SPEC, bool, O>;
#[doc = "Field `ENABLECLUT` reader - Set 1 to enable CLUT. This field is ignored for LBO render mode."]
pub type ENABLECLUT_R = crate::BitReader<bool>;
#[doc = "Field `ENABLECLUT` writer - Set 1 to enable CLUT. This field is ignored for LBO render mode."]
pub type ENABLECLUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPERATIONSETUP_SPEC, bool, O>;
#[doc = "Field `ENABLEMATRIX` reader - Set 1 to enable matrix."]
pub type ENABLEMATRIX_R = crate::BitReader<bool>;
#[doc = "Field `ENABLEMATRIX` writer - Set 1 to enable matrix."]
pub type ENABLEMATRIX_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPERATIONSETUP_SPEC, bool, O>;
#[doc = "Field `ENABLEGPSCALER` reader - Set 1 enable GPScaler. This field is ignored for LBO render mode."]
pub type ENABLEGPSCALER_R = crate::BitReader<bool>;
#[doc = "Field `ENABLEGPSCALER` writer - Set 1 enable GPScaler. This field is ignored for LBO render mode."]
pub type ENABLEGPSCALER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OPERATIONSETUP_SPEC, bool, O>;
#[doc = "Field `ENABLEBLITBLEND` reader - Set 1 enable the BlitBlend module. If this field is set"]
pub type ENABLEBLITBLEND_R = crate::BitReader<bool>;
#[doc = "Field `ENABLEBLITBLEND` writer - Set 1 enable the BlitBlend module. If this field is set"]
pub type ENABLEBLITBLEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OPERATIONSETUP_SPEC, bool, O>;
#[doc = "Field `CONSTANTCOLORFILL` reader - If this field is set, the selected destination (BufferSelect) will be filled with constant color (ConstantColorRed, ConstantColorGreen, ConstantColorBlue, ConstantAlpha) not using any fetch pipeline. Only possible in LBO mode."]
pub type CONSTANTCOLORFILL_R = crate::BitReader<bool>;
#[doc = "Field `CONSTANTCOLORFILL` writer - If this field is set, the selected destination (BufferSelect) will be filled with constant color (ConstantColorRed, ConstantColorGreen, ConstantColorBlue, ConstantAlpha) not using any fetch pipeline. Only possible in LBO mode."]
pub type CONSTANTCOLORFILL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OPERATIONSETUP_SPEC, bool, O>;
#[doc = "Field `USECOPS` reader - Cops are used for the next fetch operation. If the value is zero, CopsAddress is not evaluated. Only possible in LBO mode."]
pub type USECOPS_R = crate::BitReader<bool>;
#[doc = "Field `USECOPS` writer - Cops are used for the next fetch operation. If the value is zero, CopsAddress is not evaluated. Only possible in LBO mode."]
pub type USECOPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPERATIONSETUP_SPEC, bool, O>;
#[doc = "Field `COPSADDRESS` reader - Address to use in Current Object Processing State (COPS) memory (required for decode operations). Only possible in LBO mode."]
pub type COPSADDRESS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COPSADDRESS` writer - Address to use in Current Object Processing State (COPS) memory (required for decode operations). Only possible in LBO mode."]
pub type COPSADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPERATIONSETUP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Select operation for destination buffer. In IBO the buffer is provided via the FETCHDST in LBO the buffer is provided via internal RGBA slice buffer."]
    #[inline(always)]
    pub fn destinationselect(&self) -> DESTINATIONSELECT_R {
        DESTINATIONSELECT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Select operation for mask buffer. In IBO the buffer is provided via the FETCHMASK; in LBO the buffer is provided via internal ALPHA slice buffer. If the value of both fields this and DestinationSelect is BLEND, OFF is used for this field."]
    #[inline(always)]
    pub fn maskselect(&self) -> MASKSELECT_R {
        MASKSELECT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Line buffer to render into. This field is ignored for IBO render mode."]
    #[inline(always)]
    pub fn bufferselect(&self) -> BUFFERSELECT_R {
        BUFFERSELECT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set 1 to enable ROP module. This field is ignored for LBO render mode."]
    #[inline(always)]
    pub fn enablerop(&self) -> ENABLEROP_R {
        ENABLEROP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set 1 to enable CLUT. This field is ignored for LBO render mode."]
    #[inline(always)]
    pub fn enableclut(&self) -> ENABLECLUT_R {
        ENABLECLUT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set 1 to enable matrix."]
    #[inline(always)]
    pub fn enablematrix(&self) -> ENABLEMATRIX_R {
        ENABLEMATRIX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set 1 enable GPScaler. This field is ignored for LBO render mode."]
    #[inline(always)]
    pub fn enablegpscaler(&self) -> ENABLEGPSCALER_R {
        ENABLEGPSCALER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set 1 enable the BlitBlend module. If this field is set"]
    #[inline(always)]
    pub fn enableblitblend(&self) -> ENABLEBLITBLEND_R {
        ENABLEBLITBLEND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If this field is set, the selected destination (BufferSelect) will be filled with constant color (ConstantColorRed, ConstantColorGreen, ConstantColorBlue, ConstantAlpha) not using any fetch pipeline. Only possible in LBO mode."]
    #[inline(always)]
    pub fn constantcolorfill(&self) -> CONSTANTCOLORFILL_R {
        CONSTANTCOLORFILL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Cops are used for the next fetch operation. If the value is zero, CopsAddress is not evaluated. Only possible in LBO mode."]
    #[inline(always)]
    pub fn usecops(&self) -> USECOPS_R {
        USECOPS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Address to use in Current Object Processing State (COPS) memory (required for decode operations). Only possible in LBO mode."]
    #[inline(always)]
    pub fn copsaddress(&self) -> COPSADDRESS_R {
        COPSADDRESS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select operation for destination buffer. In IBO the buffer is provided via the FETCHDST in LBO the buffer is provided via internal RGBA slice buffer."]
    #[inline(always)]
    #[must_use]
    pub fn destinationselect(&mut self) -> DESTINATIONSELECT_W<0> {
        DESTINATIONSELECT_W::new(self)
    }
    #[doc = "Bits 4:5 - Select operation for mask buffer. In IBO the buffer is provided via the FETCHMASK; in LBO the buffer is provided via internal ALPHA slice buffer. If the value of both fields this and DestinationSelect is BLEND, OFF is used for this field."]
    #[inline(always)]
    #[must_use]
    pub fn maskselect(&mut self) -> MASKSELECT_W<4> {
        MASKSELECT_W::new(self)
    }
    #[doc = "Bit 8 - Line buffer to render into. This field is ignored for IBO render mode."]
    #[inline(always)]
    #[must_use]
    pub fn bufferselect(&mut self) -> BUFFERSELECT_W<8> {
        BUFFERSELECT_W::new(self)
    }
    #[doc = "Bit 9 - Set 1 to enable ROP module. This field is ignored for LBO render mode."]
    #[inline(always)]
    #[must_use]
    pub fn enablerop(&mut self) -> ENABLEROP_W<9> {
        ENABLEROP_W::new(self)
    }
    #[doc = "Bit 10 - Set 1 to enable CLUT. This field is ignored for LBO render mode."]
    #[inline(always)]
    #[must_use]
    pub fn enableclut(&mut self) -> ENABLECLUT_W<10> {
        ENABLECLUT_W::new(self)
    }
    #[doc = "Bit 11 - Set 1 to enable matrix."]
    #[inline(always)]
    #[must_use]
    pub fn enablematrix(&mut self) -> ENABLEMATRIX_W<11> {
        ENABLEMATRIX_W::new(self)
    }
    #[doc = "Bit 12 - Set 1 enable GPScaler. This field is ignored for LBO render mode."]
    #[inline(always)]
    #[must_use]
    pub fn enablegpscaler(&mut self) -> ENABLEGPSCALER_W<12> {
        ENABLEGPSCALER_W::new(self)
    }
    #[doc = "Bit 13 - Set 1 enable the BlitBlend module. If this field is set"]
    #[inline(always)]
    #[must_use]
    pub fn enableblitblend(&mut self) -> ENABLEBLITBLEND_W<13> {
        ENABLEBLITBLEND_W::new(self)
    }
    #[doc = "Bit 14 - If this field is set, the selected destination (BufferSelect) will be filled with constant color (ConstantColorRed, ConstantColorGreen, ConstantColorBlue, ConstantAlpha) not using any fetch pipeline. Only possible in LBO mode."]
    #[inline(always)]
    #[must_use]
    pub fn constantcolorfill(&mut self) -> CONSTANTCOLORFILL_W<14> {
        CONSTANTCOLORFILL_W::new(self)
    }
    #[doc = "Bit 15 - Cops are used for the next fetch operation. If the value is zero, CopsAddress is not evaluated. Only possible in LBO mode."]
    #[inline(always)]
    #[must_use]
    pub fn usecops(&mut self) -> USECOPS_W<15> {
        USECOPS_W::new(self)
    }
    #[doc = "Bits 16:23 - Address to use in Current Object Processing State (COPS) memory (required for decode operations). Only possible in LBO mode."]
    #[inline(always)]
    #[must_use]
    pub fn copsaddress(&mut self) -> COPSADDRESS_W<16> {
        COPSADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Setup for rendering operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [operationsetup](index.html) module"]
pub struct OPERATIONSETUP_SPEC;
impl crate::RegisterSpec for OPERATIONSETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [operationsetup::R](R) reader structure"]
impl crate::Readable for OPERATIONSETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [operationsetup::W](W) writer structure"]
impl crate::Writable for OPERATIONSETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPERATIONSETUP to value 0"]
impl crate::Resettable for OPERATIONSETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
