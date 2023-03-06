#[doc = "Register `TX_FIFO_CTL` reader"]
pub struct R(crate::R<TX_FIFO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_FIFO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_FIFO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_FIFO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_FIFO_CTL` writer"]
pub struct W(crate::W<TX_FIFO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_FIFO_CTL_SPEC>;
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
impl From<crate::W<TX_FIFO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_FIFO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated: -INTR.TX_FIFO_TRIGGER = (#FIFO entries &lt; TRIGGER_LEVEL)"]
pub type TRIGGER_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated: -INTR.TX_FIFO_TRIGGER = (#FIFO entries &lt; TRIGGER_LEVEL)"]
pub type TRIGGER_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_FIFO_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `CLEAR` reader - This is a synchronous clear signal to the TX FIFO. When '1', the TX FIFO content are cleared. If a quick clear is required, the field should be set to '1' and followed by '0'. If a clear is required for an extended time, the field should be set to 1 during the complete time."]
pub type CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `CLEAR` writer - This is a synchronous clear signal to the TX FIFO. When '1', the TX FIFO content are cleared. If a quick clear is required, the field should be set to '1' and followed by '0'. If a clear is required for an extended time, the field should be set to 1 during the complete time."]
pub type CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_FIFO_CTL_SPEC, bool, O>;
#[doc = "Field `FREEZE` reader - Freeze functionality: '0': HW uses TX FIFO data and pops the data from the TX FIFO for every HW read. '1': HW read from TX FIFO does not pop the data from the TX FIFO. Note: Freeze functionality is for debug purpose only."]
pub type FREEZE_R = crate::BitReader<bool>;
#[doc = "Field `FREEZE` writer - Freeze functionality: '0': HW uses TX FIFO data and pops the data from the TX FIFO for every HW read. '1': HW read from TX FIFO does not pop the data from the TX FIFO. Note: Freeze functionality is for debug purpose only."]
pub type FREEZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_FIFO_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated: -INTR.TX_FIFO_TRIGGER = (#FIFO entries &lt; TRIGGER_LEVEL)"]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 16 - This is a synchronous clear signal to the TX FIFO. When '1', the TX FIFO content are cleared. If a quick clear is required, the field should be set to '1' and followed by '0'. If a clear is required for an extended time, the field should be set to 1 during the complete time."]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Freeze functionality: '0': HW uses TX FIFO data and pops the data from the TX FIFO for every HW read. '1': HW read from TX FIFO does not pop the data from the TX FIFO. Note: Freeze functionality is for debug purpose only."]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated: -INTR.TX_FIFO_TRIGGER = (#FIFO entries &lt; TRIGGER_LEVEL)"]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W<0> {
        TRIGGER_LEVEL_W::new(self)
    }
    #[doc = "Bit 16 - This is a synchronous clear signal to the TX FIFO. When '1', the TX FIFO content are cleared. If a quick clear is required, the field should be set to '1' and followed by '0'. If a clear is required for an extended time, the field should be set to 1 during the complete time."]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<16> {
        CLEAR_W::new(self)
    }
    #[doc = "Bit 17 - Freeze functionality: '0': HW uses TX FIFO data and pops the data from the TX FIFO for every HW read. '1': HW read from TX FIFO does not pop the data from the TX FIFO. Note: Freeze functionality is for debug purpose only."]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FREEZE_W<17> {
        FREEZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_ctl](index.html) module"]
pub struct TX_FIFO_CTL_SPEC;
impl crate::RegisterSpec for TX_FIFO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_fifo_ctl::R](R) reader structure"]
impl crate::Readable for TX_FIFO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_fifo_ctl::W](W) writer structure"]
impl crate::Writable for TX_FIFO_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_FIFO_CTL to value 0"]
impl crate::Resettable for TX_FIFO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
