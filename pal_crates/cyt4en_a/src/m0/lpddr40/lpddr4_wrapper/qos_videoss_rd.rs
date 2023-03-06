#[doc = "Register `QOS_VIDEOSS_RD` reader"]
pub struct R(crate::R<QOS_VIDEOSS_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QOS_VIDEOSS_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QOS_VIDEOSS_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QOS_VIDEOSS_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QOS_VIDEOSS_RD` writer"]
pub struct W(crate::W<QOS_VIDEOSS_RD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QOS_VIDEOSS_RD_SPEC>;
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
impl From<crate::W<QOS_VIDEOSS_RD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QOS_VIDEOSS_RD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QOS_MASTERID_0` reader - QoS value of the VIDEOSS read master ID0"]
pub type QOS_MASTERID_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_0` writer - QoS value of the VIDEOSS read master ID0"]
pub type QOS_MASTERID_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_1` reader - QoS value of the VIDEOSS read master ID1"]
pub type QOS_MASTERID_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_1` writer - QoS value of the VIDEOSS read master ID1"]
pub type QOS_MASTERID_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_2` reader - QoS value of the VIDEOSS read master ID2"]
pub type QOS_MASTERID_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_2` writer - QoS value of the VIDEOSS read master ID2"]
pub type QOS_MASTERID_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_3` reader - QoS value of the VIDEOSS read master ID3"]
pub type QOS_MASTERID_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_3` writer - QoS value of the VIDEOSS read master ID3"]
pub type QOS_MASTERID_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_4` reader - QoS value of the VIDEOSS read master ID4"]
pub type QOS_MASTERID_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_4` writer - QoS value of the VIDEOSS read master ID4"]
pub type QOS_MASTERID_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_5` reader - QoS value of the VIDEOSS read master ID5"]
pub type QOS_MASTERID_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_5` writer - QoS value of the VIDEOSS read master ID5"]
pub type QOS_MASTERID_5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_6` reader - QoS value of the VIDEOSS read master ID6"]
pub type QOS_MASTERID_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_6` writer - QoS value of the VIDEOSS read master ID6"]
pub type QOS_MASTERID_6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_7` reader - QoS value of the VIDEOSS read master ID7"]
pub type QOS_MASTERID_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_7` writer - QoS value of the VIDEOSS read master ID7"]
pub type QOS_MASTERID_7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_8` reader - QoS value of the VIDEOSS read master ID8"]
pub type QOS_MASTERID_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_8` writer - QoS value of the VIDEOSS read master ID8"]
pub type QOS_MASTERID_8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_9` reader - QoS value of the VIDEOSS read master ID9"]
pub type QOS_MASTERID_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_9` writer - QoS value of the VIDEOSS read master ID9"]
pub type QOS_MASTERID_9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_10` reader - QoS value of the VIDEOSS read master ID10"]
pub type QOS_MASTERID_10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_10` writer - QoS value of the VIDEOSS read master ID10"]
pub type QOS_MASTERID_10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_11` reader - QoS value of the VIDEOSS read master ID11"]
pub type QOS_MASTERID_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_11` writer - QoS value of the VIDEOSS read master ID11"]
pub type QOS_MASTERID_11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_12` reader - QoS value of the VIDEOSS read master ID12"]
pub type QOS_MASTERID_12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_12` writer - QoS value of the VIDEOSS read master ID12"]
pub type QOS_MASTERID_12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_13` reader - QoS value of the VIDEOSS read master ID13"]
pub type QOS_MASTERID_13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_13` writer - QoS value of the VIDEOSS read master ID13"]
pub type QOS_MASTERID_13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_14` reader - QoS value of the VIDEOSS read master ID14"]
pub type QOS_MASTERID_14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_14` writer - QoS value of the VIDEOSS read master ID14"]
pub type QOS_MASTERID_14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
#[doc = "Field `QOS_MASTERID_15` reader - QoS value of the VIDEOSS read master ID15"]
pub type QOS_MASTERID_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS_MASTERID_15` writer - QoS value of the VIDEOSS read master ID15"]
pub type QOS_MASTERID_15_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QOS_VIDEOSS_RD_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - QoS value of the VIDEOSS read master ID0"]
    #[inline(always)]
    pub fn qos_masterid_0(&self) -> QOS_MASTERID_0_R {
        QOS_MASTERID_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - QoS value of the VIDEOSS read master ID1"]
    #[inline(always)]
    pub fn qos_masterid_1(&self) -> QOS_MASTERID_1_R {
        QOS_MASTERID_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - QoS value of the VIDEOSS read master ID2"]
    #[inline(always)]
    pub fn qos_masterid_2(&self) -> QOS_MASTERID_2_R {
        QOS_MASTERID_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - QoS value of the VIDEOSS read master ID3"]
    #[inline(always)]
    pub fn qos_masterid_3(&self) -> QOS_MASTERID_3_R {
        QOS_MASTERID_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - QoS value of the VIDEOSS read master ID4"]
    #[inline(always)]
    pub fn qos_masterid_4(&self) -> QOS_MASTERID_4_R {
        QOS_MASTERID_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - QoS value of the VIDEOSS read master ID5"]
    #[inline(always)]
    pub fn qos_masterid_5(&self) -> QOS_MASTERID_5_R {
        QOS_MASTERID_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - QoS value of the VIDEOSS read master ID6"]
    #[inline(always)]
    pub fn qos_masterid_6(&self) -> QOS_MASTERID_6_R {
        QOS_MASTERID_6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - QoS value of the VIDEOSS read master ID7"]
    #[inline(always)]
    pub fn qos_masterid_7(&self) -> QOS_MASTERID_7_R {
        QOS_MASTERID_7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - QoS value of the VIDEOSS read master ID8"]
    #[inline(always)]
    pub fn qos_masterid_8(&self) -> QOS_MASTERID_8_R {
        QOS_MASTERID_8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - QoS value of the VIDEOSS read master ID9"]
    #[inline(always)]
    pub fn qos_masterid_9(&self) -> QOS_MASTERID_9_R {
        QOS_MASTERID_9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - QoS value of the VIDEOSS read master ID10"]
    #[inline(always)]
    pub fn qos_masterid_10(&self) -> QOS_MASTERID_10_R {
        QOS_MASTERID_10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - QoS value of the VIDEOSS read master ID11"]
    #[inline(always)]
    pub fn qos_masterid_11(&self) -> QOS_MASTERID_11_R {
        QOS_MASTERID_11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - QoS value of the VIDEOSS read master ID12"]
    #[inline(always)]
    pub fn qos_masterid_12(&self) -> QOS_MASTERID_12_R {
        QOS_MASTERID_12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - QoS value of the VIDEOSS read master ID13"]
    #[inline(always)]
    pub fn qos_masterid_13(&self) -> QOS_MASTERID_13_R {
        QOS_MASTERID_13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - QoS value of the VIDEOSS read master ID14"]
    #[inline(always)]
    pub fn qos_masterid_14(&self) -> QOS_MASTERID_14_R {
        QOS_MASTERID_14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - QoS value of the VIDEOSS read master ID15"]
    #[inline(always)]
    pub fn qos_masterid_15(&self) -> QOS_MASTERID_15_R {
        QOS_MASTERID_15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - QoS value of the VIDEOSS read master ID0"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_0(&mut self) -> QOS_MASTERID_0_W<0> {
        QOS_MASTERID_0_W::new(self)
    }
    #[doc = "Bits 2:3 - QoS value of the VIDEOSS read master ID1"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_1(&mut self) -> QOS_MASTERID_1_W<2> {
        QOS_MASTERID_1_W::new(self)
    }
    #[doc = "Bits 4:5 - QoS value of the VIDEOSS read master ID2"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_2(&mut self) -> QOS_MASTERID_2_W<4> {
        QOS_MASTERID_2_W::new(self)
    }
    #[doc = "Bits 6:7 - QoS value of the VIDEOSS read master ID3"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_3(&mut self) -> QOS_MASTERID_3_W<6> {
        QOS_MASTERID_3_W::new(self)
    }
    #[doc = "Bits 8:9 - QoS value of the VIDEOSS read master ID4"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_4(&mut self) -> QOS_MASTERID_4_W<8> {
        QOS_MASTERID_4_W::new(self)
    }
    #[doc = "Bits 10:11 - QoS value of the VIDEOSS read master ID5"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_5(&mut self) -> QOS_MASTERID_5_W<10> {
        QOS_MASTERID_5_W::new(self)
    }
    #[doc = "Bits 12:13 - QoS value of the VIDEOSS read master ID6"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_6(&mut self) -> QOS_MASTERID_6_W<12> {
        QOS_MASTERID_6_W::new(self)
    }
    #[doc = "Bits 14:15 - QoS value of the VIDEOSS read master ID7"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_7(&mut self) -> QOS_MASTERID_7_W<14> {
        QOS_MASTERID_7_W::new(self)
    }
    #[doc = "Bits 16:17 - QoS value of the VIDEOSS read master ID8"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_8(&mut self) -> QOS_MASTERID_8_W<16> {
        QOS_MASTERID_8_W::new(self)
    }
    #[doc = "Bits 18:19 - QoS value of the VIDEOSS read master ID9"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_9(&mut self) -> QOS_MASTERID_9_W<18> {
        QOS_MASTERID_9_W::new(self)
    }
    #[doc = "Bits 20:21 - QoS value of the VIDEOSS read master ID10"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_10(&mut self) -> QOS_MASTERID_10_W<20> {
        QOS_MASTERID_10_W::new(self)
    }
    #[doc = "Bits 22:23 - QoS value of the VIDEOSS read master ID11"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_11(&mut self) -> QOS_MASTERID_11_W<22> {
        QOS_MASTERID_11_W::new(self)
    }
    #[doc = "Bits 24:25 - QoS value of the VIDEOSS read master ID12"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_12(&mut self) -> QOS_MASTERID_12_W<24> {
        QOS_MASTERID_12_W::new(self)
    }
    #[doc = "Bits 26:27 - QoS value of the VIDEOSS read master ID13"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_13(&mut self) -> QOS_MASTERID_13_W<26> {
        QOS_MASTERID_13_W::new(self)
    }
    #[doc = "Bits 28:29 - QoS value of the VIDEOSS read master ID14"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_14(&mut self) -> QOS_MASTERID_14_W<28> {
        QOS_MASTERID_14_W::new(self)
    }
    #[doc = "Bits 30:31 - QoS value of the VIDEOSS read master ID15"]
    #[inline(always)]
    #[must_use]
    pub fn qos_masterid_15(&mut self) -> QOS_MASTERID_15_W<30> {
        QOS_MASTERID_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QoS Configuration Register for VIDEOSS read master\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qos_videoss_rd](index.html) module"]
pub struct QOS_VIDEOSS_RD_SPEC;
impl crate::RegisterSpec for QOS_VIDEOSS_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qos_videoss_rd::R](R) reader structure"]
impl crate::Readable for QOS_VIDEOSS_RD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qos_videoss_rd::W](W) writer structure"]
impl crate::Writable for QOS_VIDEOSS_RD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QOS_VIDEOSS_RD to value 0"]
impl crate::Resettable for QOS_VIDEOSS_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
