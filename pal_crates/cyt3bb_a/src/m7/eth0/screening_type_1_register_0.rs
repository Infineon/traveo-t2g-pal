#[doc = "Register `SCREENING_TYPE_1_REGISTER_0` reader"]
pub struct R(crate::R<SCREENING_TYPE_1_REGISTER_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCREENING_TYPE_1_REGISTER_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCREENING_TYPE_1_REGISTER_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCREENING_TYPE_1_REGISTER_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCREENING_TYPE_1_REGISTER_0` writer"]
pub struct W(crate::W<SCREENING_TYPE_1_REGISTER_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCREENING_TYPE_1_REGISTER_0_SPEC>;
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
impl From<crate::W<SCREENING_TYPE_1_REGISTER_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCREENING_TYPE_1_REGISTER_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUEUE_NUMBER` reader - Queue Number (0 to 15)"]
pub type QUEUE_NUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QUEUE_NUMBER` writer - Queue Number (0 to 15)"]
pub type QUEUE_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCREENING_TYPE_1_REGISTER_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DSTC_MATCH` reader - DS/TC Match"]
pub type DSTC_MATCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSTC_MATCH` writer - DS/TC Match"]
pub type DSTC_MATCH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCREENING_TYPE_1_REGISTER_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `UDP_PORT_MATCH` reader - UDP Port Match"]
pub type UDP_PORT_MATCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UDP_PORT_MATCH` writer - UDP Port Match"]
pub type UDP_PORT_MATCH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCREENING_TYPE_1_REGISTER_0_SPEC, u16, u16, 16, O>;
#[doc = "Field `DSTC_ENABLE` reader - DS/TC Enable"]
pub type DSTC_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DSTC_ENABLE` writer - DS/TC Enable"]
pub type DSTC_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCREENING_TYPE_1_REGISTER_0_SPEC, bool, O>;
#[doc = "Field `UDP_PORT_MATCH_ENABLE` reader - UDP port match enable"]
pub type UDP_PORT_MATCH_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `UDP_PORT_MATCH_ENABLE` writer - UDP port match enable"]
pub type UDP_PORT_MATCH_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCREENING_TYPE_1_REGISTER_0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Queue Number (0 to 15)"]
    #[inline(always)]
    pub fn queue_number(&self) -> QUEUE_NUMBER_R {
        QUEUE_NUMBER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - DS/TC Match"]
    #[inline(always)]
    pub fn dstc_match(&self) -> DSTC_MATCH_R {
        DSTC_MATCH_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:27 - UDP Port Match"]
    #[inline(always)]
    pub fn udp_port_match(&self) -> UDP_PORT_MATCH_R {
        UDP_PORT_MATCH_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 28 - DS/TC Enable"]
    #[inline(always)]
    pub fn dstc_enable(&self) -> DSTC_ENABLE_R {
        DSTC_ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - UDP port match enable"]
    #[inline(always)]
    pub fn udp_port_match_enable(&self) -> UDP_PORT_MATCH_ENABLE_R {
        UDP_PORT_MATCH_ENABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Queue Number (0 to 15)"]
    #[inline(always)]
    #[must_use]
    pub fn queue_number(&mut self) -> QUEUE_NUMBER_W<0> {
        QUEUE_NUMBER_W::new(self)
    }
    #[doc = "Bits 4:11 - DS/TC Match"]
    #[inline(always)]
    #[must_use]
    pub fn dstc_match(&mut self) -> DSTC_MATCH_W<4> {
        DSTC_MATCH_W::new(self)
    }
    #[doc = "Bits 12:27 - UDP Port Match"]
    #[inline(always)]
    #[must_use]
    pub fn udp_port_match(&mut self) -> UDP_PORT_MATCH_W<12> {
        UDP_PORT_MATCH_W::new(self)
    }
    #[doc = "Bit 28 - DS/TC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dstc_enable(&mut self) -> DSTC_ENABLE_W<28> {
        DSTC_ENABLE_W::new(self)
    }
    #[doc = "Bit 29 - UDP port match enable"]
    #[inline(always)]
    #[must_use]
    pub fn udp_port_match_enable(&mut self) -> UDP_PORT_MATCH_ENABLE_W<29> {
        UDP_PORT_MATCH_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Screening type 1 registers are used to allocate up to 16 priority queues to received frames based on certain IP or UDP fields of incoming frames. Firstly, when DS/TC match enable is set (bit 28), the DS (Differentiated Services) field of the received IPv4 header or TCfield (traffic class) of IPv6 headers are matched against bits 11 to 4. Secondly, when UDP port match enable is set (bit 29), the UDP Destination Port of the received UDP frame is matched against bits 27 to 12. Both UDP and DS/TC matching can be enabled simultaneously or individually. If a match is successful, then the queue value programmed in bits 2 to 0 is allocated to the frame. The required number of Type 1 screening registers is configured in the gem defines file. Up to 16 type 1 screening registers have been allocated APB address space between 0x500 and 0x53C.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [screening_type_1_register_0](index.html) module"]
pub struct SCREENING_TYPE_1_REGISTER_0_SPEC;
impl crate::RegisterSpec for SCREENING_TYPE_1_REGISTER_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [screening_type_1_register_0::R](R) reader structure"]
impl crate::Readable for SCREENING_TYPE_1_REGISTER_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [screening_type_1_register_0::W](W) writer structure"]
impl crate::Writable for SCREENING_TYPE_1_REGISTER_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCREENING_TYPE_1_REGISTER_0 to value 0"]
impl crate::Resettable for SCREENING_TYPE_1_REGISTER_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
