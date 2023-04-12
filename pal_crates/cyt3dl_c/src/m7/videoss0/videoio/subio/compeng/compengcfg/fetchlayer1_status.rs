#[doc = "Register `FETCHLAYER1_STATUS` reader"]
pub struct R(crate::R<FETCHLAYER1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FETCHLAYER1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FETCHLAYER1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FETCHLAYER1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FETCHLAYER1_SEL` reader - Status of the connection of the fetchlayer1 module"]
pub type FETCHLAYER1_SEL_R = crate::FieldReader<u8, FETCHLAYER1_SEL_A>;
#[doc = "Status of the connection of the fetchlayer1 module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FETCHLAYER1_SEL_A {
    #[doc = "1: fetchlayer1 module is used from extdst0 processing path"]
    EXTDST0 = 1,
    #[doc = "2: fetchlayer1 module is used from extdst4 processing path"]
    EXTDST4 = 2,
    #[doc = "3: fetchlayer1 module is used from extdst1 processing path"]
    EXTDST1 = 3,
    #[doc = "4: fetchlayer1 module is used from extdst5 processing path"]
    EXTDST5 = 4,
    #[doc = "5: fetchlayer1 module is used from store4 processing path"]
    STORE4 = 5,
    #[doc = "0: fetchlayer1 module is not used"]
    DISABLE = 0,
}
impl From<FETCHLAYER1_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FETCHLAYER1_SEL_A) -> Self {
        variant as _
    }
}
impl FETCHLAYER1_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FETCHLAYER1_SEL_A> {
        match self.bits {
            1 => Some(FETCHLAYER1_SEL_A::EXTDST0),
            2 => Some(FETCHLAYER1_SEL_A::EXTDST4),
            3 => Some(FETCHLAYER1_SEL_A::EXTDST1),
            4 => Some(FETCHLAYER1_SEL_A::EXTDST5),
            5 => Some(FETCHLAYER1_SEL_A::STORE4),
            0 => Some(FETCHLAYER1_SEL_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXTDST0`"]
    #[inline(always)]
    pub fn is_extdst0(&self) -> bool {
        *self == FETCHLAYER1_SEL_A::EXTDST0
    }
    #[doc = "Checks if the value of the field is `EXTDST4`"]
    #[inline(always)]
    pub fn is_extdst4(&self) -> bool {
        *self == FETCHLAYER1_SEL_A::EXTDST4
    }
    #[doc = "Checks if the value of the field is `EXTDST1`"]
    #[inline(always)]
    pub fn is_extdst1(&self) -> bool {
        *self == FETCHLAYER1_SEL_A::EXTDST1
    }
    #[doc = "Checks if the value of the field is `EXTDST5`"]
    #[inline(always)]
    pub fn is_extdst5(&self) -> bool {
        *self == FETCHLAYER1_SEL_A::EXTDST5
    }
    #[doc = "Checks if the value of the field is `STORE4`"]
    #[inline(always)]
    pub fn is_store4(&self) -> bool {
        *self == FETCHLAYER1_SEL_A::STORE4
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FETCHLAYER1_SEL_A::DISABLE
    }
}
impl R {
    #[doc = "Bits 16:18 - Status of the connection of the fetchlayer1 module"]
    #[inline(always)]
    pub fn fetchlayer1_sel(&self) -> FETCHLAYER1_SEL_R {
        FETCHLAYER1_SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "Status information for pixel engine configuration of fetchlayer1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fetchlayer1_status](index.html) module"]
pub struct FETCHLAYER1_STATUS_SPEC;
impl crate::RegisterSpec for FETCHLAYER1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fetchlayer1_status::R](R) reader structure"]
impl crate::Readable for FETCHLAYER1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FETCHLAYER1_STATUS to value 0"]
impl crate::Resettable for FETCHLAYER1_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
