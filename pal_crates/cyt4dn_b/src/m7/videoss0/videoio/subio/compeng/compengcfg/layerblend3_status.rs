#[doc = "Register `LAYERBLEND3_STATUS` reader"]
pub struct R(crate::R<LAYERBLEND3_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYERBLEND3_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYERBLEND3_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYERBLEND3_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LAYERBLEND3_SEL` reader - Status of the connection of the layerblend3 module"]
pub type LAYERBLEND3_SEL_R = crate::FieldReader<u8, LAYERBLEND3_SEL_A>;
#[doc = "Status of the connection of the layerblend3 module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LAYERBLEND3_SEL_A {
    #[doc = "1: layerblend3 module is used from extdst0 processing path"]
    EXTDST0 = 1,
    #[doc = "2: layerblend3 module is used from extdst4 processing path"]
    EXTDST4 = 2,
    #[doc = "3: layerblend3 module is used from extdst1 processing path"]
    EXTDST1 = 3,
    #[doc = "4: layerblend3 module is used from extdst5 processing path"]
    EXTDST5 = 4,
    #[doc = "5: layerblend3 module is used from store4 processing path"]
    STORE4 = 5,
    #[doc = "0: layerblend3 module is not used"]
    DISABLE = 0,
}
impl From<LAYERBLEND3_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LAYERBLEND3_SEL_A) -> Self {
        variant as _
    }
}
impl LAYERBLEND3_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LAYERBLEND3_SEL_A> {
        match self.bits {
            1 => Some(LAYERBLEND3_SEL_A::EXTDST0),
            2 => Some(LAYERBLEND3_SEL_A::EXTDST4),
            3 => Some(LAYERBLEND3_SEL_A::EXTDST1),
            4 => Some(LAYERBLEND3_SEL_A::EXTDST5),
            5 => Some(LAYERBLEND3_SEL_A::STORE4),
            0 => Some(LAYERBLEND3_SEL_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXTDST0`"]
    #[inline(always)]
    pub fn is_extdst0(&self) -> bool {
        *self == LAYERBLEND3_SEL_A::EXTDST0
    }
    #[doc = "Checks if the value of the field is `EXTDST4`"]
    #[inline(always)]
    pub fn is_extdst4(&self) -> bool {
        *self == LAYERBLEND3_SEL_A::EXTDST4
    }
    #[doc = "Checks if the value of the field is `EXTDST1`"]
    #[inline(always)]
    pub fn is_extdst1(&self) -> bool {
        *self == LAYERBLEND3_SEL_A::EXTDST1
    }
    #[doc = "Checks if the value of the field is `EXTDST5`"]
    #[inline(always)]
    pub fn is_extdst5(&self) -> bool {
        *self == LAYERBLEND3_SEL_A::EXTDST5
    }
    #[doc = "Checks if the value of the field is `STORE4`"]
    #[inline(always)]
    pub fn is_store4(&self) -> bool {
        *self == LAYERBLEND3_SEL_A::STORE4
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LAYERBLEND3_SEL_A::DISABLE
    }
}
impl R {
    #[doc = "Bits 16:18 - Status of the connection of the layerblend3 module"]
    #[inline(always)]
    pub fn layerblend3_sel(&self) -> LAYERBLEND3_SEL_R {
        LAYERBLEND3_SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "Status information for pixel engine configuration of layerblend3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layerblend3_status](index.html) module"]
pub struct LAYERBLEND3_STATUS_SPEC;
impl crate::RegisterSpec for LAYERBLEND3_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layerblend3_status::R](R) reader structure"]
impl crate::Readable for LAYERBLEND3_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LAYERBLEND3_STATUS to value 0"]
impl crate::Resettable for LAYERBLEND3_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
