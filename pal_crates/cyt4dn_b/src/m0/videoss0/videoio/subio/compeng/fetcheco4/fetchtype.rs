#[doc = "Register `FETCHTYPE` reader"]
pub struct R(crate::R<FETCHTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FETCHTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FETCHTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FETCHTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FETCHTYPE` reader - This field can be used to determine what kind of fetch unit this is."]
pub type FETCHTYPE_R = crate::FieldReader<u8, FETCHTYPE_A>;
#[doc = "This field can be used to determine what kind of fetch unit this is.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FETCHTYPE_A {
    #[doc = "0: Fetch unit with RL and RLAD decoder."]
    DECODE = 0,
    #[doc = "1: Fetch unit with fractional plane (8 layers)."]
    LAYER = 1,
    #[doc = "2: Fetch unit with arbitrary warping and fractional plane (8 layers)."]
    WARP = 2,
    #[doc = "3: Fetch unit with minimum feature set for alpha, chroma and coordinate planes."]
    ECO = 3,
    #[doc = "4: Fetch unit with affine, perspective and arbitrary warping."]
    PERSP = 4,
    #[doc = "5: Fetch unit with affine and arbitrary warping."]
    ROT = 5,
    #[doc = "6: Fetch unit with RL and RLAD decoder, reduced feature set."]
    DECODEL = 6,
    #[doc = "7: Fetch unit with fractional plane (8 layers), reduced feature set."]
    LAYERL = 7,
    #[doc = "8: Fetch unit with affine and arbitrary warping, reduced feature set."]
    ROTL = 8,
    #[doc = "9: Fetch unit with reduced minimum feature set for alpha, chroma and coordinate planes."]
    ECOL = 9,
    #[doc = "10: Fetch unit with DECODE and PERSP capabilities."]
    BLIT = 10,
}
impl From<FETCHTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: FETCHTYPE_A) -> Self {
        variant as _
    }
}
impl FETCHTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FETCHTYPE_A> {
        match self.bits {
            0 => Some(FETCHTYPE_A::DECODE),
            1 => Some(FETCHTYPE_A::LAYER),
            2 => Some(FETCHTYPE_A::WARP),
            3 => Some(FETCHTYPE_A::ECO),
            4 => Some(FETCHTYPE_A::PERSP),
            5 => Some(FETCHTYPE_A::ROT),
            6 => Some(FETCHTYPE_A::DECODEL),
            7 => Some(FETCHTYPE_A::LAYERL),
            8 => Some(FETCHTYPE_A::ROTL),
            9 => Some(FETCHTYPE_A::ECOL),
            10 => Some(FETCHTYPE_A::BLIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DECODE`"]
    #[inline(always)]
    pub fn is_decode(&self) -> bool {
        *self == FETCHTYPE_A::DECODE
    }
    #[doc = "Checks if the value of the field is `LAYER`"]
    #[inline(always)]
    pub fn is_layer(&self) -> bool {
        *self == FETCHTYPE_A::LAYER
    }
    #[doc = "Checks if the value of the field is `WARP`"]
    #[inline(always)]
    pub fn is_warp(&self) -> bool {
        *self == FETCHTYPE_A::WARP
    }
    #[doc = "Checks if the value of the field is `ECO`"]
    #[inline(always)]
    pub fn is_eco(&self) -> bool {
        *self == FETCHTYPE_A::ECO
    }
    #[doc = "Checks if the value of the field is `PERSP`"]
    #[inline(always)]
    pub fn is_persp(&self) -> bool {
        *self == FETCHTYPE_A::PERSP
    }
    #[doc = "Checks if the value of the field is `ROT`"]
    #[inline(always)]
    pub fn is_rot(&self) -> bool {
        *self == FETCHTYPE_A::ROT
    }
    #[doc = "Checks if the value of the field is `DECODEL`"]
    #[inline(always)]
    pub fn is_decodel(&self) -> bool {
        *self == FETCHTYPE_A::DECODEL
    }
    #[doc = "Checks if the value of the field is `LAYERL`"]
    #[inline(always)]
    pub fn is_layerl(&self) -> bool {
        *self == FETCHTYPE_A::LAYERL
    }
    #[doc = "Checks if the value of the field is `ROTL`"]
    #[inline(always)]
    pub fn is_rotl(&self) -> bool {
        *self == FETCHTYPE_A::ROTL
    }
    #[doc = "Checks if the value of the field is `ECOL`"]
    #[inline(always)]
    pub fn is_ecol(&self) -> bool {
        *self == FETCHTYPE_A::ECOL
    }
    #[doc = "Checks if the value of the field is `BLIT`"]
    #[inline(always)]
    pub fn is_blit(&self) -> bool {
        *self == FETCHTYPE_A::BLIT
    }
}
impl R {
    #[doc = "Bits 0:3 - This field can be used to determine what kind of fetch unit this is."]
    #[inline(always)]
    pub fn fetchtype(&self) -> FETCHTYPE_R {
        FETCHTYPE_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Fetch unit type.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fetchtype](index.html) module"]
pub struct FETCHTYPE_SPEC;
impl crate::RegisterSpec for FETCHTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fetchtype::R](R) reader structure"]
impl crate::Readable for FETCHTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FETCHTYPE to value 0"]
impl crate::Resettable for FETCHTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
