#[doc = "Register `STATIC_CONTROL` reader"]
pub struct R(crate::R<STATIC_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATIC_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATIC_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATIC_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATIC_CONTROL` writer"]
pub struct W(crate::W<STATIC_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATIC_CONTROL_SPEC>;
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
impl From<crate::W<STATIC_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATIC_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0_ENABLED` reader - Enable bit for channel 0"]
pub type CH0_ENABLED_R = crate::BitReader<CH0_ENABLED_A>;
#[doc = "Enable bit for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_ENABLED_A {
    #[doc = "0: Disable"]
    DIS = 0,
    #[doc = "1: Enable"]
    EN = 1,
}
impl From<CH0_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0_ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_ENABLED_A {
        match self.bits {
            false => CH0_ENABLED_A::DIS,
            true => CH0_ENABLED_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CH0_ENABLED_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CH0_ENABLED_A::EN
    }
}
#[doc = "Field `CH0_ENABLED` writer - Enable bit for channel 0"]
pub type CH0_ENABLED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, CH0_ENABLED_A, O>;
impl<'a, const O: u8> CH0_ENABLED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CH0_ENABLED_A::DIS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CH0_ENABLED_A::EN)
    }
}
#[doc = "Field `CH1_ENABLED` reader - Enable bit for channel 1"]
pub type CH1_ENABLED_R = crate::BitReader<CH1_ENABLED_A>;
#[doc = "Enable bit for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_ENABLED_A {
    #[doc = "0: Disable"]
    DIS = 0,
    #[doc = "1: Enable"]
    EN = 1,
}
impl From<CH1_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1_ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_ENABLED_A {
        match self.bits {
            false => CH1_ENABLED_A::DIS,
            true => CH1_ENABLED_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CH1_ENABLED_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CH1_ENABLED_A::EN
    }
}
#[doc = "Field `CH1_ENABLED` writer - Enable bit for channel 1"]
pub type CH1_ENABLED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, CH1_ENABLED_A, O>;
impl<'a, const O: u8> CH1_ENABLED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CH1_ENABLED_A::DIS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CH1_ENABLED_A::EN)
    }
}
#[doc = "Field `CH2_ENABLED` reader - Enable bit for channel 2"]
pub type CH2_ENABLED_R = crate::BitReader<CH2_ENABLED_A>;
#[doc = "Enable bit for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_ENABLED_A {
    #[doc = "0: Disable"]
    DIS = 0,
    #[doc = "1: Enable"]
    EN = 1,
}
impl From<CH2_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2_ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_ENABLED_A {
        match self.bits {
            false => CH2_ENABLED_A::DIS,
            true => CH2_ENABLED_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CH2_ENABLED_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CH2_ENABLED_A::EN
    }
}
#[doc = "Field `CH2_ENABLED` writer - Enable bit for channel 2"]
pub type CH2_ENABLED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, CH2_ENABLED_A, O>;
impl<'a, const O: u8> CH2_ENABLED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CH2_ENABLED_A::DIS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CH2_ENABLED_A::EN)
    }
}
#[doc = "Field `CH3_ENABLED` reader - Enable bit for channel 3"]
pub type CH3_ENABLED_R = crate::BitReader<CH3_ENABLED_A>;
#[doc = "Enable bit for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_ENABLED_A {
    #[doc = "0: Disable"]
    DIS = 0,
    #[doc = "1: Enable"]
    EN = 1,
}
impl From<CH3_ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3_ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_ENABLED_A {
        match self.bits {
            false => CH3_ENABLED_A::DIS,
            true => CH3_ENABLED_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CH3_ENABLED_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CH3_ENABLED_A::EN
    }
}
#[doc = "Field `CH3_ENABLED` writer - Enable bit for channel 3"]
pub type CH3_ENABLED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, CH3_ENABLED_A, O>;
impl<'a, const O: u8> CH3_ENABLED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CH3_ENABLED_A::DIS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CH3_ENABLED_A::EN)
    }
}
#[doc = "Field `MEASUREMENT_EN` reader - Enable measurement of gfxcache. Corresponding fields are CHX_BYPASS, CHX_MISS and CHX_HIT, OVERFLOW and SAVE_AND_RESET_MEASUREMENTS."]
pub type MEASUREMENT_EN_R = crate::BitReader<MEASUREMENT_EN_A>;
#[doc = "Enable measurement of gfxcache. Corresponding fields are CHX_BYPASS, CHX_MISS and CHX_HIT, OVERFLOW and SAVE_AND_RESET_MEASUREMENTS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEASUREMENT_EN_A {
    #[doc = "0: Disable"]
    DIS = 0,
    #[doc = "1: Enable"]
    EN = 1,
}
impl From<MEASUREMENT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MEASUREMENT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MEASUREMENT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEASUREMENT_EN_A {
        match self.bits {
            false => MEASUREMENT_EN_A::DIS,
            true => MEASUREMENT_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MEASUREMENT_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MEASUREMENT_EN_A::EN
    }
}
#[doc = "Field `MEASUREMENT_EN` writer - Enable measurement of gfxcache. Corresponding fields are CHX_BYPASS, CHX_MISS and CHX_HIT, OVERFLOW and SAVE_AND_RESET_MEASUREMENTS."]
pub type MEASUREMENT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, MEASUREMENT_EN_A, O>;
impl<'a, const O: u8> MEASUREMENT_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MEASUREMENT_EN_A::DIS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MEASUREMENT_EN_A::EN)
    }
}
#[doc = "Field `PARTITION_EN` reader - Partition cache ways to two regions"]
pub type PARTITION_EN_R = crate::BitReader<PARTITION_EN_A>;
#[doc = "Partition cache ways to two regions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARTITION_EN_A {
    #[doc = "0: Disable partition. Whole cache is one region"]
    DIS = 0,
    #[doc = "1: Enable partition. Two regions."]
    EN = 1,
}
impl From<PARTITION_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PARTITION_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PARTITION_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARTITION_EN_A {
        match self.bits {
            false => PARTITION_EN_A::DIS,
            true => PARTITION_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PARTITION_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PARTITION_EN_A::EN
    }
}
#[doc = "Field `PARTITION_EN` writer - Partition cache ways to two regions"]
pub type PARTITION_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, PARTITION_EN_A, O>;
impl<'a, const O: u8> PARTITION_EN_W<'a, O> {
    #[doc = "Disable partition. Whole cache is one region"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PARTITION_EN_A::DIS)
    }
    #[doc = "Enable partition. Two regions."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PARTITION_EN_A::EN)
    }
}
#[doc = "Field `PARTITION_MD` reader - Cache ways partition mode"]
pub type PARTITION_MD_R = crate::BitReader<PARTITION_MD_A>;
#[doc = "Cache ways partition mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARTITION_MD_A {
    #[doc = "0: Region1st:Region2nd = 1:1"]
    MODE11 = 0,
    #[doc = "1: Region1st:Region2nd = 3:1"]
    MODE31 = 1,
}
impl From<PARTITION_MD_A> for bool {
    #[inline(always)]
    fn from(variant: PARTITION_MD_A) -> Self {
        variant as u8 != 0
    }
}
impl PARTITION_MD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARTITION_MD_A {
        match self.bits {
            false => PARTITION_MD_A::MODE11,
            true => PARTITION_MD_A::MODE31,
        }
    }
    #[doc = "Checks if the value of the field is `MODE11`"]
    #[inline(always)]
    pub fn is_mode11(&self) -> bool {
        *self == PARTITION_MD_A::MODE11
    }
    #[doc = "Checks if the value of the field is `MODE31`"]
    #[inline(always)]
    pub fn is_mode31(&self) -> bool {
        *self == PARTITION_MD_A::MODE31
    }
}
#[doc = "Field `PARTITION_MD` writer - Cache ways partition mode"]
pub type PARTITION_MD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, PARTITION_MD_A, O>;
impl<'a, const O: u8> PARTITION_MD_W<'a, O> {
    #[doc = "Region1st:Region2nd = 1:1"]
    #[inline(always)]
    pub fn mode11(self) -> &'a mut W {
        self.variant(PARTITION_MD_A::MODE11)
    }
    #[doc = "Region1st:Region2nd = 3:1"]
    #[inline(always)]
    pub fn mode31(self) -> &'a mut W {
        self.variant(PARTITION_MD_A::MODE31)
    }
}
#[doc = "Field `CH0_REGION_SEL` reader - Allocate the cache region for channel 0 (only effective when PARTITION_EN=1)"]
pub type CH0_REGION_SEL_R = crate::BitReader<CH0_REGION_SEL_A>;
#[doc = "Allocate the cache region for channel 0 (only effective when PARTITION_EN=1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_REGION_SEL_A {
    #[doc = "0: The first region"]
    REGION1ST = 0,
    #[doc = "1: The second region"]
    REGION2ND = 1,
}
impl From<CH0_REGION_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_REGION_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0_REGION_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_REGION_SEL_A {
        match self.bits {
            false => CH0_REGION_SEL_A::REGION1ST,
            true => CH0_REGION_SEL_A::REGION2ND,
        }
    }
    #[doc = "Checks if the value of the field is `REGION1ST`"]
    #[inline(always)]
    pub fn is_region1st(&self) -> bool {
        *self == CH0_REGION_SEL_A::REGION1ST
    }
    #[doc = "Checks if the value of the field is `REGION2ND`"]
    #[inline(always)]
    pub fn is_region2nd(&self) -> bool {
        *self == CH0_REGION_SEL_A::REGION2ND
    }
}
#[doc = "Field `CH0_REGION_SEL` writer - Allocate the cache region for channel 0 (only effective when PARTITION_EN=1)"]
pub type CH0_REGION_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, CH0_REGION_SEL_A, O>;
impl<'a, const O: u8> CH0_REGION_SEL_W<'a, O> {
    #[doc = "The first region"]
    #[inline(always)]
    pub fn region1st(self) -> &'a mut W {
        self.variant(CH0_REGION_SEL_A::REGION1ST)
    }
    #[doc = "The second region"]
    #[inline(always)]
    pub fn region2nd(self) -> &'a mut W {
        self.variant(CH0_REGION_SEL_A::REGION2ND)
    }
}
#[doc = "Field `CH1_REGION_SEL` reader - Allocate the cache region for channel 1 (only effective when PARTITION_EN=1)"]
pub type CH1_REGION_SEL_R = crate::BitReader<CH1_REGION_SEL_A>;
#[doc = "Allocate the cache region for channel 1 (only effective when PARTITION_EN=1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_REGION_SEL_A {
    #[doc = "0: The first region"]
    REGION1ST = 0,
    #[doc = "1: The second region"]
    REGION2ND = 1,
}
impl From<CH1_REGION_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_REGION_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1_REGION_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_REGION_SEL_A {
        match self.bits {
            false => CH1_REGION_SEL_A::REGION1ST,
            true => CH1_REGION_SEL_A::REGION2ND,
        }
    }
    #[doc = "Checks if the value of the field is `REGION1ST`"]
    #[inline(always)]
    pub fn is_region1st(&self) -> bool {
        *self == CH1_REGION_SEL_A::REGION1ST
    }
    #[doc = "Checks if the value of the field is `REGION2ND`"]
    #[inline(always)]
    pub fn is_region2nd(&self) -> bool {
        *self == CH1_REGION_SEL_A::REGION2ND
    }
}
#[doc = "Field `CH1_REGION_SEL` writer - Allocate the cache region for channel 1 (only effective when PARTITION_EN=1)"]
pub type CH1_REGION_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, CH1_REGION_SEL_A, O>;
impl<'a, const O: u8> CH1_REGION_SEL_W<'a, O> {
    #[doc = "The first region"]
    #[inline(always)]
    pub fn region1st(self) -> &'a mut W {
        self.variant(CH1_REGION_SEL_A::REGION1ST)
    }
    #[doc = "The second region"]
    #[inline(always)]
    pub fn region2nd(self) -> &'a mut W {
        self.variant(CH1_REGION_SEL_A::REGION2ND)
    }
}
#[doc = "Field `CH2_REGION_SEL` reader - Allocate the cache region for channel 2 (only effective when PARTITION_EN=1)"]
pub type CH2_REGION_SEL_R = crate::BitReader<CH2_REGION_SEL_A>;
#[doc = "Allocate the cache region for channel 2 (only effective when PARTITION_EN=1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_REGION_SEL_A {
    #[doc = "0: The first region"]
    REGION1ST = 0,
    #[doc = "1: The second region"]
    REGION2ND = 1,
}
impl From<CH2_REGION_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_REGION_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2_REGION_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_REGION_SEL_A {
        match self.bits {
            false => CH2_REGION_SEL_A::REGION1ST,
            true => CH2_REGION_SEL_A::REGION2ND,
        }
    }
    #[doc = "Checks if the value of the field is `REGION1ST`"]
    #[inline(always)]
    pub fn is_region1st(&self) -> bool {
        *self == CH2_REGION_SEL_A::REGION1ST
    }
    #[doc = "Checks if the value of the field is `REGION2ND`"]
    #[inline(always)]
    pub fn is_region2nd(&self) -> bool {
        *self == CH2_REGION_SEL_A::REGION2ND
    }
}
#[doc = "Field `CH2_REGION_SEL` writer - Allocate the cache region for channel 2 (only effective when PARTITION_EN=1)"]
pub type CH2_REGION_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, CH2_REGION_SEL_A, O>;
impl<'a, const O: u8> CH2_REGION_SEL_W<'a, O> {
    #[doc = "The first region"]
    #[inline(always)]
    pub fn region1st(self) -> &'a mut W {
        self.variant(CH2_REGION_SEL_A::REGION1ST)
    }
    #[doc = "The second region"]
    #[inline(always)]
    pub fn region2nd(self) -> &'a mut W {
        self.variant(CH2_REGION_SEL_A::REGION2ND)
    }
}
#[doc = "Field `CH3_REGION_SEL` reader - Allocate the cache region for channel 3 (only effective when PARTITION_EN=1)"]
pub type CH3_REGION_SEL_R = crate::BitReader<CH3_REGION_SEL_A>;
#[doc = "Allocate the cache region for channel 3 (only effective when PARTITION_EN=1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_REGION_SEL_A {
    #[doc = "0: The first region"]
    REGION1ST = 0,
    #[doc = "1: The second region"]
    REGION2ND = 1,
}
impl From<CH3_REGION_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_REGION_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3_REGION_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_REGION_SEL_A {
        match self.bits {
            false => CH3_REGION_SEL_A::REGION1ST,
            true => CH3_REGION_SEL_A::REGION2ND,
        }
    }
    #[doc = "Checks if the value of the field is `REGION1ST`"]
    #[inline(always)]
    pub fn is_region1st(&self) -> bool {
        *self == CH3_REGION_SEL_A::REGION1ST
    }
    #[doc = "Checks if the value of the field is `REGION2ND`"]
    #[inline(always)]
    pub fn is_region2nd(&self) -> bool {
        *self == CH3_REGION_SEL_A::REGION2ND
    }
}
#[doc = "Field `CH3_REGION_SEL` writer - Allocate the cache region for channel 3 (only effective when PARTITION_EN=1)"]
pub type CH3_REGION_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATIC_CONTROL_SPEC, CH3_REGION_SEL_A, O>;
impl<'a, const O: u8> CH3_REGION_SEL_W<'a, O> {
    #[doc = "The first region"]
    #[inline(always)]
    pub fn region1st(self) -> &'a mut W {
        self.variant(CH3_REGION_SEL_A::REGION1ST)
    }
    #[doc = "The second region"]
    #[inline(always)]
    pub fn region2nd(self) -> &'a mut W {
        self.variant(CH3_REGION_SEL_A::REGION2ND)
    }
}
impl R {
    #[doc = "Bit 0 - Enable bit for channel 0"]
    #[inline(always)]
    pub fn ch0_enabled(&self) -> CH0_ENABLED_R {
        CH0_ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable bit for channel 1"]
    #[inline(always)]
    pub fn ch1_enabled(&self) -> CH1_ENABLED_R {
        CH1_ENABLED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for channel 2"]
    #[inline(always)]
    pub fn ch2_enabled(&self) -> CH2_ENABLED_R {
        CH2_ENABLED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable bit for channel 3"]
    #[inline(always)]
    pub fn ch3_enabled(&self) -> CH3_ENABLED_R {
        CH3_ENABLED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable measurement of gfxcache. Corresponding fields are CHX_BYPASS, CHX_MISS and CHX_HIT, OVERFLOW and SAVE_AND_RESET_MEASUREMENTS."]
    #[inline(always)]
    pub fn measurement_en(&self) -> MEASUREMENT_EN_R {
        MEASUREMENT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Partition cache ways to two regions"]
    #[inline(always)]
    pub fn partition_en(&self) -> PARTITION_EN_R {
        PARTITION_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Cache ways partition mode"]
    #[inline(always)]
    pub fn partition_md(&self) -> PARTITION_MD_R {
        PARTITION_MD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Allocate the cache region for channel 0 (only effective when PARTITION_EN=1)"]
    #[inline(always)]
    pub fn ch0_region_sel(&self) -> CH0_REGION_SEL_R {
        CH0_REGION_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Allocate the cache region for channel 1 (only effective when PARTITION_EN=1)"]
    #[inline(always)]
    pub fn ch1_region_sel(&self) -> CH1_REGION_SEL_R {
        CH1_REGION_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Allocate the cache region for channel 2 (only effective when PARTITION_EN=1)"]
    #[inline(always)]
    pub fn ch2_region_sel(&self) -> CH2_REGION_SEL_R {
        CH2_REGION_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Allocate the cache region for channel 3 (only effective when PARTITION_EN=1)"]
    #[inline(always)]
    pub fn ch3_region_sel(&self) -> CH3_REGION_SEL_R {
        CH3_REGION_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_enabled(&mut self) -> CH0_ENABLED_W<0> {
        CH0_ENABLED_W::new(self)
    }
    #[doc = "Bit 1 - Enable bit for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enabled(&mut self) -> CH1_ENABLED_W<1> {
        CH1_ENABLED_W::new(self)
    }
    #[doc = "Bit 2 - Enable bit for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_enabled(&mut self) -> CH2_ENABLED_W<2> {
        CH2_ENABLED_W::new(self)
    }
    #[doc = "Bit 3 - Enable bit for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_enabled(&mut self) -> CH3_ENABLED_W<3> {
        CH3_ENABLED_W::new(self)
    }
    #[doc = "Bit 8 - Enable measurement of gfxcache. Corresponding fields are CHX_BYPASS, CHX_MISS and CHX_HIT, OVERFLOW and SAVE_AND_RESET_MEASUREMENTS."]
    #[inline(always)]
    #[must_use]
    pub fn measurement_en(&mut self) -> MEASUREMENT_EN_W<8> {
        MEASUREMENT_EN_W::new(self)
    }
    #[doc = "Bit 12 - Partition cache ways to two regions"]
    #[inline(always)]
    #[must_use]
    pub fn partition_en(&mut self) -> PARTITION_EN_W<12> {
        PARTITION_EN_W::new(self)
    }
    #[doc = "Bit 13 - Cache ways partition mode"]
    #[inline(always)]
    #[must_use]
    pub fn partition_md(&mut self) -> PARTITION_MD_W<13> {
        PARTITION_MD_W::new(self)
    }
    #[doc = "Bit 16 - Allocate the cache region for channel 0 (only effective when PARTITION_EN=1)"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_region_sel(&mut self) -> CH0_REGION_SEL_W<16> {
        CH0_REGION_SEL_W::new(self)
    }
    #[doc = "Bit 17 - Allocate the cache region for channel 1 (only effective when PARTITION_EN=1)"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_region_sel(&mut self) -> CH1_REGION_SEL_W<17> {
        CH1_REGION_SEL_W::new(self)
    }
    #[doc = "Bit 18 - Allocate the cache region for channel 2 (only effective when PARTITION_EN=1)"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_region_sel(&mut self) -> CH2_REGION_SEL_W<18> {
        CH2_REGION_SEL_W::new(self)
    }
    #[doc = "Bit 19 - Allocate the cache region for channel 3 (only effective when PARTITION_EN=1)"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_region_sel(&mut self) -> CH3_REGION_SEL_W<19> {
        CH3_REGION_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Static control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [static_control](index.html) module"]
pub struct STATIC_CONTROL_SPEC;
impl crate::RegisterSpec for STATIC_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [static_control::R](R) reader structure"]
impl crate::Readable for STATIC_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [static_control::W](W) writer structure"]
impl crate::Writable for STATIC_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATIC_CONTROL to value 0"]
impl crate::Resettable for STATIC_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
