#[doc = "Register `FM_CTL` reader"]
pub struct R(crate::R<FM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FM_CTL` writer"]
pub struct W(crate::W<FM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM_CTL_SPEC>;
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
impl From<crate::W<FM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FM_MODE` reader - Flash macro mode selection: d0: Read/Idle - Normal mode, read array enabled d1: Not Used - the 1st analog POR is done by enable/enable_hv d2 - POR FUR Download - Downloads critical Flash initialization data from OTP (BG, rd, redu, etc....) d3 - POR IRAM MMR Download - Downloads from OTP region the MMR / IRAM into to the 8051 RDL shadows d4 - POR SW Download - Downloads from OTP region the SW code into to the 8051 MCU SRAM d5 - POR Code_Work Prepare - Loads the Code and Work Flash MG's to be ready for user mode operation d6 - Not Used d7 - Program 32b (WORK) - Used as program confirm command for 32 (Work) bits program d8 - Program 64b (CODE) - Used as program confirm command for 64 (Code) bits program d9 - Program 256b (CODE) - Used as program confirm command for 256 (Code) bits program d10: Program Page (CODE) - Used as program confirm command for page program for Code flash d11: Not Used d12 - Sector Erase - Erase for all kinds of sectors (Code/Work/SMS) d13 - Blank check Entry (UBC) d14 - Blank Check Read 32bit (WORK) - Blank check mode d15 - Blank check Exit d16 - Not Used d17 - Erase Suspend - Suspend command to the Erase operation d18 - Erase Resume - Resume command to Erase suspended operation d19 - Not Used d20- Not Used d21- Not Used d22- Not Used d23- Not Used d24- Not Used d25- Not Used d26- Not Used d27- Not Used d28- Not Used d29- Not Used d30: Not Used d31: Not Used"]
pub type FM_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FM_MODE` writer - Flash macro mode selection: d0: Read/Idle - Normal mode, read array enabled d1: Not Used - the 1st analog POR is done by enable/enable_hv d2 - POR FUR Download - Downloads critical Flash initialization data from OTP (BG, rd, redu, etc....) d3 - POR IRAM MMR Download - Downloads from OTP region the MMR / IRAM into to the 8051 RDL shadows d4 - POR SW Download - Downloads from OTP region the SW code into to the 8051 MCU SRAM d5 - POR Code_Work Prepare - Loads the Code and Work Flash MG's to be ready for user mode operation d6 - Not Used d7 - Program 32b (WORK) - Used as program confirm command for 32 (Work) bits program d8 - Program 64b (CODE) - Used as program confirm command for 64 (Code) bits program d9 - Program 256b (CODE) - Used as program confirm command for 256 (Code) bits program d10: Program Page (CODE) - Used as program confirm command for page program for Code flash d11: Not Used d12 - Sector Erase - Erase for all kinds of sectors (Code/Work/SMS) d13 - Blank check Entry (UBC) d14 - Blank Check Read 32bit (WORK) - Blank check mode d15 - Blank check Exit d16 - Not Used d17 - Erase Suspend - Suspend command to the Erase operation d18 - Erase Resume - Resume command to Erase suspended operation d19 - Not Used d20- Not Used d21- Not Used d22- Not Used d23- Not Used d24- Not Used d25- Not Used d26- Not Used d27- Not Used d28- Not Used d29- Not Used d30: Not Used d31: Not Used"]
pub type FM_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FM_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `EMB_START` reader - '0': not active '1': starts the actual embedded operation"]
pub type EMB_START_R = crate::BitReader<bool>;
#[doc = "Field `EMB_START` writer - '0': not active '1': starts the actual embedded operation"]
pub type EMB_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Flash macro mode selection: d0: Read/Idle - Normal mode, read array enabled d1: Not Used - the 1st analog POR is done by enable/enable_hv d2 - POR FUR Download - Downloads critical Flash initialization data from OTP (BG, rd, redu, etc....) d3 - POR IRAM MMR Download - Downloads from OTP region the MMR / IRAM into to the 8051 RDL shadows d4 - POR SW Download - Downloads from OTP region the SW code into to the 8051 MCU SRAM d5 - POR Code_Work Prepare - Loads the Code and Work Flash MG's to be ready for user mode operation d6 - Not Used d7 - Program 32b (WORK) - Used as program confirm command for 32 (Work) bits program d8 - Program 64b (CODE) - Used as program confirm command for 64 (Code) bits program d9 - Program 256b (CODE) - Used as program confirm command for 256 (Code) bits program d10: Program Page (CODE) - Used as program confirm command for page program for Code flash d11: Not Used d12 - Sector Erase - Erase for all kinds of sectors (Code/Work/SMS) d13 - Blank check Entry (UBC) d14 - Blank Check Read 32bit (WORK) - Blank check mode d15 - Blank check Exit d16 - Not Used d17 - Erase Suspend - Suspend command to the Erase operation d18 - Erase Resume - Resume command to Erase suspended operation d19 - Not Used d20- Not Used d21- Not Used d22- Not Used d23- Not Used d24- Not Used d25- Not Used d26- Not Used d27- Not Used d28- Not Used d29- Not Used d30: Not Used d31: Not Used"]
    #[inline(always)]
    pub fn fm_mode(&self) -> FM_MODE_R {
        FM_MODE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - '0': not active '1': starts the actual embedded operation"]
    #[inline(always)]
    pub fn emb_start(&self) -> EMB_START_R {
        EMB_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Flash macro mode selection: d0: Read/Idle - Normal mode, read array enabled d1: Not Used - the 1st analog POR is done by enable/enable_hv d2 - POR FUR Download - Downloads critical Flash initialization data from OTP (BG, rd, redu, etc....) d3 - POR IRAM MMR Download - Downloads from OTP region the MMR / IRAM into to the 8051 RDL shadows d4 - POR SW Download - Downloads from OTP region the SW code into to the 8051 MCU SRAM d5 - POR Code_Work Prepare - Loads the Code and Work Flash MG's to be ready for user mode operation d6 - Not Used d7 - Program 32b (WORK) - Used as program confirm command for 32 (Work) bits program d8 - Program 64b (CODE) - Used as program confirm command for 64 (Code) bits program d9 - Program 256b (CODE) - Used as program confirm command for 256 (Code) bits program d10: Program Page (CODE) - Used as program confirm command for page program for Code flash d11: Not Used d12 - Sector Erase - Erase for all kinds of sectors (Code/Work/SMS) d13 - Blank check Entry (UBC) d14 - Blank Check Read 32bit (WORK) - Blank check mode d15 - Blank check Exit d16 - Not Used d17 - Erase Suspend - Suspend command to the Erase operation d18 - Erase Resume - Resume command to Erase suspended operation d19 - Not Used d20- Not Used d21- Not Used d22- Not Used d23- Not Used d24- Not Used d25- Not Used d26- Not Used d27- Not Used d28- Not Used d29- Not Used d30: Not Used d31: Not Used"]
    #[inline(always)]
    #[must_use]
    pub fn fm_mode(&mut self) -> FM_MODE_W<0> {
        FM_MODE_W::new(self)
    }
    #[doc = "Bit 31 - '0': not active '1': starts the actual embedded operation"]
    #[inline(always)]
    #[must_use]
    pub fn emb_start(&mut self) -> EMB_START_W<31> {
        EMB_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Macro Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_ctl](index.html) module"]
pub struct FM_CTL_SPEC;
impl crate::RegisterSpec for FM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fm_ctl::R](R) reader structure"]
impl crate::Readable for FM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fm_ctl::W](W) writer structure"]
impl crate::Writable for FM_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FM_CTL to value 0"]
impl crate::Resettable for FM_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
