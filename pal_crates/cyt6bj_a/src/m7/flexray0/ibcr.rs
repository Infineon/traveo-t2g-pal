#[doc = "Register `IBCR` reader"]
pub struct R(crate::R<IBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IBCR` writer"]
pub struct W(crate::W<IBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IBCR_SPEC>;
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
impl From<crate::W<IBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBRH` reader - Input Buffer Request Host Selects the target message buffer in the Message RAM for data transfer from Input Buffer. Valid values are 0x00 to 0x7F (0...127)."]
pub type IBRH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBRH` writer - Input Buffer Request Host Selects the target message buffer in the Message RAM for data transfer from Input Buffer. Valid values are 0x00 to 0x7F (0...127)."]
pub type IBRH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IBCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `IBSYH` reader - Input Buffer Busy Host Set to '1' by writing IBRH\\[6:0\\]
while IBSYS is still '1'. After the ongoing transfer between IBF Shadow and the Message RAM has completed, the IBSYH is set back to '0'. 1 = Request while transfer between IBF Shadow and Message RAM in progress 0 = No request pending"]
pub type IBSYH_R = crate::BitReader<IBSYH_A>;
#[doc = "Input Buffer Busy Host Set to '1' by writing IBRH\\[6:0\\]
while IBSYS is still '1'. After the ongoing transfer between IBF Shadow and the Message RAM has completed, the IBSYH is set back to '0'. 1 = Request while transfer between IBF Shadow and Message RAM in progress 0 = No request pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBSYH_A {
    #[doc = "0: N/A"]
    REQUEST_NOT_PENDING = 0,
    #[doc = "1: N/A"]
    REQUEST_IN_PROGRESS = 1,
}
impl From<IBSYH_A> for bool {
    #[inline(always)]
    fn from(variant: IBSYH_A) -> Self {
        variant as u8 != 0
    }
}
impl IBSYH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBSYH_A {
        match self.bits {
            false => IBSYH_A::REQUEST_NOT_PENDING,
            true => IBSYH_A::REQUEST_IN_PROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `REQUEST_NOT_PENDING`"]
    #[inline(always)]
    pub fn is_request_not_pending(&self) -> bool {
        *self == IBSYH_A::REQUEST_NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `REQUEST_IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_request_in_progress(&self) -> bool {
        *self == IBSYH_A::REQUEST_IN_PROGRESS
    }
}
#[doc = "Field `IBRS` reader - Input Buffer Request Shadow Number of the target message buffer actually updated / lately updated. Valid values are 0x00 to 0x7F (0...127)."]
pub type IBRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBSYS` reader - Input Buffer Busy Shadow Set to '1' after writing IBRH\\[6:0\\]. When the transfer between IBF Shadow and the Message RAM has completed, IBSYS is set back to '0'. 1 = Transfer between IBF Shadow and Message RAM in progress 0 = Transfer between IBF Shadow and Message RAM completed"]
pub type IBSYS_R = crate::BitReader<IBSYS_A>;
#[doc = "Input Buffer Busy Shadow Set to '1' after writing IBRH\\[6:0\\]. When the transfer between IBF Shadow and the Message RAM has completed, IBSYS is set back to '0'. 1 = Transfer between IBF Shadow and Message RAM in progress 0 = Transfer between IBF Shadow and Message RAM completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBSYS_A {
    #[doc = "0: N/A"]
    TXFR_IBF2MSRAM_COMPLETE = 0,
    #[doc = "1: N/A"]
    TXFR_IBF2MSRAM_IN_PROGRESS = 1,
}
impl From<IBSYS_A> for bool {
    #[inline(always)]
    fn from(variant: IBSYS_A) -> Self {
        variant as u8 != 0
    }
}
impl IBSYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBSYS_A {
        match self.bits {
            false => IBSYS_A::TXFR_IBF2MSRAM_COMPLETE,
            true => IBSYS_A::TXFR_IBF2MSRAM_IN_PROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `TXFR_IBF2MSRAM_COMPLETE`"]
    #[inline(always)]
    pub fn is_txfr_ibf2msram_complete(&self) -> bool {
        *self == IBSYS_A::TXFR_IBF2MSRAM_COMPLETE
    }
    #[doc = "Checks if the value of the field is `TXFR_IBF2MSRAM_IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_txfr_ibf2msram_in_progress(&self) -> bool {
        *self == IBSYS_A::TXFR_IBF2MSRAM_IN_PROGRESS
    }
}
impl R {
    #[doc = "Bits 0:6 - Input Buffer Request Host Selects the target message buffer in the Message RAM for data transfer from Input Buffer. Valid values are 0x00 to 0x7F (0...127)."]
    #[inline(always)]
    pub fn ibrh(&self) -> IBRH_R {
        IBRH_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Input Buffer Busy Host Set to '1' by writing IBRH\\[6:0\\]
while IBSYS is still '1'. After the ongoing transfer between IBF Shadow and the Message RAM has completed, the IBSYH is set back to '0'. 1 = Request while transfer between IBF Shadow and Message RAM in progress 0 = No request pending"]
    #[inline(always)]
    pub fn ibsyh(&self) -> IBSYH_R {
        IBSYH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Input Buffer Request Shadow Number of the target message buffer actually updated / lately updated. Valid values are 0x00 to 0x7F (0...127)."]
    #[inline(always)]
    pub fn ibrs(&self) -> IBRS_R {
        IBRS_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Input Buffer Busy Shadow Set to '1' after writing IBRH\\[6:0\\]. When the transfer between IBF Shadow and the Message RAM has completed, IBSYS is set back to '0'. 1 = Transfer between IBF Shadow and Message RAM in progress 0 = Transfer between IBF Shadow and Message RAM completed"]
    #[inline(always)]
    pub fn ibsys(&self) -> IBSYS_R {
        IBSYS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input Buffer Request Host Selects the target message buffer in the Message RAM for data transfer from Input Buffer. Valid values are 0x00 to 0x7F (0...127)."]
    #[inline(always)]
    #[must_use]
    pub fn ibrh(&mut self) -> IBRH_W<0> {
        IBRH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Buffer Command Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibcr](index.html) module"]
pub struct IBCR_SPEC;
impl crate::RegisterSpec for IBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibcr::R](R) reader structure"]
impl crate::Readable for IBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ibcr::W](W) writer structure"]
impl crate::Writable for IBCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IBCR to value 0"]
impl crate::Resettable for IBCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
