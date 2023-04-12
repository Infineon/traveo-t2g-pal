#[doc = "Register `PID_CHECKSUM` reader"]
pub struct R(crate::R<PID_CHECKSUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID_CHECKSUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID_CHECKSUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID_CHECKSUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PID_CHECKSUM` writer"]
pub struct W(crate::W<PID_CHECKSUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PID_CHECKSUM_SPEC>;
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
impl From<crate::W<PID_CHECKSUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PID_CHECKSUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID` reader - Header protected identifier (PID). - Bits 5 down to 0: frame identifier ID\\[5:0\\]. Frame identifier 0x3c is for a 'master request' frame, 0x3d is for a 'slave response' frame, 0x3e and 0x3f are for future LIN enhancements. Frame identifier ID\\[5:4\\]
is optionally used for length control; i.e. specifies the number of response data fields. - Bits 1 down to 0: parity bits P\\[1\\]
and P\\[0\\]. - P\\[1\\]
= ! (ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[1\\]) - P\\[0\\]
= (ID\\[4\\]
^ ID\\[2\\]
^ ID\\[1\\]
^ ID\\[0\\]) Transmission: To be transmitted PID field. SW needs to calculate the PID field parity bits P\\[1\\]
and P\\[0\\]. Reception: Received PID field. Slave node SW uses the PID field to determine how to handle the response for a received frame header: TX_RESPONSE or RX_RESPONSE."]
pub type PID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PID` writer - Header protected identifier (PID). - Bits 5 down to 0: frame identifier ID\\[5:0\\]. Frame identifier 0x3c is for a 'master request' frame, 0x3d is for a 'slave response' frame, 0x3e and 0x3f are for future LIN enhancements. Frame identifier ID\\[5:4\\]
is optionally used for length control; i.e. specifies the number of response data fields. - Bits 1 down to 0: parity bits P\\[1\\]
and P\\[0\\]. - P\\[1\\]
= ! (ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[1\\]) - P\\[0\\]
= (ID\\[4\\]
^ ID\\[2\\]
^ ID\\[1\\]
^ ID\\[0\\]) Transmission: To be transmitted PID field. SW needs to calculate the PID field parity bits P\\[1\\]
and P\\[0\\]. Reception: Received PID field. Slave node SW uses the PID field to determine how to handle the response for a received frame header: TX_RESPONSE or RX_RESPONSE."]
pub type PID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PID_CHECKSUM_SPEC, u8, u8, 8, O>;
#[doc = "Field `CHECKSUM` reader - Checksum. Transmission: HW calculated checksum (SW does not need to calculate the checksum) over the transmitted PID field (optional per CTL.CHECKSUM_ENHANCED) and data fields. Reception: Received checksum. Note that in case of a RX_CHECKSUM_ERROR, SW can use the received PID field and the received data fields to calculate the correct checksum value."]
pub type CHECKSUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Header protected identifier (PID). - Bits 5 down to 0: frame identifier ID\\[5:0\\]. Frame identifier 0x3c is for a 'master request' frame, 0x3d is for a 'slave response' frame, 0x3e and 0x3f are for future LIN enhancements. Frame identifier ID\\[5:4\\]
is optionally used for length control; i.e. specifies the number of response data fields. - Bits 1 down to 0: parity bits P\\[1\\]
and P\\[0\\]. - P\\[1\\]
= ! (ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[1\\]) - P\\[0\\]
= (ID\\[4\\]
^ ID\\[2\\]
^ ID\\[1\\]
^ ID\\[0\\]) Transmission: To be transmitted PID field. SW needs to calculate the PID field parity bits P\\[1\\]
and P\\[0\\]. Reception: Received PID field. Slave node SW uses the PID field to determine how to handle the response for a received frame header: TX_RESPONSE or RX_RESPONSE."]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Checksum. Transmission: HW calculated checksum (SW does not need to calculate the checksum) over the transmitted PID field (optional per CTL.CHECKSUM_ENHANCED) and data fields. Reception: Received checksum. Note that in case of a RX_CHECKSUM_ERROR, SW can use the received PID field and the received data fields to calculate the correct checksum value."]
    #[inline(always)]
    pub fn checksum(&self) -> CHECKSUM_R {
        CHECKSUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Header protected identifier (PID). - Bits 5 down to 0: frame identifier ID\\[5:0\\]. Frame identifier 0x3c is for a 'master request' frame, 0x3d is for a 'slave response' frame, 0x3e and 0x3f are for future LIN enhancements. Frame identifier ID\\[5:4\\]
is optionally used for length control; i.e. specifies the number of response data fields. - Bits 1 down to 0: parity bits P\\[1\\]
and P\\[0\\]. - P\\[1\\]
= ! (ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[1\\]) - P\\[0\\]
= (ID\\[4\\]
^ ID\\[2\\]
^ ID\\[1\\]
^ ID\\[0\\]) Transmission: To be transmitted PID field. SW needs to calculate the PID field parity bits P\\[1\\]
and P\\[0\\]. Reception: Received PID field. Slave node SW uses the PID field to determine how to handle the response for a received frame header: TX_RESPONSE or RX_RESPONSE."]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<0> {
        PID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PID and checksum\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid_checksum](index.html) module"]
pub struct PID_CHECKSUM_SPEC;
impl crate::RegisterSpec for PID_CHECKSUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid_checksum::R](R) reader structure"]
impl crate::Readable for PID_CHECKSUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pid_checksum::W](W) writer structure"]
impl crate::Writable for PID_CHECKSUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PID_CHECKSUM to value 0"]
impl crate::Resettable for PID_CHECKSUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
