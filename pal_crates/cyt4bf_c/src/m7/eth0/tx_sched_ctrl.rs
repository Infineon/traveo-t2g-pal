#[doc = "Register `TX_SCHED_CTRL` reader"]
pub struct R(crate::R<TX_SCHED_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_SCHED_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_SCHED_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_SCHED_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_SCHED_CTRL` writer"]
pub struct W(crate::W<TX_SCHED_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_SCHED_CTRL_SPEC>;
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
impl From<crate::W<TX_SCHED_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_SCHED_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_SCHED_Q0` reader - 'Queue 0 selection. 00 : Fixed Priority 01 : CBS Enabled only valid for top two enabled queues and if CBS capability selected. 10 : DWRR Enabled 11 : ETS Enabled'"]
pub type TX_SCHED_Q0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_SCHED_Q0` writer - 'Queue 0 selection. 00 : Fixed Priority 01 : CBS Enabled only valid for top two enabled queues and if CBS capability selected. 10 : DWRR Enabled 11 : ETS Enabled'"]
pub type TX_SCHED_Q0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_SCHED_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_SCHED_Q1` reader - 'Queue 1 selection. 00 : Fixed Priority 01 : CBS Enabled only valid for top two enabled queues and if CBS capability selected. 10 : DWRR Enabled 11 : ETS Enabled'"]
pub type TX_SCHED_Q1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_SCHED_Q1` writer - 'Queue 1 selection. 00 : Fixed Priority 01 : CBS Enabled only valid for top two enabled queues and if CBS capability selected. 10 : DWRR Enabled 11 : ETS Enabled'"]
pub type TX_SCHED_Q1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_SCHED_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_SCHED_Q2` reader - 'Queue 2 selection. 00 : Fixed Priority 01 : CBS Enabled only valid for top two enabled queues and if CBS capability selected. 10 : DWRR Enabled 11 : ETS Enabled"]
pub type TX_SCHED_Q2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_SCHED_Q2` writer - 'Queue 2 selection. 00 : Fixed Priority 01 : CBS Enabled only valid for top two enabled queues and if CBS capability selected. 10 : DWRR Enabled 11 : ETS Enabled"]
pub type TX_SCHED_Q2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_SCHED_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_SCHED_Q3` reader - Write ignore, read 0"]
pub type TX_SCHED_Q3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REMOVED_31_8` reader - Write ignore, read 0"]
pub type REMOVED_31_8_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:1 - 'Queue 0 selection. 00 : Fixed Priority 01 : CBS Enabled only valid for top two enabled queues and if CBS capability selected. 10 : DWRR Enabled 11 : ETS Enabled'"]
    #[inline(always)]
    pub fn tx_sched_q0(&self) -> TX_SCHED_Q0_R {
        TX_SCHED_Q0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 'Queue 1 selection. 00 : Fixed Priority 01 : CBS Enabled only valid for top two enabled queues and if CBS capability selected. 10 : DWRR Enabled 11 : ETS Enabled'"]
    #[inline(always)]
    pub fn tx_sched_q1(&self) -> TX_SCHED_Q1_R {
        TX_SCHED_Q1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 'Queue 2 selection. 00 : Fixed Priority 01 : CBS Enabled only valid for top two enabled queues and if CBS capability selected. 10 : DWRR Enabled 11 : ETS Enabled"]
    #[inline(always)]
    pub fn tx_sched_q2(&self) -> TX_SCHED_Q2_R {
        TX_SCHED_Q2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Write ignore, read 0"]
    #[inline(always)]
    pub fn tx_sched_q3(&self) -> TX_SCHED_Q3_R {
        TX_SCHED_Q3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_31_8(&self) -> REMOVED_31_8_R {
        REMOVED_31_8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 'Queue 0 selection. 00 : Fixed Priority 01 : CBS Enabled only valid for top two enabled queues and if CBS capability selected. 10 : DWRR Enabled 11 : ETS Enabled'"]
    #[inline(always)]
    #[must_use]
    pub fn tx_sched_q0(&mut self) -> TX_SCHED_Q0_W<0> {
        TX_SCHED_Q0_W::new(self)
    }
    #[doc = "Bits 2:3 - 'Queue 1 selection. 00 : Fixed Priority 01 : CBS Enabled only valid for top two enabled queues and if CBS capability selected. 10 : DWRR Enabled 11 : ETS Enabled'"]
    #[inline(always)]
    #[must_use]
    pub fn tx_sched_q1(&mut self) -> TX_SCHED_Q1_W<2> {
        TX_SCHED_Q1_W::new(self)
    }
    #[doc = "Bits 4:5 - 'Queue 2 selection. 00 : Fixed Priority 01 : CBS Enabled only valid for top two enabled queues and if CBS capability selected. 10 : DWRR Enabled 11 : ETS Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tx_sched_q2(&mut self) -> TX_SCHED_Q2_W<4> {
        TX_SCHED_Q2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the transmit scheduling algorithm the user can select for each active transmit queue. By default all queues are initialized to fixed priority, with the top indexed queue having overall priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_sched_ctrl](index.html) module"]
pub struct TX_SCHED_CTRL_SPEC;
impl crate::RegisterSpec for TX_SCHED_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_sched_ctrl::R](R) reader structure"]
impl crate::Readable for TX_SCHED_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_sched_ctrl::W](W) writer structure"]
impl crate::Writable for TX_SCHED_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_SCHED_CTRL to value 0"]
impl crate::Resettable for TX_SCHED_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
