#[doc = "Register `TEST2` reader"]
pub struct R(crate::R<TEST2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST2` writer"]
pub struct W(crate::W<TEST2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST2_SPEC>;
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
impl From<crate::W<TEST2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS` reader - RAM Select In RAM Test mode the RAM blocks selected by RS\\[2:0\\]
are mapped to module address 0x400 to 7FF (1024 byte addresses). 000 = Input Buffer RAM 1 (IBF1) 001 = Input Buffer RAM 2 (IBF2) 010 = Output Buffer RAM 1 (OBF1) 011 = Output Buffer RAM 2 (OBF2) 100 = Transient Buffer RAM A (TBF1) 101 = Transient Buffer RAM B (TBF2) 110 = Message RAM (MBF) 111 = unused"]
pub type RS_R = crate::FieldReader<u8, RS_A>;
#[doc = "RAM Select In RAM Test mode the RAM blocks selected by RS\\[2:0\\]
are mapped to module address 0x400 to 7FF (1024 byte addresses). 000 = Input Buffer RAM 1 (IBF1) 001 = Input Buffer RAM 2 (IBF2) 010 = Output Buffer RAM 1 (OBF1) 011 = Output Buffer RAM 2 (OBF2) 100 = Transient Buffer RAM A (TBF1) 101 = Transient Buffer RAM B (TBF2) 110 = Message RAM (MBF) 111 = unused\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RS_A {
    #[doc = "0: N/A"]
    IBF1 = 0,
    #[doc = "1: N/A"]
    IBF2 = 1,
    #[doc = "2: N/A"]
    OBF1 = 2,
    #[doc = "3: N/A"]
    OBF2 = 3,
    #[doc = "4: N/A"]
    TBF1 = 4,
    #[doc = "5: N/A"]
    TBF2 = 5,
    #[doc = "6: N/A"]
    MBF = 6,
}
impl From<RS_A> for u8 {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as _
    }
}
impl RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RS_A> {
        match self.bits {
            0 => Some(RS_A::IBF1),
            1 => Some(RS_A::IBF2),
            2 => Some(RS_A::OBF1),
            3 => Some(RS_A::OBF2),
            4 => Some(RS_A::TBF1),
            5 => Some(RS_A::TBF2),
            6 => Some(RS_A::MBF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IBF1`"]
    #[inline(always)]
    pub fn is_ibf1(&self) -> bool {
        *self == RS_A::IBF1
    }
    #[doc = "Checks if the value of the field is `IBF2`"]
    #[inline(always)]
    pub fn is_ibf2(&self) -> bool {
        *self == RS_A::IBF2
    }
    #[doc = "Checks if the value of the field is `OBF1`"]
    #[inline(always)]
    pub fn is_obf1(&self) -> bool {
        *self == RS_A::OBF1
    }
    #[doc = "Checks if the value of the field is `OBF2`"]
    #[inline(always)]
    pub fn is_obf2(&self) -> bool {
        *self == RS_A::OBF2
    }
    #[doc = "Checks if the value of the field is `TBF1`"]
    #[inline(always)]
    pub fn is_tbf1(&self) -> bool {
        *self == RS_A::TBF1
    }
    #[doc = "Checks if the value of the field is `TBF2`"]
    #[inline(always)]
    pub fn is_tbf2(&self) -> bool {
        *self == RS_A::TBF2
    }
    #[doc = "Checks if the value of the field is `MBF`"]
    #[inline(always)]
    pub fn is_mbf(&self) -> bool {
        *self == RS_A::MBF
    }
}
#[doc = "Field `RS` writer - RAM Select In RAM Test mode the RAM blocks selected by RS\\[2:0\\]
are mapped to module address 0x400 to 7FF (1024 byte addresses). 000 = Input Buffer RAM 1 (IBF1) 001 = Input Buffer RAM 2 (IBF2) 010 = Output Buffer RAM 1 (OBF1) 011 = Output Buffer RAM 2 (OBF2) 100 = Transient Buffer RAM A (TBF1) 101 = Transient Buffer RAM B (TBF2) 110 = Message RAM (MBF) 111 = unused"]
pub type RS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEST2_SPEC, u8, RS_A, 3, O>;
impl<'a, const O: u8> RS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ibf1(self) -> &'a mut W {
        self.variant(RS_A::IBF1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ibf2(self) -> &'a mut W {
        self.variant(RS_A::IBF2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn obf1(self) -> &'a mut W {
        self.variant(RS_A::OBF1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn obf2(self) -> &'a mut W {
        self.variant(RS_A::OBF2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn tbf1(self) -> &'a mut W {
        self.variant(RS_A::TBF1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn tbf2(self) -> &'a mut W {
        self.variant(RS_A::TBF2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn mbf(self) -> &'a mut W {
        self.variant(RS_A::MBF)
    }
}
#[doc = "Field `SSEL` reader - Segment Select To enable access to the complete Message RAM (8192 byte addresses) the Message RAM is segmented. 000 = access to RAM bytes 0000h to 03FFh enabled 001 = access to RAM bytes 0400h to 07FFh enabled 010 = access to RAM bytes 0800h to 0BFFh enabled 011 = access to RAM bytes 0C00h to 0FFFh enabled 100 = access to RAM bytes 1000h to 13FFh enabled 101 = access to RAM bytes 1400h to 17FFh enabled 110 = access to RAM bytes 1800h to 1BFFh enabled 111 = access to RAM bytes 1C00h to 1FFFh enabled"]
pub type SSEL_R = crate::FieldReader<u8, SSEL_A>;
#[doc = "Segment Select To enable access to the complete Message RAM (8192 byte addresses) the Message RAM is segmented. 000 = access to RAM bytes 0000h to 03FFh enabled 001 = access to RAM bytes 0400h to 07FFh enabled 010 = access to RAM bytes 0800h to 0BFFh enabled 011 = access to RAM bytes 0C00h to 0FFFh enabled 100 = access to RAM bytes 1000h to 13FFh enabled 101 = access to RAM bytes 1400h to 17FFh enabled 110 = access to RAM bytes 1800h to 1BFFh enabled 111 = access to RAM bytes 1C00h to 1FFFh enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSEL_A {
    #[doc = "0: N/A"]
    ADDR_0000_TO_03FF_ENABLED = 0,
    #[doc = "1: N/A"]
    ADDR_0400_TO_07FF_ENABLED = 1,
    #[doc = "2: N/A"]
    ADDR_0800_TO_0BFF_ENABLED = 2,
    #[doc = "3: N/A"]
    ADDR_0C00_TO_0FFF_ENABLED = 3,
    #[doc = "4: N/A"]
    ADDR_1000_TO_13FF_ENABLED = 4,
    #[doc = "5: N/A"]
    ADDR_1400_TO_17FF_ENABLED = 5,
    #[doc = "6: N/A"]
    ADDR_1800_TO_1BFF_ENABLED = 6,
    #[doc = "7: N/A"]
    ADDR_1C00_TO_1FFF_ENABLED = 7,
}
impl From<SSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SSEL_A) -> Self {
        variant as _
    }
}
impl SSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSEL_A {
        match self.bits {
            0 => SSEL_A::ADDR_0000_TO_03FF_ENABLED,
            1 => SSEL_A::ADDR_0400_TO_07FF_ENABLED,
            2 => SSEL_A::ADDR_0800_TO_0BFF_ENABLED,
            3 => SSEL_A::ADDR_0C00_TO_0FFF_ENABLED,
            4 => SSEL_A::ADDR_1000_TO_13FF_ENABLED,
            5 => SSEL_A::ADDR_1400_TO_17FF_ENABLED,
            6 => SSEL_A::ADDR_1800_TO_1BFF_ENABLED,
            7 => SSEL_A::ADDR_1C00_TO_1FFF_ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADDR_0000_TO_03FF_ENABLED`"]
    #[inline(always)]
    pub fn is_addr_0000_to_03ff_enabled(&self) -> bool {
        *self == SSEL_A::ADDR_0000_TO_03FF_ENABLED
    }
    #[doc = "Checks if the value of the field is `ADDR_0400_TO_07FF_ENABLED`"]
    #[inline(always)]
    pub fn is_addr_0400_to_07ff_enabled(&self) -> bool {
        *self == SSEL_A::ADDR_0400_TO_07FF_ENABLED
    }
    #[doc = "Checks if the value of the field is `ADDR_0800_TO_0BFF_ENABLED`"]
    #[inline(always)]
    pub fn is_addr_0800_to_0bff_enabled(&self) -> bool {
        *self == SSEL_A::ADDR_0800_TO_0BFF_ENABLED
    }
    #[doc = "Checks if the value of the field is `ADDR_0C00_TO_0FFF_ENABLED`"]
    #[inline(always)]
    pub fn is_addr_0c00_to_0fff_enabled(&self) -> bool {
        *self == SSEL_A::ADDR_0C00_TO_0FFF_ENABLED
    }
    #[doc = "Checks if the value of the field is `ADDR_1000_TO_13FF_ENABLED`"]
    #[inline(always)]
    pub fn is_addr_1000_to_13ff_enabled(&self) -> bool {
        *self == SSEL_A::ADDR_1000_TO_13FF_ENABLED
    }
    #[doc = "Checks if the value of the field is `ADDR_1400_TO_17FF_ENABLED`"]
    #[inline(always)]
    pub fn is_addr_1400_to_17ff_enabled(&self) -> bool {
        *self == SSEL_A::ADDR_1400_TO_17FF_ENABLED
    }
    #[doc = "Checks if the value of the field is `ADDR_1800_TO_1BFF_ENABLED`"]
    #[inline(always)]
    pub fn is_addr_1800_to_1bff_enabled(&self) -> bool {
        *self == SSEL_A::ADDR_1800_TO_1BFF_ENABLED
    }
    #[doc = "Checks if the value of the field is `ADDR_1C00_TO_1FFF_ENABLED`"]
    #[inline(always)]
    pub fn is_addr_1c00_to_1fff_enabled(&self) -> bool {
        *self == SSEL_A::ADDR_1C00_TO_1FFF_ENABLED
    }
}
#[doc = "Field `SSEL` writer - Segment Select To enable access to the complete Message RAM (8192 byte addresses) the Message RAM is segmented. 000 = access to RAM bytes 0000h to 03FFh enabled 001 = access to RAM bytes 0400h to 07FFh enabled 010 = access to RAM bytes 0800h to 0BFFh enabled 011 = access to RAM bytes 0C00h to 0FFFh enabled 100 = access to RAM bytes 1000h to 13FFh enabled 101 = access to RAM bytes 1400h to 17FFh enabled 110 = access to RAM bytes 1800h to 1BFFh enabled 111 = access to RAM bytes 1C00h to 1FFFh enabled"]
pub type SSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TEST2_SPEC, u8, SSEL_A, 3, O>;
impl<'a, const O: u8> SSEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn addr_0000_to_03ff_enabled(self) -> &'a mut W {
        self.variant(SSEL_A::ADDR_0000_TO_03FF_ENABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn addr_0400_to_07ff_enabled(self) -> &'a mut W {
        self.variant(SSEL_A::ADDR_0400_TO_07FF_ENABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn addr_0800_to_0bff_enabled(self) -> &'a mut W {
        self.variant(SSEL_A::ADDR_0800_TO_0BFF_ENABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn addr_0c00_to_0fff_enabled(self) -> &'a mut W {
        self.variant(SSEL_A::ADDR_0C00_TO_0FFF_ENABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn addr_1000_to_13ff_enabled(self) -> &'a mut W {
        self.variant(SSEL_A::ADDR_1000_TO_13FF_ENABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn addr_1400_to_17ff_enabled(self) -> &'a mut W {
        self.variant(SSEL_A::ADDR_1400_TO_17FF_ENABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn addr_1800_to_1bff_enabled(self) -> &'a mut W {
        self.variant(SSEL_A::ADDR_1800_TO_1BFF_ENABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn addr_1c00_to_1fff_enabled(self) -> &'a mut W {
        self.variant(SSEL_A::ADDR_1C00_TO_1FFF_ENABLED)
    }
}
#[doc = "Field `WRPB` reader - Write Parity Bit Value of parity bit to be written to bit 32 of the addressed RAM word."]
pub type WRPB_R = crate::BitReader<bool>;
#[doc = "Field `WRPB` writer - Write Parity Bit Value of parity bit to be written to bit 32 of the addressed RAM word."]
pub type WRPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST2_SPEC, bool, O>;
#[doc = "Field `RDPB` reader - Read Parity Bit Value of parity bit read from bit 32 of the addressed RAM word."]
pub type RDPB_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - RAM Select In RAM Test mode the RAM blocks selected by RS\\[2:0\\]
are mapped to module address 0x400 to 7FF (1024 byte addresses). 000 = Input Buffer RAM 1 (IBF1) 001 = Input Buffer RAM 2 (IBF2) 010 = Output Buffer RAM 1 (OBF1) 011 = Output Buffer RAM 2 (OBF2) 100 = Transient Buffer RAM A (TBF1) 101 = Transient Buffer RAM B (TBF2) 110 = Message RAM (MBF) 111 = unused"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Segment Select To enable access to the complete Message RAM (8192 byte addresses) the Message RAM is segmented. 000 = access to RAM bytes 0000h to 03FFh enabled 001 = access to RAM bytes 0400h to 07FFh enabled 010 = access to RAM bytes 0800h to 0BFFh enabled 011 = access to RAM bytes 0C00h to 0FFFh enabled 100 = access to RAM bytes 1000h to 13FFh enabled 101 = access to RAM bytes 1400h to 17FFh enabled 110 = access to RAM bytes 1800h to 1BFFh enabled 111 = access to RAM bytes 1C00h to 1FFFh enabled"]
    #[inline(always)]
    pub fn ssel(&self) -> SSEL_R {
        SSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 14 - Write Parity Bit Value of parity bit to be written to bit 32 of the addressed RAM word."]
    #[inline(always)]
    pub fn wrpb(&self) -> WRPB_R {
        WRPB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Read Parity Bit Value of parity bit read from bit 32 of the addressed RAM word."]
    #[inline(always)]
    pub fn rdpb(&self) -> RDPB_R {
        RDPB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - RAM Select In RAM Test mode the RAM blocks selected by RS\\[2:0\\]
are mapped to module address 0x400 to 7FF (1024 byte addresses). 000 = Input Buffer RAM 1 (IBF1) 001 = Input Buffer RAM 2 (IBF2) 010 = Output Buffer RAM 1 (OBF1) 011 = Output Buffer RAM 2 (OBF2) 100 = Transient Buffer RAM A (TBF1) 101 = Transient Buffer RAM B (TBF2) 110 = Message RAM (MBF) 111 = unused"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<0> {
        RS_W::new(self)
    }
    #[doc = "Bits 4:6 - Segment Select To enable access to the complete Message RAM (8192 byte addresses) the Message RAM is segmented. 000 = access to RAM bytes 0000h to 03FFh enabled 001 = access to RAM bytes 0400h to 07FFh enabled 010 = access to RAM bytes 0800h to 0BFFh enabled 011 = access to RAM bytes 0C00h to 0FFFh enabled 100 = access to RAM bytes 1000h to 13FFh enabled 101 = access to RAM bytes 1400h to 17FFh enabled 110 = access to RAM bytes 1800h to 1BFFh enabled 111 = access to RAM bytes 1C00h to 1FFFh enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ssel(&mut self) -> SSEL_W<4> {
        SSEL_W::new(self)
    }
    #[doc = "Bit 14 - Write Parity Bit Value of parity bit to be written to bit 32 of the addressed RAM word."]
    #[inline(always)]
    #[must_use]
    pub fn wrpb(&mut self) -> WRPB_W<14> {
        WRPB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test2](index.html) module"]
pub struct TEST2_SPEC;
impl crate::RegisterSpec for TEST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test2::R](R) reader structure"]
impl crate::Readable for TEST2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test2::W](W) writer structure"]
impl crate::Writable for TEST2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST2 to value 0"]
impl crate::Resettable for TEST2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
