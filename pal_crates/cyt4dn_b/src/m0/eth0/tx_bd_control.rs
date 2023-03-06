#[doc = "Register `TX_BD_CONTROL` reader"]
pub struct R(crate::R<TX_BD_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_BD_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_BD_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_BD_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_BD_CONTROL` writer"]
pub struct W(crate::W<TX_BD_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_BD_CONTROL_SPEC>;
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
impl From<crate::W<TX_BD_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_BD_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BD_TS_MODE` reader - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type TX_BD_TS_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_BD_TS_MODE` writer - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type TX_BD_TS_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_BD_CONTROL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 4:5 - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn tx_bd_ts_mode(&self) -> TX_BD_TS_MODE_R {
        TX_BD_TS_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    #[must_use]
    pub fn tx_bd_ts_mode(&mut self) -> TX_BD_TS_MODE_W<4> {
        TX_BD_TS_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX BD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_bd_control](index.html) module"]
pub struct TX_BD_CONTROL_SPEC;
impl crate::RegisterSpec for TX_BD_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_bd_control::R](R) reader structure"]
impl crate::Readable for TX_BD_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_bd_control::W](W) writer structure"]
impl crate::Writable for TX_BD_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_BD_CONTROL to value 0"]
impl crate::Resettable for TX_BD_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
