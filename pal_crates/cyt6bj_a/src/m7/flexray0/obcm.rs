#[doc = "Register `OBCM` reader"]
pub struct R(crate::R<OBCM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBCM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBCM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBCM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OBCM` writer"]
pub struct W(crate::W<OBCM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OBCM_SPEC>;
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
impl From<crate::W<OBCM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OBCM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RHSS` reader - Read Header Section Shadow 1 = Header section selected for transfer from Message RAM to Output Buffer 0 = Header section is not read"]
pub type RHSS_R = crate::BitReader<RHSS_A>;
#[doc = "Read Header Section Shadow 1 = Header section selected for transfer from Message RAM to Output Buffer 0 = Header section is not read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RHSS_A {
    #[doc = "0: N/A"]
    HEADER_SECTION_NOT_READ = 0,
    #[doc = "1: N/A"]
    HEADER_SECTION_FOR_MSRAM2OBF_TXFR = 1,
}
impl From<RHSS_A> for bool {
    #[inline(always)]
    fn from(variant: RHSS_A) -> Self {
        variant as u8 != 0
    }
}
impl RHSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHSS_A {
        match self.bits {
            false => RHSS_A::HEADER_SECTION_NOT_READ,
            true => RHSS_A::HEADER_SECTION_FOR_MSRAM2OBF_TXFR,
        }
    }
    #[doc = "Checks if the value of the field is `HEADER_SECTION_NOT_READ`"]
    #[inline(always)]
    pub fn is_header_section_not_read(&self) -> bool {
        *self == RHSS_A::HEADER_SECTION_NOT_READ
    }
    #[doc = "Checks if the value of the field is `HEADER_SECTION_FOR_MSRAM2OBF_TXFR`"]
    #[inline(always)]
    pub fn is_header_section_for_msram2obf_txfr(&self) -> bool {
        *self == RHSS_A::HEADER_SECTION_FOR_MSRAM2OBF_TXFR
    }
}
#[doc = "Field `RHSS` writer - Read Header Section Shadow 1 = Header section selected for transfer from Message RAM to Output Buffer 0 = Header section is not read"]
pub type RHSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OBCM_SPEC, RHSS_A, O>;
impl<'a, const O: u8> RHSS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn header_section_not_read(self) -> &'a mut W {
        self.variant(RHSS_A::HEADER_SECTION_NOT_READ)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn header_section_for_msram2obf_txfr(self) -> &'a mut W {
        self.variant(RHSS_A::HEADER_SECTION_FOR_MSRAM2OBF_TXFR)
    }
}
#[doc = "Field `RDSS` reader - Read Data Section Shadow 1 = Data section selected for transfer from Message RAM to Output Buffer 0 = Data section is not read"]
pub type RDSS_R = crate::BitReader<RDSS_A>;
#[doc = "Read Data Section Shadow 1 = Data section selected for transfer from Message RAM to Output Buffer 0 = Data section is not read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDSS_A {
    #[doc = "0: N/A"]
    DATA_SECTION_NOT_READ = 0,
    #[doc = "1: N/A"]
    DATA_SECTION_FOR_MSRAM2OBF_TXFR = 1,
}
impl From<RDSS_A> for bool {
    #[inline(always)]
    fn from(variant: RDSS_A) -> Self {
        variant as u8 != 0
    }
}
impl RDSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDSS_A {
        match self.bits {
            false => RDSS_A::DATA_SECTION_NOT_READ,
            true => RDSS_A::DATA_SECTION_FOR_MSRAM2OBF_TXFR,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_SECTION_NOT_READ`"]
    #[inline(always)]
    pub fn is_data_section_not_read(&self) -> bool {
        *self == RDSS_A::DATA_SECTION_NOT_READ
    }
    #[doc = "Checks if the value of the field is `DATA_SECTION_FOR_MSRAM2OBF_TXFR`"]
    #[inline(always)]
    pub fn is_data_section_for_msram2obf_txfr(&self) -> bool {
        *self == RDSS_A::DATA_SECTION_FOR_MSRAM2OBF_TXFR
    }
}
#[doc = "Field `RDSS` writer - Read Data Section Shadow 1 = Data section selected for transfer from Message RAM to Output Buffer 0 = Data section is not read"]
pub type RDSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OBCM_SPEC, RDSS_A, O>;
impl<'a, const O: u8> RDSS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn data_section_not_read(self) -> &'a mut W {
        self.variant(RDSS_A::DATA_SECTION_NOT_READ)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn data_section_for_msram2obf_txfr(self) -> &'a mut W {
        self.variant(RDSS_A::DATA_SECTION_FOR_MSRAM2OBF_TXFR)
    }
}
#[doc = "Field `RHSH` reader - Read Header Section Host 1 = Header section selected for transfer from Message RAM to Output Buffer 0 = Header section is not read"]
pub type RHSH_R = crate::BitReader<RHSH_A>;
#[doc = "Read Header Section Host 1 = Header section selected for transfer from Message RAM to Output Buffer 0 = Header section is not read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RHSH_A {
    #[doc = "0: N/A"]
    HEADER_SECTION_NOT_READ = 0,
    #[doc = "1: N/A"]
    HEADER_SECTION_FOR_MSRAM2OBF_TXFR = 1,
}
impl From<RHSH_A> for bool {
    #[inline(always)]
    fn from(variant: RHSH_A) -> Self {
        variant as u8 != 0
    }
}
impl RHSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RHSH_A {
        match self.bits {
            false => RHSH_A::HEADER_SECTION_NOT_READ,
            true => RHSH_A::HEADER_SECTION_FOR_MSRAM2OBF_TXFR,
        }
    }
    #[doc = "Checks if the value of the field is `HEADER_SECTION_NOT_READ`"]
    #[inline(always)]
    pub fn is_header_section_not_read(&self) -> bool {
        *self == RHSH_A::HEADER_SECTION_NOT_READ
    }
    #[doc = "Checks if the value of the field is `HEADER_SECTION_FOR_MSRAM2OBF_TXFR`"]
    #[inline(always)]
    pub fn is_header_section_for_msram2obf_txfr(&self) -> bool {
        *self == RHSH_A::HEADER_SECTION_FOR_MSRAM2OBF_TXFR
    }
}
#[doc = "Field `RDSH` reader - Read Data Section Host 1 = Data section selected for transfer from Message RAM to Output Buffer 0 = Data section is not read"]
pub type RDSH_R = crate::BitReader<RDSH_A>;
#[doc = "Read Data Section Host 1 = Data section selected for transfer from Message RAM to Output Buffer 0 = Data section is not read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDSH_A {
    #[doc = "0: N/A"]
    DATA_SECTION_NOT_READ = 0,
    #[doc = "1: N/A"]
    DATA_SECTION_FOR_MSRAM2OBF_TXFR = 1,
}
impl From<RDSH_A> for bool {
    #[inline(always)]
    fn from(variant: RDSH_A) -> Self {
        variant as u8 != 0
    }
}
impl RDSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDSH_A {
        match self.bits {
            false => RDSH_A::DATA_SECTION_NOT_READ,
            true => RDSH_A::DATA_SECTION_FOR_MSRAM2OBF_TXFR,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_SECTION_NOT_READ`"]
    #[inline(always)]
    pub fn is_data_section_not_read(&self) -> bool {
        *self == RDSH_A::DATA_SECTION_NOT_READ
    }
    #[doc = "Checks if the value of the field is `DATA_SECTION_FOR_MSRAM2OBF_TXFR`"]
    #[inline(always)]
    pub fn is_data_section_for_msram2obf_txfr(&self) -> bool {
        *self == RDSH_A::DATA_SECTION_FOR_MSRAM2OBF_TXFR
    }
}
impl R {
    #[doc = "Bit 0 - Read Header Section Shadow 1 = Header section selected for transfer from Message RAM to Output Buffer 0 = Header section is not read"]
    #[inline(always)]
    pub fn rhss(&self) -> RHSS_R {
        RHSS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Data Section Shadow 1 = Data section selected for transfer from Message RAM to Output Buffer 0 = Data section is not read"]
    #[inline(always)]
    pub fn rdss(&self) -> RDSS_R {
        RDSS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Read Header Section Host 1 = Header section selected for transfer from Message RAM to Output Buffer 0 = Header section is not read"]
    #[inline(always)]
    pub fn rhsh(&self) -> RHSH_R {
        RHSH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Read Data Section Host 1 = Data section selected for transfer from Message RAM to Output Buffer 0 = Data section is not read"]
    #[inline(always)]
    pub fn rdsh(&self) -> RDSH_R {
        RDSH_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read Header Section Shadow 1 = Header section selected for transfer from Message RAM to Output Buffer 0 = Header section is not read"]
    #[inline(always)]
    #[must_use]
    pub fn rhss(&mut self) -> RHSS_W<0> {
        RHSS_W::new(self)
    }
    #[doc = "Bit 1 - Read Data Section Shadow 1 = Data section selected for transfer from Message RAM to Output Buffer 0 = Data section is not read"]
    #[inline(always)]
    #[must_use]
    pub fn rdss(&mut self) -> RDSS_W<1> {
        RDSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Buffer Command Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obcm](index.html) module"]
pub struct OBCM_SPEC;
impl crate::RegisterSpec for OBCM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [obcm::R](R) reader structure"]
impl crate::Readable for OBCM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [obcm::W](W) writer structure"]
impl crate::Writable for OBCM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OBCM to value 0"]
impl crate::Resettable for OBCM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
