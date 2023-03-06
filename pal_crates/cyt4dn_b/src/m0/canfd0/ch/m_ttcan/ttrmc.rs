#[doc = "Register `TTRMC` reader"]
pub struct R(crate::R<TTRMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTRMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTRMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTRMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTRMC` writer"]
pub struct W(crate::W<TTRMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTRMC_SPEC>;
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
impl From<crate::W<TTRMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTRMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RID` reader - Reference Identifier Identifier transmitted with reference message and used for reference message filtering. Standard or extended reference identifier depending on bit XTD. A standard identifier has to be written to ID\\[28:18\\]."]
pub type RID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RID` writer - Reference Identifier Identifier transmitted with reference message and used for reference message filtering. Standard or extended reference identifier depending on bit XTD. A standard identifier has to be written to ID\\[28:18\\]."]
pub type RID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTRMC_SPEC, u32, u32, 29, O>;
#[doc = "Field `XTD` reader - Extended Identifier 0= 11-bit standard identifier 1= 29-bit extended identifier"]
pub type XTD_R = crate::BitReader<bool>;
#[doc = "Field `XTD` writer - Extended Identifier 0= 11-bit standard identifier 1= 29-bit extended identifier"]
pub type XTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTRMC_SPEC, bool, O>;
#[doc = "Field `RMPS` reader - Reference Message Payload Select Ignored in case of time slaves. 0= Reference message has no additional payload 1= The following elements are taken from Tx Buffer 0: Message Marker MM, Event FIFO Control EFC, Data Length Code DLC, Data Bytes DB Level 1: bytes 2-8, Level 0,2: bytes 5-8)"]
pub type RMPS_R = crate::BitReader<bool>;
#[doc = "Field `RMPS` writer - Reference Message Payload Select Ignored in case of time slaves. 0= Reference message has no additional payload 1= The following elements are taken from Tx Buffer 0: Message Marker MM, Event FIFO Control EFC, Data Length Code DLC, Data Bytes DB Level 1: bytes 2-8, Level 0,2: bytes 5-8)"]
pub type RMPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTRMC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:28 - Reference Identifier Identifier transmitted with reference message and used for reference message filtering. Standard or extended reference identifier depending on bit XTD. A standard identifier has to be written to ID\\[28:18\\]."]
    #[inline(always)]
    pub fn rid(&self) -> RID_R {
        RID_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 30 - Extended Identifier 0= 11-bit standard identifier 1= 29-bit extended identifier"]
    #[inline(always)]
    pub fn xtd(&self) -> XTD_R {
        XTD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reference Message Payload Select Ignored in case of time slaves. 0= Reference message has no additional payload 1= The following elements are taken from Tx Buffer 0: Message Marker MM, Event FIFO Control EFC, Data Length Code DLC, Data Bytes DB Level 1: bytes 2-8, Level 0,2: bytes 5-8)"]
    #[inline(always)]
    pub fn rmps(&self) -> RMPS_R {
        RMPS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Reference Identifier Identifier transmitted with reference message and used for reference message filtering. Standard or extended reference identifier depending on bit XTD. A standard identifier has to be written to ID\\[28:18\\]."]
    #[inline(always)]
    #[must_use]
    pub fn rid(&mut self) -> RID_W<0> {
        RID_W::new(self)
    }
    #[doc = "Bit 30 - Extended Identifier 0= 11-bit standard identifier 1= 29-bit extended identifier"]
    #[inline(always)]
    #[must_use]
    pub fn xtd(&mut self) -> XTD_W<30> {
        XTD_W::new(self)
    }
    #[doc = "Bit 31 - Reference Message Payload Select Ignored in case of time slaves. 0= Reference message has no additional payload 1= The following elements are taken from Tx Buffer 0: Message Marker MM, Event FIFO Control EFC, Data Length Code DLC, Data Bytes DB Level 1: bytes 2-8, Level 0,2: bytes 5-8)"]
    #[inline(always)]
    #[must_use]
    pub fn rmps(&mut self) -> RMPS_W<31> {
        RMPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TT Reference Message Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttrmc](index.html) module"]
pub struct TTRMC_SPEC;
impl crate::RegisterSpec for TTRMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttrmc::R](R) reader structure"]
impl crate::Readable for TTRMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttrmc::W](W) writer structure"]
impl crate::Writable for TTRMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTRMC to value 0"]
impl crate::Resettable for TTRMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
