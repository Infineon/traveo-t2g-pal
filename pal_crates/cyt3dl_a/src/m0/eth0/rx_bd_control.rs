#[doc = "Register `RX_BD_CONTROL` reader"]
pub struct R(crate::R<RX_BD_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_BD_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_BD_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_BD_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_BD_CONTROL` writer"]
pub struct W(crate::W<RX_BD_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_BD_CONTROL_SPEC>;
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
impl From<crate::W<RX_BD_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_BD_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_BD_TS_MODE` reader - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type RX_BD_TS_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_BD_TS_MODE` writer - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type RX_BD_TS_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_BD_CONTROL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 4:5 - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn rx_bd_ts_mode(&self) -> RX_BD_TS_MODE_R {
        RX_BD_TS_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    #[must_use]
    pub fn rx_bd_ts_mode(&mut self) -> RX_BD_TS_MODE_W<4> {
        RX_BD_TS_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX BD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_bd_control](index.html) module"]
pub struct RX_BD_CONTROL_SPEC;
impl crate::RegisterSpec for RX_BD_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_bd_control::R](R) reader structure"]
impl crate::Readable for RX_BD_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_bd_control::W](W) writer structure"]
impl crate::Writable for RX_BD_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_BD_CONTROL to value 0"]
impl crate::Resettable for RX_BD_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
