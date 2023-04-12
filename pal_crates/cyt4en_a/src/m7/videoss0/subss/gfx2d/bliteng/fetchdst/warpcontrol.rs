#[doc = "Register `WARPCONTROL` reader"]
pub struct R(crate::R<WARPCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WARPCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WARPCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WARPCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WARPCONTROL` writer"]
pub struct W(crate::W<WARPCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WARPCONTROL_SPEC>;
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
impl From<crate::W<WARPCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WARPCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WARPBITSPERPIXEL` reader - Number of bits per pixel in the coordinate layer, which is read by another Fetch unit. Has to be 1, 2, 4, 8, 16 or 32."]
pub type WARPBITSPERPIXEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WARPBITSPERPIXEL` writer - Number of bits per pixel in the coordinate layer, which is read by another Fetch unit. Has to be 1, 2, 4, 8, 16 or 32."]
pub type WARPBITSPERPIXEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WARPCONTROL_SPEC, u8, u8, 6, O>;
#[doc = "Field `WARPCOORDINATEMODE` reader - Content of pixel data in the coordinate layer."]
pub type WARPCOORDINATEMODE_R = crate::FieldReader<u8, WARPCOORDINATEMODE_A>;
#[doc = "Content of pixel data in the coordinate layer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WARPCOORDINATEMODE_A {
    #[doc = "0: x and y (sample points)."]
    PNT = 0,
    #[doc = "1: dx and dy (vectors between adjacent sample points)."]
    D_PNT = 1,
    #[doc = "2: ddx and ddy (deltas between adjacent vectors)."]
    DD_PNT = 2,
}
impl From<WARPCOORDINATEMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WARPCOORDINATEMODE_A) -> Self {
        variant as _
    }
}
impl WARPCOORDINATEMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WARPCOORDINATEMODE_A> {
        match self.bits {
            0 => Some(WARPCOORDINATEMODE_A::PNT),
            1 => Some(WARPCOORDINATEMODE_A::D_PNT),
            2 => Some(WARPCOORDINATEMODE_A::DD_PNT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PNT`"]
    #[inline(always)]
    pub fn is_pnt(&self) -> bool {
        *self == WARPCOORDINATEMODE_A::PNT
    }
    #[doc = "Checks if the value of the field is `D_PNT`"]
    #[inline(always)]
    pub fn is_d_pnt(&self) -> bool {
        *self == WARPCOORDINATEMODE_A::D_PNT
    }
    #[doc = "Checks if the value of the field is `DD_PNT`"]
    #[inline(always)]
    pub fn is_dd_pnt(&self) -> bool {
        *self == WARPCOORDINATEMODE_A::DD_PNT
    }
}
#[doc = "Field `WARPCOORDINATEMODE` writer - Content of pixel data in the coordinate layer."]
pub type WARPCOORDINATEMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WARPCONTROL_SPEC, u8, WARPCOORDINATEMODE_A, 2, O>;
impl<'a, const O: u8> WARPCOORDINATEMODE_W<'a, O> {
    #[doc = "x and y (sample points)."]
    #[inline(always)]
    pub fn pnt(self) -> &'a mut W {
        self.variant(WARPCOORDINATEMODE_A::PNT)
    }
    #[doc = "dx and dy (vectors between adjacent sample points)."]
    #[inline(always)]
    pub fn d_pnt(self) -> &'a mut W {
        self.variant(WARPCOORDINATEMODE_A::D_PNT)
    }
    #[doc = "ddx and ddy (deltas between adjacent vectors)."]
    #[inline(always)]
    pub fn dd_pnt(self) -> &'a mut W {
        self.variant(WARPCOORDINATEMODE_A::DD_PNT)
    }
}
#[doc = "Field `WARPSYMMETRICOFFSET` reader - Value 1 enables symmetric range for negative and positive coordinate values by adding an offset of +0.03125 internally to all coordinate input values. Recommended for small coordinate formats in DD_PNT mode."]
pub type WARPSYMMETRICOFFSET_R = crate::BitReader<bool>;
#[doc = "Field `WARPSYMMETRICOFFSET` writer - Value 1 enables symmetric range for negative and positive coordinate values by adding an offset of +0.03125 internally to all coordinate input values. Recommended for small coordinate formats in DD_PNT mode."]
pub type WARPSYMMETRICOFFSET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WARPCONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Number of bits per pixel in the coordinate layer, which is read by another Fetch unit. Has to be 1, 2, 4, 8, 16 or 32."]
    #[inline(always)]
    pub fn warpbitsperpixel(&self) -> WARPBITSPERPIXEL_R {
        WARPBITSPERPIXEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - Content of pixel data in the coordinate layer."]
    #[inline(always)]
    pub fn warpcoordinatemode(&self) -> WARPCOORDINATEMODE_R {
        WARPCOORDINATEMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Value 1 enables symmetric range for negative and positive coordinate values by adding an offset of +0.03125 internally to all coordinate input values. Recommended for small coordinate formats in DD_PNT mode."]
    #[inline(always)]
    pub fn warpsymmetricoffset(&self) -> WARPSYMMETRICOFFSET_R {
        WARPSYMMETRICOFFSET_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of bits per pixel in the coordinate layer, which is read by another Fetch unit. Has to be 1, 2, 4, 8, 16 or 32."]
    #[inline(always)]
    #[must_use]
    pub fn warpbitsperpixel(&mut self) -> WARPBITSPERPIXEL_W<0> {
        WARPBITSPERPIXEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Content of pixel data in the coordinate layer."]
    #[inline(always)]
    #[must_use]
    pub fn warpcoordinatemode(&mut self) -> WARPCOORDINATEMODE_W<8> {
        WARPCOORDINATEMODE_W::new(self)
    }
    #[doc = "Bit 12 - Value 1 enables symmetric range for negative and positive coordinate values by adding an offset of +0.03125 internally to all coordinate input values. Recommended for small coordinate formats in DD_PNT mode."]
    #[inline(always)]
    #[must_use]
    pub fn warpsymmetricoffset(&mut self) -> WARPSYMMETRICOFFSET_W<12> {
        WARPSYMMETRICOFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Warping control options.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [warpcontrol](index.html) module"]
pub struct WARPCONTROL_SPEC;
impl crate::RegisterSpec for WARPCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [warpcontrol::R](R) reader structure"]
impl crate::Readable for WARPCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [warpcontrol::W](W) writer structure"]
impl crate::Writable for WARPCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WARPCONTROL to value 0x20"]
impl crate::Resettable for WARPCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
