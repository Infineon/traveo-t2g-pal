#[doc = "Register `TMG_DATA_CTL` reader"]
pub struct R(crate::R<TMG_DATA_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMG_DATA_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMG_DATA_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMG_DATA_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMG_DATA_CTL` writer"]
pub struct W(crate::W<TMG_DATA_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMG_DATA_CTL_SPEC>;
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
impl From<crate::W<TMG_DATA_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMG_DATA_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S_PRG_RXHS_SETTLE` reader - Settling time to neglect transition effects for data. Ideally in the middle of the min/max range for THS-SETTLE. Depends on the bit rate (UI, unit interval time) and ESC clock period (T_ESC). PRG_RXHS_SETTLE = round (((115 ns + 8*UI) / T_ESC) - 2.5). Examples for 60 MHz ESC clock: 5: 500 Mbps and above 6: 250 Mbps 10: 80 Mbps"]
pub type S_PRG_RXHS_SETTLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S_PRG_RXHS_SETTLE` writer - Settling time to neglect transition effects for data. Ideally in the middle of the min/max range for THS-SETTLE. Depends on the bit rate (UI, unit interval time) and ESC clock period (T_ESC). PRG_RXHS_SETTLE = round (((115 ns + 8*UI) / T_ESC) - 2.5). Examples for 60 MHz ESC clock: 5: 500 Mbps and above 6: 250 Mbps 10: 80 Mbps"]
pub type S_PRG_RXHS_SETTLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TMG_DATA_CTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `S_PRG_HS_PREPARE` reader - N/A"]
pub type S_PRG_HS_PREPARE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S_PRG_HS_PREPARE` writer - N/A"]
pub type S_PRG_HS_PREPARE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TMG_DATA_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `S_PRG_HS_ZERO` reader - N/A"]
pub type S_PRG_HS_ZERO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S_PRG_HS_ZERO` writer - N/A"]
pub type S_PRG_HS_ZERO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TMG_DATA_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `S_PRG_HS_TRAIL` reader - N/A"]
pub type S_PRG_HS_TRAIL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S_PRG_HS_TRAIL` writer - N/A"]
pub type S_PRG_HS_TRAIL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TMG_DATA_CTL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:5 - Settling time to neglect transition effects for data. Ideally in the middle of the min/max range for THS-SETTLE. Depends on the bit rate (UI, unit interval time) and ESC clock period (T_ESC). PRG_RXHS_SETTLE = round (((115 ns + 8*UI) / T_ESC) - 2.5). Examples for 60 MHz ESC clock: 5: 500 Mbps and above 6: 250 Mbps 10: 80 Mbps"]
    #[inline(always)]
    pub fn s_prg_rxhs_settle(&self) -> S_PRG_RXHS_SETTLE_R {
        S_PRG_RXHS_SETTLE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn s_prg_hs_prepare(&self) -> S_PRG_HS_PREPARE_R {
        S_PRG_HS_PREPARE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:20 - N/A"]
    #[inline(always)]
    pub fn s_prg_hs_zero(&self) -> S_PRG_HS_ZERO_R {
        S_PRG_HS_ZERO_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - N/A"]
    #[inline(always)]
    pub fn s_prg_hs_trail(&self) -> S_PRG_HS_TRAIL_R {
        S_PRG_HS_TRAIL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Settling time to neglect transition effects for data. Ideally in the middle of the min/max range for THS-SETTLE. Depends on the bit rate (UI, unit interval time) and ESC clock period (T_ESC). PRG_RXHS_SETTLE = round (((115 ns + 8*UI) / T_ESC) - 2.5). Examples for 60 MHz ESC clock: 5: 500 Mbps and above 6: 250 Mbps 10: 80 Mbps"]
    #[inline(always)]
    #[must_use]
    pub fn s_prg_rxhs_settle(&mut self) -> S_PRG_RXHS_SETTLE_W<0> {
        S_PRG_RXHS_SETTLE_W::new(self)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn s_prg_hs_prepare(&mut self) -> S_PRG_HS_PREPARE_W<8> {
        S_PRG_HS_PREPARE_W::new(self)
    }
    #[doc = "Bits 16:20 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn s_prg_hs_zero(&mut self) -> S_PRG_HS_ZERO_W<16> {
        S_PRG_HS_ZERO_W::new(self)
    }
    #[doc = "Bits 24:28 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn s_prg_hs_trail(&mut self) -> S_PRG_HS_TRAIL_W<24> {
        S_PRG_HS_TRAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Data Lane Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmg_data_ctl](index.html) module"]
pub struct TMG_DATA_CTL_SPEC;
impl crate::RegisterSpec for TMG_DATA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmg_data_ctl::R](R) reader structure"]
impl crate::Readable for TMG_DATA_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmg_data_ctl::W](W) writer structure"]
impl crate::Writable for TMG_DATA_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMG_DATA_CTL to value 0"]
impl crate::Resettable for TMG_DATA_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
