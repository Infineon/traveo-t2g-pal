#[doc = "Register `DMCTL` reader"]
pub struct R(crate::R<DMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMCTL` writer"]
pub struct W(crate::W<DMCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMCTL_SPEC>;
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
impl From<crate::W<DMCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDRT` reader - DDR Type. Always write 3'b111 for LPDDR4. Only LPDDR4 is supported. LPDDR4 = 3'b111 LPDDR4"]
pub type DDRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDRT` writer - DDR Type. Always write 3'b111 for LPDDR4. Only LPDDR4 is supported. LPDDR4 = 3'b111 LPDDR4"]
pub type DDRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMCTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DFI_FREQ_RATIO` reader - DFI Frequency Ratio Always write 2'b10 to this register (only frequency ratio 1:4 is supported). DIV_4 = 2'b10 Controller to PHY frequency ratio is 1:4."]
pub type DFI_FREQ_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DFI_FREQ_RATIO` writer - DFI Frequency Ratio Always write 2'b10 to this register (only frequency ratio 1:4 is supported). DIV_4 = 2'b10 Controller to PHY frequency ratio is 1:4."]
pub type DFI_FREQ_RATIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DRAM_BANK_EN` reader - N/A"]
pub type DRAM_BANK_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRAM_BANK_EN` writer - N/A"]
pub type DRAM_BANK_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SWITCH_CLOSE` reader - Force closing open page when Read/Write direction changes. DONT_CLOSE_ON_RW_CHG = 0 Do not close an open page when Read/Write direction changes on the same page. DO_CLOSE_ON_RW_CHG = 1 Close an open page when Read/Write direction changes on the same page."]
pub type SWITCH_CLOSE_R = crate::BitReader<bool>;
#[doc = "Field `SWITCH_CLOSE` writer - Force closing open page when Read/Write direction changes. DONT_CLOSE_ON_RW_CHG = 0 Do not close an open page when Read/Write direction changes on the same page. DO_CLOSE_ON_RW_CHG = 1 Close an open page when Read/Write direction changes on the same page."]
pub type SWITCH_CLOSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMCTL_SPEC, bool, O>;
#[doc = "Field `BANK_POLICY` reader - CLOSE_PAGE = 0 Close Page Policy OPEN_PAGE = 1 Open page policy"]
pub type BANK_POLICY_R = crate::BitReader<bool>;
#[doc = "Field `BANK_POLICY` writer - CLOSE_PAGE = 0 Close Page Policy OPEN_PAGE = 1 Open page policy"]
pub type BANK_POLICY_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMCTL_SPEC, bool, O>;
#[doc = "Field `WR_DBI` reader - Memory Controller Write Databus Inversion (DBI) Mode Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type WR_DBI_R = crate::BitReader<bool>;
#[doc = "Field `WR_DBI` writer - Memory Controller Write Databus Inversion (DBI) Mode Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type WR_DBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMCTL_SPEC, bool, O>;
#[doc = "Field `RD_DBI` reader - Memory Controller Read Databus Inversion (DBI) Mode Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type RD_DBI_R = crate::BitReader<bool>;
#[doc = "Field `RD_DBI` writer - Memory Controller Read Databus Inversion (DBI) Mode Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type RD_DBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMCTL_SPEC, bool, O>;
#[doc = "Field `DRAM_CHAN_EN` reader - N/A"]
pub type DRAM_CHAN_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRAM_CHAN_EN` writer - N/A"]
pub type DRAM_CHAN_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RD_REQ_MIN` reader - Minimum Available Read requests will be served before switching to Write direction"]
pub type RD_REQ_MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_REQ_MIN` writer - Minimum Available Read requests will be served before switching to Write direction"]
pub type RD_REQ_MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `WR_REQ_MIN` reader - Minimum Available Write requests will be served before switching to Read direction"]
pub type WR_REQ_MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_REQ_MIN` writer - Minimum Available Write requests will be served before switching to Read direction"]
pub type WR_REQ_MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CHAN_UNLOCK` reader - Channel unlock. A bank requester FSM can be unlocked while it is blocked by the other in the same port. 0: Disabled 1: Enabled"]
pub type CHAN_UNLOCK_R = crate::BitReader<bool>;
#[doc = "Field `CHAN_UNLOCK` writer - Channel unlock. A bank requester FSM can be unlocked while it is blocked by the other in the same port. 0: Disabled 1: Enabled"]
pub type CHAN_UNLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMCTL_SPEC, bool, O>;
#[doc = "Field `HI_PRI_IMM` reader - High priority immediately serving. The Arbiter will grant for the high priority stream immediately. 0: Disabled 1: Enabled"]
pub type HI_PRI_IMM_R = crate::BitReader<bool>;
#[doc = "Field `HI_PRI_IMM` writer - High priority immediately serving. The Arbiter will grant for the high priority stream immediately. 0: Disabled 1: Enabled"]
pub type HI_PRI_IMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - DDR Type. Always write 3'b111 for LPDDR4. Only LPDDR4 is supported. LPDDR4 = 3'b111 LPDDR4"]
    #[inline(always)]
    pub fn ddrt(&self) -> DDRT_R {
        DDRT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - DFI Frequency Ratio Always write 2'b10 to this register (only frequency ratio 1:4 is supported). DIV_4 = 2'b10 Controller to PHY frequency ratio is 1:4."]
    #[inline(always)]
    pub fn dfi_freq_ratio(&self) -> DFI_FREQ_RATIO_R {
        DFI_FREQ_RATIO_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - N/A"]
    #[inline(always)]
    pub fn dram_bank_en(&self) -> DRAM_BANK_EN_R {
        DRAM_BANK_EN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Force closing open page when Read/Write direction changes. DONT_CLOSE_ON_RW_CHG = 0 Do not close an open page when Read/Write direction changes on the same page. DO_CLOSE_ON_RW_CHG = 1 Close an open page when Read/Write direction changes on the same page."]
    #[inline(always)]
    pub fn switch_close(&self) -> SWITCH_CLOSE_R {
        SWITCH_CLOSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CLOSE_PAGE = 0 Close Page Policy OPEN_PAGE = 1 Open page policy"]
    #[inline(always)]
    pub fn bank_policy(&self) -> BANK_POLICY_R {
        BANK_POLICY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Memory Controller Write Databus Inversion (DBI) Mode Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn wr_dbi(&self) -> WR_DBI_R {
        WR_DBI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Memory Controller Read Databus Inversion (DBI) Mode Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn rd_dbi(&self) -> RD_DBI_R {
        RD_DBI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - N/A"]
    #[inline(always)]
    pub fn dram_chan_en(&self) -> DRAM_CHAN_EN_R {
        DRAM_CHAN_EN_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:20 - Minimum Available Read requests will be served before switching to Write direction"]
    #[inline(always)]
    pub fn rd_req_min(&self) -> RD_REQ_MIN_R {
        RD_REQ_MIN_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:28 - Minimum Available Write requests will be served before switching to Read direction"]
    #[inline(always)]
    pub fn wr_req_min(&self) -> WR_REQ_MIN_R {
        WR_REQ_MIN_R::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bit 29 - Channel unlock. A bank requester FSM can be unlocked while it is blocked by the other in the same port. 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn chan_unlock(&self) -> CHAN_UNLOCK_R {
        CHAN_UNLOCK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - High priority immediately serving. The Arbiter will grant for the high priority stream immediately. 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn hi_pri_imm(&self) -> HI_PRI_IMM_R {
        HI_PRI_IMM_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - DDR Type. Always write 3'b111 for LPDDR4. Only LPDDR4 is supported. LPDDR4 = 3'b111 LPDDR4"]
    #[inline(always)]
    #[must_use]
    pub fn ddrt(&mut self) -> DDRT_W<0> {
        DDRT_W::new(self)
    }
    #[doc = "Bits 3:4 - DFI Frequency Ratio Always write 2'b10 to this register (only frequency ratio 1:4 is supported). DIV_4 = 2'b10 Controller to PHY frequency ratio is 1:4."]
    #[inline(always)]
    #[must_use]
    pub fn dfi_freq_ratio(&mut self) -> DFI_FREQ_RATIO_W<3> {
        DFI_FREQ_RATIO_W::new(self)
    }
    #[doc = "Bits 5:6 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn dram_bank_en(&mut self) -> DRAM_BANK_EN_W<5> {
        DRAM_BANK_EN_W::new(self)
    }
    #[doc = "Bit 7 - Force closing open page when Read/Write direction changes. DONT_CLOSE_ON_RW_CHG = 0 Do not close an open page when Read/Write direction changes on the same page. DO_CLOSE_ON_RW_CHG = 1 Close an open page when Read/Write direction changes on the same page."]
    #[inline(always)]
    #[must_use]
    pub fn switch_close(&mut self) -> SWITCH_CLOSE_W<7> {
        SWITCH_CLOSE_W::new(self)
    }
    #[doc = "Bit 8 - CLOSE_PAGE = 0 Close Page Policy OPEN_PAGE = 1 Open page policy"]
    #[inline(always)]
    #[must_use]
    pub fn bank_policy(&mut self) -> BANK_POLICY_W<8> {
        BANK_POLICY_W::new(self)
    }
    #[doc = "Bit 9 - Memory Controller Write Databus Inversion (DBI) Mode Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn wr_dbi(&mut self) -> WR_DBI_W<9> {
        WR_DBI_W::new(self)
    }
    #[doc = "Bit 10 - Memory Controller Read Databus Inversion (DBI) Mode Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn rd_dbi(&mut self) -> RD_DBI_W<10> {
        RD_DBI_W::new(self)
    }
    #[doc = "Bits 11:12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn dram_chan_en(&mut self) -> DRAM_CHAN_EN_W<11> {
        DRAM_CHAN_EN_W::new(self)
    }
    #[doc = "Bits 13:20 - Minimum Available Read requests will be served before switching to Write direction"]
    #[inline(always)]
    #[must_use]
    pub fn rd_req_min(&mut self) -> RD_REQ_MIN_W<13> {
        RD_REQ_MIN_W::new(self)
    }
    #[doc = "Bits 21:28 - Minimum Available Write requests will be served before switching to Read direction"]
    #[inline(always)]
    #[must_use]
    pub fn wr_req_min(&mut self) -> WR_REQ_MIN_W<21> {
        WR_REQ_MIN_W::new(self)
    }
    #[doc = "Bit 29 - Channel unlock. A bank requester FSM can be unlocked while it is blocked by the other in the same port. 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn chan_unlock(&mut self) -> CHAN_UNLOCK_W<29> {
        CHAN_UNLOCK_W::new(self)
    }
    #[doc = "Bit 30 - High priority immediately serving. The Arbiter will grant for the high priority stream immediately. 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn hi_pri_imm(&mut self) -> HI_PRI_IMM_W<30> {
        HI_PRI_IMM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamo Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmctl](index.html) module"]
pub struct DMCTL_SPEC;
impl crate::RegisterSpec for DMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmctl::R](R) reader structure"]
impl crate::Readable for DMCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmctl::W](W) writer structure"]
impl crate::Writable for DMCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMCTL to value 0x6000_0000"]
impl crate::Resettable for DMCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_0000;
}
