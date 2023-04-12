#[doc = "Register `TTOCF` reader"]
pub struct R(crate::R<TTOCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTOCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTOCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTOCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTOCF` writer"]
pub struct W(crate::W<TTOCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTOCF_SPEC>;
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
impl From<crate::W<TTOCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTOCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OM` reader - Operation Mode 00= Event-driven CAN communication, default 01= TTCAN level 1 10= TTCAN level 2 11= TTCAN level 0"]
pub type OM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OM` writer - Operation Mode 00= Event-driven CAN communication, default 01= TTCAN level 1 10= TTCAN level 2 11= TTCAN level 0"]
pub type OM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTOCF_SPEC, u8, u8, 2, O>;
#[doc = "Field `GEN` reader - Gap Enable 0= Strictly time-triggered operation 1= External event-synchronized time-triggered operation"]
pub type GEN_R = crate::BitReader<bool>;
#[doc = "Field `GEN` writer - Gap Enable 0= Strictly time-triggered operation 1= External event-synchronized time-triggered operation"]
pub type GEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCF_SPEC, bool, O>;
#[doc = "Field `TM` reader - Time Master 0= Time Master function disabled 1= Potential Time Master"]
pub type TM_R = crate::BitReader<bool>;
#[doc = "Field `TM` writer - Time Master 0= Time Master function disabled 1= Potential Time Master"]
pub type TM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCF_SPEC, bool, O>;
#[doc = "Field `LDSDL` reader - LD of Synchronization Deviation Limit The Synchronization Deviation Limit SDL is configured by its dual logarithm LDSDL with SDL = 2(LDSDL + 5). It should not exceed the clock tolerance given by the CAN bit timing configuration. 0x0-7 LD of Synchronization Deviation Limit (SDL &lt;= 32...4096)"]
pub type LDSDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LDSDL` writer - LD of Synchronization Deviation Limit The Synchronization Deviation Limit SDL is configured by its dual logarithm LDSDL with SDL = 2(LDSDL + 5). It should not exceed the clock tolerance given by the CAN bit timing configuration. 0x0-7 LD of Synchronization Deviation Limit (SDL &lt;= 32...4096)"]
pub type LDSDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTOCF_SPEC, u8, u8, 3, O>;
#[doc = "Field `IRTO` reader - Initial Reference Trigger Offset 0x00-7F Positive offset, range from 0 to 127"]
pub type IRTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRTO` writer - Initial Reference Trigger Offset 0x00-7F Positive offset, range from 0 to 127"]
pub type IRTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTOCF_SPEC, u8, u8, 7, O>;
#[doc = "Field `EECS` reader - Enable External Clock Synchronization If enabled, TUR configuration (TURCF.NCL only) may be updated during TTCAN operation. 0= External clock synchronization in TTCAN Level 0,2 disabled 1= External clock synchronization in TTCAN Level 0,2 enabled"]
pub type EECS_R = crate::BitReader<bool>;
#[doc = "Field `EECS` writer - Enable External Clock Synchronization If enabled, TUR configuration (TURCF.NCL only) may be updated during TTCAN operation. 0= External clock synchronization in TTCAN Level 0,2 disabled 1= External clock synchronization in TTCAN Level 0,2 enabled"]
pub type EECS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCF_SPEC, bool, O>;
#[doc = "Field `AWL` reader - Application Watchdog Limit The application watchdog can be disabled by programming AWL to 0x00. 0x00-FF Maximum time after which the application has to serve the application watchdog. The application watchdog is incremented once each 256 NTUs."]
pub type AWL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWL` writer - Application Watchdog Limit The application watchdog can be disabled by programming AWL to 0x00. 0x00-FF Maximum time after which the application has to serve the application watchdog. The application watchdog is incremented once each 256 NTUs."]
pub type AWL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTOCF_SPEC, u8, u8, 8, O>;
#[doc = "Field `EGTF` reader - Enable Global Time Filtering 0= Global time filtering in TTCAN Level 0,2 is disabled 1= Global time filtering in TTCAN Level 0,2 is enabled"]
pub type EGTF_R = crate::BitReader<bool>;
#[doc = "Field `EGTF` writer - Enable Global Time Filtering 0= Global time filtering in TTCAN Level 0,2 is disabled 1= Global time filtering in TTCAN Level 0,2 is enabled"]
pub type EGTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCF_SPEC, bool, O>;
#[doc = "Field `ECC` reader - Enable Clock Calibration 0= Automatic clock calibration in TTCAN Level 0,2 is disabled 1= Automatic clock calibration in TTCAN Level 0,2 is enabled"]
pub type ECC_R = crate::BitReader<bool>;
#[doc = "Field `ECC` writer - Enable Clock Calibration 0= Automatic clock calibration in TTCAN Level 0,2 is disabled 1= Automatic clock calibration in TTCAN Level 0,2 is enabled"]
pub type ECC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCF_SPEC, bool, O>;
#[doc = "Field `EVTP` reader - Event Trigger Polarity 0= Rising edge trigger 1= Falling edge trigger"]
pub type EVTP_R = crate::BitReader<bool>;
#[doc = "Field `EVTP` writer - Event Trigger Polarity 0= Rising edge trigger 1= Falling edge trigger"]
pub type EVTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Operation Mode 00= Event-driven CAN communication, default 01= TTCAN level 1 10= TTCAN level 2 11= TTCAN level 0"]
    #[inline(always)]
    pub fn om(&self) -> OM_R {
        OM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Gap Enable 0= Strictly time-triggered operation 1= External event-synchronized time-triggered operation"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Time Master 0= Time Master function disabled 1= Potential Time Master"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - LD of Synchronization Deviation Limit The Synchronization Deviation Limit SDL is configured by its dual logarithm LDSDL with SDL = 2(LDSDL + 5). It should not exceed the clock tolerance given by the CAN bit timing configuration. 0x0-7 LD of Synchronization Deviation Limit (SDL &lt;= 32...4096)"]
    #[inline(always)]
    pub fn ldsdl(&self) -> LDSDL_R {
        LDSDL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:14 - Initial Reference Trigger Offset 0x00-7F Positive offset, range from 0 to 127"]
    #[inline(always)]
    pub fn irto(&self) -> IRTO_R {
        IRTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Enable External Clock Synchronization If enabled, TUR configuration (TURCF.NCL only) may be updated during TTCAN operation. 0= External clock synchronization in TTCAN Level 0,2 disabled 1= External clock synchronization in TTCAN Level 0,2 enabled"]
    #[inline(always)]
    pub fn eecs(&self) -> EECS_R {
        EECS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Application Watchdog Limit The application watchdog can be disabled by programming AWL to 0x00. 0x00-FF Maximum time after which the application has to serve the application watchdog. The application watchdog is incremented once each 256 NTUs."]
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable Global Time Filtering 0= Global time filtering in TTCAN Level 0,2 is disabled 1= Global time filtering in TTCAN Level 0,2 is enabled"]
    #[inline(always)]
    pub fn egtf(&self) -> EGTF_R {
        EGTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Clock Calibration 0= Automatic clock calibration in TTCAN Level 0,2 is disabled 1= Automatic clock calibration in TTCAN Level 0,2 is enabled"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Event Trigger Polarity 0= Rising edge trigger 1= Falling edge trigger"]
    #[inline(always)]
    pub fn evtp(&self) -> EVTP_R {
        EVTP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operation Mode 00= Event-driven CAN communication, default 01= TTCAN level 1 10= TTCAN level 2 11= TTCAN level 0"]
    #[inline(always)]
    #[must_use]
    pub fn om(&mut self) -> OM_W<0> {
        OM_W::new(self)
    }
    #[doc = "Bit 3 - Gap Enable 0= Strictly time-triggered operation 1= External event-synchronized time-triggered operation"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<3> {
        GEN_W::new(self)
    }
    #[doc = "Bit 4 - Time Master 0= Time Master function disabled 1= Potential Time Master"]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<4> {
        TM_W::new(self)
    }
    #[doc = "Bits 5:7 - LD of Synchronization Deviation Limit The Synchronization Deviation Limit SDL is configured by its dual logarithm LDSDL with SDL = 2(LDSDL + 5). It should not exceed the clock tolerance given by the CAN bit timing configuration. 0x0-7 LD of Synchronization Deviation Limit (SDL &lt;= 32...4096)"]
    #[inline(always)]
    #[must_use]
    pub fn ldsdl(&mut self) -> LDSDL_W<5> {
        LDSDL_W::new(self)
    }
    #[doc = "Bits 8:14 - Initial Reference Trigger Offset 0x00-7F Positive offset, range from 0 to 127"]
    #[inline(always)]
    #[must_use]
    pub fn irto(&mut self) -> IRTO_W<8> {
        IRTO_W::new(self)
    }
    #[doc = "Bit 15 - Enable External Clock Synchronization If enabled, TUR configuration (TURCF.NCL only) may be updated during TTCAN operation. 0= External clock synchronization in TTCAN Level 0,2 disabled 1= External clock synchronization in TTCAN Level 0,2 enabled"]
    #[inline(always)]
    #[must_use]
    pub fn eecs(&mut self) -> EECS_W<15> {
        EECS_W::new(self)
    }
    #[doc = "Bits 16:23 - Application Watchdog Limit The application watchdog can be disabled by programming AWL to 0x00. 0x00-FF Maximum time after which the application has to serve the application watchdog. The application watchdog is incremented once each 256 NTUs."]
    #[inline(always)]
    #[must_use]
    pub fn awl(&mut self) -> AWL_W<16> {
        AWL_W::new(self)
    }
    #[doc = "Bit 24 - Enable Global Time Filtering 0= Global time filtering in TTCAN Level 0,2 is disabled 1= Global time filtering in TTCAN Level 0,2 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn egtf(&mut self) -> EGTF_W<24> {
        EGTF_W::new(self)
    }
    #[doc = "Bit 25 - Enable Clock Calibration 0= Automatic clock calibration in TTCAN Level 0,2 is disabled 1= Automatic clock calibration in TTCAN Level 0,2 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ecc(&mut self) -> ECC_W<25> {
        ECC_W::new(self)
    }
    #[doc = "Bit 26 - Event Trigger Polarity 0= Rising edge trigger 1= Falling edge trigger"]
    #[inline(always)]
    #[must_use]
    pub fn evtp(&mut self) -> EVTP_W<26> {
        EVTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TT Operation Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttocf](index.html) module"]
pub struct TTOCF_SPEC;
impl crate::RegisterSpec for TTOCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttocf::R](R) reader structure"]
impl crate::Readable for TTOCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttocf::W](W) writer structure"]
impl crate::Writable for TTOCF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTOCF to value 0x0001_0000"]
impl crate::Resettable for TTOCF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
