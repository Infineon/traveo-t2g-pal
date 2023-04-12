#[doc = "Register `INTERFACECONFIG` reader"]
pub struct R(crate::R<INTERFACECONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERFACECONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERFACECONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERFACECONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERFACECONFIG` writer"]
pub struct W(crate::W<INTERFACECONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERFACECONFIG_SPEC>;
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
impl From<crate::W<INTERFACECONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERFACECONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERFACETYPE` reader - Describes the type of the external interface."]
pub type INTERFACETYPE_R = crate::FieldReader<u8, INTERFACETYPE_A>;
#[doc = "Describes the type of the external interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTERFACETYPE_A {
    #[doc = "0: Intel 8080 compatible interface (strobe with RD#/WR#)."]
    INTEL80_TYPE1 = 0,
    #[doc = "1: Intel 8080 compatible interface (strobe with CS#)."]
    INTEL80_TYPE2 = 1,
    #[doc = "2: Motorola 6800 compatible interface (strobe with E)."]
    MOTOROLA68_TYPE1 = 2,
    #[doc = "3: Motorola 6800 compatible interface (strobe with CS#)."]
    MOTOROLA68_TYPE2 = 3,
}
impl From<INTERFACETYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: INTERFACETYPE_A) -> Self {
        variant as _
    }
}
impl INTERFACETYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERFACETYPE_A {
        match self.bits {
            0 => INTERFACETYPE_A::INTEL80_TYPE1,
            1 => INTERFACETYPE_A::INTEL80_TYPE2,
            2 => INTERFACETYPE_A::MOTOROLA68_TYPE1,
            3 => INTERFACETYPE_A::MOTOROLA68_TYPE2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INTEL80_TYPE1`"]
    #[inline(always)]
    pub fn is_intel80_type1(&self) -> bool {
        *self == INTERFACETYPE_A::INTEL80_TYPE1
    }
    #[doc = "Checks if the value of the field is `INTEL80_TYPE2`"]
    #[inline(always)]
    pub fn is_intel80_type2(&self) -> bool {
        *self == INTERFACETYPE_A::INTEL80_TYPE2
    }
    #[doc = "Checks if the value of the field is `MOTOROLA68_TYPE1`"]
    #[inline(always)]
    pub fn is_motorola68_type1(&self) -> bool {
        *self == INTERFACETYPE_A::MOTOROLA68_TYPE1
    }
    #[doc = "Checks if the value of the field is `MOTOROLA68_TYPE2`"]
    #[inline(always)]
    pub fn is_motorola68_type2(&self) -> bool {
        *self == INTERFACETYPE_A::MOTOROLA68_TYPE2
    }
}
#[doc = "Field `INTERFACETYPE` writer - Describes the type of the external interface."]
pub type INTERFACETYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, INTERFACECONFIG_SPEC, u8, INTERFACETYPE_A, 2, O>;
impl<'a, const O: u8> INTERFACETYPE_W<'a, O> {
    #[doc = "Intel 8080 compatible interface (strobe with RD#/WR#)."]
    #[inline(always)]
    pub fn intel80_type1(self) -> &'a mut W {
        self.variant(INTERFACETYPE_A::INTEL80_TYPE1)
    }
    #[doc = "Intel 8080 compatible interface (strobe with CS#)."]
    #[inline(always)]
    pub fn intel80_type2(self) -> &'a mut W {
        self.variant(INTERFACETYPE_A::INTEL80_TYPE2)
    }
    #[doc = "Motorola 6800 compatible interface (strobe with E)."]
    #[inline(always)]
    pub fn motorola68_type1(self) -> &'a mut W {
        self.variant(INTERFACETYPE_A::MOTOROLA68_TYPE1)
    }
    #[doc = "Motorola 6800 compatible interface (strobe with CS#)."]
    #[inline(always)]
    pub fn motorola68_type2(self) -> &'a mut W {
        self.variant(INTERFACETYPE_A::MOTOROLA68_TYPE2)
    }
}
#[doc = "Field `INTERFACEWIDTH` reader - Describes the data width of the external interface."]
pub type INTERFACEWIDTH_R = crate::FieldReader<u8, INTERFACEWIDTH_A>;
#[doc = "Describes the data width of the external interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTERFACEWIDTH_A {
    #[doc = "0: 8-bit LCDBus"]
    LCD8 = 0,
    #[doc = "1: 9-bit LCDBus"]
    LCD9 = 1,
    #[doc = "2: 16-bit LCDBus"]
    LCD16 = 2,
    #[doc = "3: 18-bit LCDBus"]
    LCD18 = 3,
}
impl From<INTERFACEWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: INTERFACEWIDTH_A) -> Self {
        variant as _
    }
}
impl INTERFACEWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERFACEWIDTH_A {
        match self.bits {
            0 => INTERFACEWIDTH_A::LCD8,
            1 => INTERFACEWIDTH_A::LCD9,
            2 => INTERFACEWIDTH_A::LCD16,
            3 => INTERFACEWIDTH_A::LCD18,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCD8`"]
    #[inline(always)]
    pub fn is_lcd8(&self) -> bool {
        *self == INTERFACEWIDTH_A::LCD8
    }
    #[doc = "Checks if the value of the field is `LCD9`"]
    #[inline(always)]
    pub fn is_lcd9(&self) -> bool {
        *self == INTERFACEWIDTH_A::LCD9
    }
    #[doc = "Checks if the value of the field is `LCD16`"]
    #[inline(always)]
    pub fn is_lcd16(&self) -> bool {
        *self == INTERFACEWIDTH_A::LCD16
    }
    #[doc = "Checks if the value of the field is `LCD18`"]
    #[inline(always)]
    pub fn is_lcd18(&self) -> bool {
        *self == INTERFACEWIDTH_A::LCD18
    }
}
#[doc = "Field `INTERFACEWIDTH` writer - Describes the data width of the external interface."]
pub type INTERFACEWIDTH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, INTERFACECONFIG_SPEC, u8, INTERFACEWIDTH_A, 2, O>;
impl<'a, const O: u8> INTERFACEWIDTH_W<'a, O> {
    #[doc = "8-bit LCDBus"]
    #[inline(always)]
    pub fn lcd8(self) -> &'a mut W {
        self.variant(INTERFACEWIDTH_A::LCD8)
    }
    #[doc = "9-bit LCDBus"]
    #[inline(always)]
    pub fn lcd9(self) -> &'a mut W {
        self.variant(INTERFACEWIDTH_A::LCD9)
    }
    #[doc = "16-bit LCDBus"]
    #[inline(always)]
    pub fn lcd16(self) -> &'a mut W {
        self.variant(INTERFACEWIDTH_A::LCD16)
    }
    #[doc = "18-bit LCDBus"]
    #[inline(always)]
    pub fn lcd18(self) -> &'a mut W {
        self.variant(INTERFACEWIDTH_A::LCD18)
    }
}
#[doc = "Field `CSPOLARITY` reader - Polarity for chip select output."]
pub type CSPOLARITY_R = crate::BitReader<CSPOLARITY_A>;
#[doc = "Polarity for chip select output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSPOLARITY_A {
    #[doc = "0: CS is a low active signal."]
    ACTIVE_LOW = 0,
    #[doc = "1: CS is a high active signal."]
    ACTIVE_HIGH = 1,
}
impl From<CSPOLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: CSPOLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl CSPOLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSPOLARITY_A {
        match self.bits {
            false => CSPOLARITY_A::ACTIVE_LOW,
            true => CSPOLARITY_A::ACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == CSPOLARITY_A::ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == CSPOLARITY_A::ACTIVE_HIGH
    }
}
#[doc = "Field `CSPOLARITY` writer - Polarity for chip select output."]
pub type CSPOLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERFACECONFIG_SPEC, CSPOLARITY_A, O>;
impl<'a, const O: u8> CSPOLARITY_W<'a, O> {
    #[doc = "CS is a low active signal."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(CSPOLARITY_A::ACTIVE_LOW)
    }
    #[doc = "CS is a high active signal."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(CSPOLARITY_A::ACTIVE_HIGH)
    }
}
#[doc = "Field `WRPOLARITY` reader - Polarity for WR# (8080) or R/W# (6800) otuput."]
pub type WRPOLARITY_R = crate::BitReader<WRPOLARITY_A>;
#[doc = "Polarity for WR# (8080) or R/W# (6800) otuput.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPOLARITY_A {
    #[doc = "0: WR# (R/W#) has default protocol polarity."]
    DEFAULT = 0,
    #[doc = "1: WR# (R/W#) has inverted protocol polarity."]
    INVERTED = 1,
}
impl From<WRPOLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: WRPOLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl WRPOLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRPOLARITY_A {
        match self.bits {
            false => WRPOLARITY_A::DEFAULT,
            true => WRPOLARITY_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == WRPOLARITY_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == WRPOLARITY_A::INVERTED
    }
}
#[doc = "Field `WRPOLARITY` writer - Polarity for WR# (8080) or R/W# (6800) otuput."]
pub type WRPOLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERFACECONFIG_SPEC, WRPOLARITY_A, O>;
impl<'a, const O: u8> WRPOLARITY_W<'a, O> {
    #[doc = "WR# (R/W#) has default protocol polarity."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(WRPOLARITY_A::DEFAULT)
    }
    #[doc = "WR# (R/W#) has inverted protocol polarity."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(WRPOLARITY_A::INVERTED)
    }
}
#[doc = "Field `RDPOLARITY` reader - Polarity for RD# (8080) or E (6800) output."]
pub type RDPOLARITY_R = crate::BitReader<RDPOLARITY_A>;
#[doc = "Polarity for RD# (8080) or E (6800) output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDPOLARITY_A {
    #[doc = "0: RD# (E) has default protocol polarity."]
    DEFAULT = 0,
    #[doc = "1: RD# (E) has inverted protocol polarity."]
    INVERTED = 1,
}
impl From<RDPOLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: RDPOLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl RDPOLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDPOLARITY_A {
        match self.bits {
            false => RDPOLARITY_A::DEFAULT,
            true => RDPOLARITY_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == RDPOLARITY_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RDPOLARITY_A::INVERTED
    }
}
#[doc = "Field `RDPOLARITY` writer - Polarity for RD# (8080) or E (6800) output."]
pub type RDPOLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERFACECONFIG_SPEC, RDPOLARITY_A, O>;
impl<'a, const O: u8> RDPOLARITY_W<'a, O> {
    #[doc = "RD# (E) has default protocol polarity."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(RDPOLARITY_A::DEFAULT)
    }
    #[doc = "RD# (E) has inverted protocol polarity."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RDPOLARITY_A::INVERTED)
    }
}
#[doc = "Field `RSPOLARITY` reader - Polarity of register select output."]
pub type RSPOLARITY_R = crate::BitReader<RSPOLARITY_A>;
#[doc = "Polarity of register select output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPOLARITY_A {
    #[doc = "0: RS is command with low signal level."]
    COMMAND_LOW = 0,
    #[doc = "1: RS is command with high signal level."]
    COMMAND_HIGH = 1,
}
impl From<RSPOLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: RSPOLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPOLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPOLARITY_A {
        match self.bits {
            false => RSPOLARITY_A::COMMAND_LOW,
            true => RSPOLARITY_A::COMMAND_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `COMMAND_LOW`"]
    #[inline(always)]
    pub fn is_command_low(&self) -> bool {
        *self == RSPOLARITY_A::COMMAND_LOW
    }
    #[doc = "Checks if the value of the field is `COMMAND_HIGH`"]
    #[inline(always)]
    pub fn is_command_high(&self) -> bool {
        *self == RSPOLARITY_A::COMMAND_HIGH
    }
}
#[doc = "Field `RSPOLARITY` writer - Polarity of register select output."]
pub type RSPOLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERFACECONFIG_SPEC, RSPOLARITY_A, O>;
impl<'a, const O: u8> RSPOLARITY_W<'a, O> {
    #[doc = "RS is command with low signal level."]
    #[inline(always)]
    pub fn command_low(self) -> &'a mut W {
        self.variant(RSPOLARITY_A::COMMAND_LOW)
    }
    #[doc = "RS is command with high signal level."]
    #[inline(always)]
    pub fn command_high(self) -> &'a mut W {
        self.variant(RSPOLARITY_A::COMMAND_HIGH)
    }
}
#[doc = "Field `DATAPOLARITY` reader - Inversion for data signals upon write."]
pub type DATAPOLARITY_R = crate::BitReader<DATAPOLARITY_A>;
#[doc = "Inversion for data signals upon write.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAPOLARITY_A {
    #[doc = "0: Data bus is used as is."]
    DEFAULT = 0,
    #[doc = "1: Data bus is inverted."]
    INVERTED = 1,
}
impl From<DATAPOLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: DATAPOLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl DATAPOLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAPOLARITY_A {
        match self.bits {
            false => DATAPOLARITY_A::DEFAULT,
            true => DATAPOLARITY_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == DATAPOLARITY_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == DATAPOLARITY_A::INVERTED
    }
}
#[doc = "Field `DATAPOLARITY` writer - Inversion for data signals upon write."]
pub type DATAPOLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERFACECONFIG_SPEC, DATAPOLARITY_A, O>;
impl<'a, const O: u8> DATAPOLARITY_W<'a, O> {
    #[doc = "Data bus is used as is."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(DATAPOLARITY_A::DEFAULT)
    }
    #[doc = "Data bus is inverted."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(DATAPOLARITY_A::INVERTED)
    }
}
#[doc = "Field `TEPOLARITY` reader - Polarity for tearing effect input."]
pub type TEPOLARITY_R = crate::BitReader<TEPOLARITY_A>;
#[doc = "Polarity for tearing effect input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEPOLARITY_A {
    #[doc = "0: TE is a high active signal."]
    ACTIVE_HIGH = 0,
    #[doc = "1: TE is a low active signal."]
    ACTIVE_LOW = 1,
}
impl From<TEPOLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: TEPOLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl TEPOLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEPOLARITY_A {
        match self.bits {
            false => TEPOLARITY_A::ACTIVE_HIGH,
            true => TEPOLARITY_A::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == TEPOLARITY_A::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == TEPOLARITY_A::ACTIVE_LOW
    }
}
#[doc = "Field `TEPOLARITY` writer - Polarity for tearing effect input."]
pub type TEPOLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERFACECONFIG_SPEC, TEPOLARITY_A, O>;
impl<'a, const O: u8> TEPOLARITY_W<'a, O> {
    #[doc = "TE is a high active signal."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(TEPOLARITY_A::ACTIVE_HIGH)
    }
    #[doc = "TE is a low active signal."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(TEPOLARITY_A::ACTIVE_LOW)
    }
}
impl R {
    #[doc = "Bits 0:1 - Describes the type of the external interface."]
    #[inline(always)]
    pub fn interfacetype(&self) -> INTERFACETYPE_R {
        INTERFACETYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Describes the data width of the external interface."]
    #[inline(always)]
    pub fn interfacewidth(&self) -> INTERFACEWIDTH_R {
        INTERFACEWIDTH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Polarity for chip select output."]
    #[inline(always)]
    pub fn cspolarity(&self) -> CSPOLARITY_R {
        CSPOLARITY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Polarity for WR# (8080) or R/W# (6800) otuput."]
    #[inline(always)]
    pub fn wrpolarity(&self) -> WRPOLARITY_R {
        WRPOLARITY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Polarity for RD# (8080) or E (6800) output."]
    #[inline(always)]
    pub fn rdpolarity(&self) -> RDPOLARITY_R {
        RDPOLARITY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Polarity of register select output."]
    #[inline(always)]
    pub fn rspolarity(&self) -> RSPOLARITY_R {
        RSPOLARITY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Inversion for data signals upon write."]
    #[inline(always)]
    pub fn datapolarity(&self) -> DATAPOLARITY_R {
        DATAPOLARITY_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Polarity for tearing effect input."]
    #[inline(always)]
    pub fn tepolarity(&self) -> TEPOLARITY_R {
        TEPOLARITY_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Describes the type of the external interface."]
    #[inline(always)]
    #[must_use]
    pub fn interfacetype(&mut self) -> INTERFACETYPE_W<0> {
        INTERFACETYPE_W::new(self)
    }
    #[doc = "Bits 8:9 - Describes the data width of the external interface."]
    #[inline(always)]
    #[must_use]
    pub fn interfacewidth(&mut self) -> INTERFACEWIDTH_W<8> {
        INTERFACEWIDTH_W::new(self)
    }
    #[doc = "Bit 16 - Polarity for chip select output."]
    #[inline(always)]
    #[must_use]
    pub fn cspolarity(&mut self) -> CSPOLARITY_W<16> {
        CSPOLARITY_W::new(self)
    }
    #[doc = "Bit 17 - Polarity for WR# (8080) or R/W# (6800) otuput."]
    #[inline(always)]
    #[must_use]
    pub fn wrpolarity(&mut self) -> WRPOLARITY_W<17> {
        WRPOLARITY_W::new(self)
    }
    #[doc = "Bit 18 - Polarity for RD# (8080) or E (6800) output."]
    #[inline(always)]
    #[must_use]
    pub fn rdpolarity(&mut self) -> RDPOLARITY_W<18> {
        RDPOLARITY_W::new(self)
    }
    #[doc = "Bit 19 - Polarity of register select output."]
    #[inline(always)]
    #[must_use]
    pub fn rspolarity(&mut self) -> RSPOLARITY_W<19> {
        RSPOLARITY_W::new(self)
    }
    #[doc = "Bit 20 - Inversion for data signals upon write."]
    #[inline(always)]
    #[must_use]
    pub fn datapolarity(&mut self) -> DATAPOLARITY_W<20> {
        DATAPOLARITY_W::new(self)
    }
    #[doc = "Bit 21 - Polarity for tearing effect input."]
    #[inline(always)]
    #[must_use]
    pub fn tepolarity(&mut self) -> TEPOLARITY_W<21> {
        TEPOLARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCDBus interface configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interfaceconfig](index.html) module"]
pub struct INTERFACECONFIG_SPEC;
impl crate::RegisterSpec for INTERFACECONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interfaceconfig::R](R) reader structure"]
impl crate::Readable for INTERFACECONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interfaceconfig::W](W) writer structure"]
impl crate::Writable for INTERFACECONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERFACECONFIG to value 0"]
impl crate::Resettable for INTERFACECONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
