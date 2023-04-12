#[doc = "Register `OBCR` reader"]
pub struct R(crate::R<OBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OBCR` writer"]
pub struct W(crate::W<OBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OBCR_SPEC>;
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
impl From<crate::W<OBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OBRS` reader - Output Buffer Request Shadow Number of source message buffer to be transferred from the Message RAM to OBF Shadow. Valid values are 0x00 to 0x7F (0...127). If the number of the first message buffer of the receive FIFO is written to this register the Message Handler transfers the message buffer addressed by the GET Index (GIDX, see \\[01\\]Section 5.10 FIFO Function) to OBF Shadow."]
pub type OBRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OBRS` writer - Output Buffer Request Shadow Number of source message buffer to be transferred from the Message RAM to OBF Shadow. Valid values are 0x00 to 0x7F (0...127). If the number of the first message buffer of the receive FIFO is written to this register the Message Handler transfers the message buffer addressed by the GET Index (GIDX, see \\[01\\]Section 5.10 FIFO Function) to OBF Shadow."]
pub type OBRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OBCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `VIEW` reader - View Shadow Buffer Toggles between OBF Shadow and OBF Host. Only writeable while OBSYS = '0'. 1 = Swap OBF Shadow and OBF Host 0 = No action"]
pub type VIEW_R = crate::BitReader<VIEW_A>;
#[doc = "View Shadow Buffer Toggles between OBF Shadow and OBF Host. Only writeable while OBSYS = '0'. 1 = Swap OBF Shadow and OBF Host 0 = No action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VIEW_A {
    #[doc = "0: N/A"]
    NO_ACTION = 0,
    #[doc = "1: N/A"]
    SWP_OBFS_AND_OBFH = 1,
}
impl From<VIEW_A> for bool {
    #[inline(always)]
    fn from(variant: VIEW_A) -> Self {
        variant as u8 != 0
    }
}
impl VIEW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIEW_A {
        match self.bits {
            false => VIEW_A::NO_ACTION,
            true => VIEW_A::SWP_OBFS_AND_OBFH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == VIEW_A::NO_ACTION
    }
    #[doc = "Checks if the value of the field is `SWP_OBFS_AND_OBFH`"]
    #[inline(always)]
    pub fn is_swp_obfs_and_obfh(&self) -> bool {
        *self == VIEW_A::SWP_OBFS_AND_OBFH
    }
}
#[doc = "Field `VIEW` writer - View Shadow Buffer Toggles between OBF Shadow and OBF Host. Only writeable while OBSYS = '0'. 1 = Swap OBF Shadow and OBF Host 0 = No action"]
pub type VIEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OBCR_SPEC, VIEW_A, O>;
impl<'a, const O: u8> VIEW_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(VIEW_A::NO_ACTION)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn swp_obfs_and_obfh(self) -> &'a mut W {
        self.variant(VIEW_A::SWP_OBFS_AND_OBFH)
    }
}
#[doc = "Field `REQ` reader - Request Message RAM Transfer Requests transfer of message buffer addressed by OBRS\\[6:0\\]
from Message RAM to OBF Shadow. Only writeable while OBSYS = '0'. 1 = Transfer to OBF Shadow requested 0 = No request"]
pub type REQ_R = crate::BitReader<REQ_A>;
#[doc = "Request Message RAM Transfer Requests transfer of message buffer addressed by OBRS\\[6:0\\]
from Message RAM to OBF Shadow. Only writeable while OBSYS = '0'. 1 = Transfer to OBF Shadow requested 0 = No request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REQ_A {
    #[doc = "0: N/A"]
    NO_REQUEST = 0,
    #[doc = "1: N/A"]
    TXFR_TO_OBFS_REQ = 1,
}
impl From<REQ_A> for bool {
    #[inline(always)]
    fn from(variant: REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQ_A {
        match self.bits {
            false => REQ_A::NO_REQUEST,
            true => REQ_A::TXFR_TO_OBFS_REQ,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == REQ_A::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `TXFR_TO_OBFS_REQ`"]
    #[inline(always)]
    pub fn is_txfr_to_obfs_req(&self) -> bool {
        *self == REQ_A::TXFR_TO_OBFS_REQ
    }
}
#[doc = "Field `REQ` writer - Request Message RAM Transfer Requests transfer of message buffer addressed by OBRS\\[6:0\\]
from Message RAM to OBF Shadow. Only writeable while OBSYS = '0'. 1 = Transfer to OBF Shadow requested 0 = No request"]
pub type REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, OBCR_SPEC, REQ_A, O>;
impl<'a, const O: u8> REQ_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_request(self) -> &'a mut W {
        self.variant(REQ_A::NO_REQUEST)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn txfr_to_obfs_req(self) -> &'a mut W {
        self.variant(REQ_A::TXFR_TO_OBFS_REQ)
    }
}
#[doc = "Field `OBSYS` reader - Output Buffer Busy Shadow Set to '1' after setting bit REQ. When the transfer between the Message RAM and OBF Shadow has completed, OBSYS is set back to '0'. 1 = Transfer between Message RAM and OBF Shadow in progress 0 = No transfer in progress"]
pub type OBSYS_R = crate::BitReader<OBSYS_A>;
#[doc = "Output Buffer Busy Shadow Set to '1' after setting bit REQ. When the transfer between the Message RAM and OBF Shadow has completed, OBSYS is set back to '0'. 1 = Transfer between Message RAM and OBF Shadow in progress 0 = No transfer in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBSYS_A {
    #[doc = "0: N/A"]
    NO_TXFR_IN_PROGRESS = 0,
    #[doc = "1: N/A"]
    TXFR_BTW_MSRAM_AND_OBFS = 1,
}
impl From<OBSYS_A> for bool {
    #[inline(always)]
    fn from(variant: OBSYS_A) -> Self {
        variant as u8 != 0
    }
}
impl OBSYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBSYS_A {
        match self.bits {
            false => OBSYS_A::NO_TXFR_IN_PROGRESS,
            true => OBSYS_A::TXFR_BTW_MSRAM_AND_OBFS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TXFR_IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_no_txfr_in_progress(&self) -> bool {
        *self == OBSYS_A::NO_TXFR_IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `TXFR_BTW_MSRAM_AND_OBFS`"]
    #[inline(always)]
    pub fn is_txfr_btw_msram_and_obfs(&self) -> bool {
        *self == OBSYS_A::TXFR_BTW_MSRAM_AND_OBFS
    }
}
#[doc = "Field `OBRH` reader - Output Buffer Request Host Number of message buffer currently accessible by the Host via RDHS\\[1...3\\], MBS, and RDDS\\[1...64\\]. By writing VIEW to '1' OBF Shadow and OBF Host are swapped and the transferred message buffer is accessible by the Host. Valid values are 0x00 to 0x7F (0...127)."]
pub type OBRH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Output Buffer Request Shadow Number of source message buffer to be transferred from the Message RAM to OBF Shadow. Valid values are 0x00 to 0x7F (0...127). If the number of the first message buffer of the receive FIFO is written to this register the Message Handler transfers the message buffer addressed by the GET Index (GIDX, see \\[01\\]Section 5.10 FIFO Function) to OBF Shadow."]
    #[inline(always)]
    pub fn obrs(&self) -> OBRS_R {
        OBRS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - View Shadow Buffer Toggles between OBF Shadow and OBF Host. Only writeable while OBSYS = '0'. 1 = Swap OBF Shadow and OBF Host 0 = No action"]
    #[inline(always)]
    pub fn view(&self) -> VIEW_R {
        VIEW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Request Message RAM Transfer Requests transfer of message buffer addressed by OBRS\\[6:0\\]
from Message RAM to OBF Shadow. Only writeable while OBSYS = '0'. 1 = Transfer to OBF Shadow requested 0 = No request"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Output Buffer Busy Shadow Set to '1' after setting bit REQ. When the transfer between the Message RAM and OBF Shadow has completed, OBSYS is set back to '0'. 1 = Transfer between Message RAM and OBF Shadow in progress 0 = No transfer in progress"]
    #[inline(always)]
    pub fn obsys(&self) -> OBSYS_R {
        OBSYS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Output Buffer Request Host Number of message buffer currently accessible by the Host via RDHS\\[1...3\\], MBS, and RDDS\\[1...64\\]. By writing VIEW to '1' OBF Shadow and OBF Host are swapped and the transferred message buffer is accessible by the Host. Valid values are 0x00 to 0x7F (0...127)."]
    #[inline(always)]
    pub fn obrh(&self) -> OBRH_R {
        OBRH_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Output Buffer Request Shadow Number of source message buffer to be transferred from the Message RAM to OBF Shadow. Valid values are 0x00 to 0x7F (0...127). If the number of the first message buffer of the receive FIFO is written to this register the Message Handler transfers the message buffer addressed by the GET Index (GIDX, see \\[01\\]Section 5.10 FIFO Function) to OBF Shadow."]
    #[inline(always)]
    #[must_use]
    pub fn obrs(&mut self) -> OBRS_W<0> {
        OBRS_W::new(self)
    }
    #[doc = "Bit 8 - View Shadow Buffer Toggles between OBF Shadow and OBF Host. Only writeable while OBSYS = '0'. 1 = Swap OBF Shadow and OBF Host 0 = No action"]
    #[inline(always)]
    #[must_use]
    pub fn view(&mut self) -> VIEW_W<8> {
        VIEW_W::new(self)
    }
    #[doc = "Bit 9 - Request Message RAM Transfer Requests transfer of message buffer addressed by OBRS\\[6:0\\]
from Message RAM to OBF Shadow. Only writeable while OBSYS = '0'. 1 = Transfer to OBF Shadow requested 0 = No request"]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> REQ_W<9> {
        REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Buffer Command Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obcr](index.html) module"]
pub struct OBCR_SPEC;
impl crate::RegisterSpec for OBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [obcr::R](R) reader structure"]
impl crate::Readable for OBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [obcr::W](W) writer structure"]
impl crate::Writable for OBCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OBCR to value 0"]
impl crate::Resettable for OBCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
