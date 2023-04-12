#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Writing '1' to this register will activate fetch and store related shadow configuration (RWS) and then start a new decoding operation. The following registers are initialized by this operation: ERRORSTATUS STORESTATUS FETCHSTATUS DECODINGSTATUS INTR_DEC ERRORCODE CORRECTIONCROPSTATUS SUSPEND DNL DRI (only if DECODINGOPTION.MARKERSKIP=0) All Huffman Tables (only if DECODINGOPTION.MARKERSKIP=0)"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Writing '1' to this register will activate fetch and store related shadow configuration (RWS) and then start a new decoding operation. The following registers are initialized by this operation: ERRORSTATUS STORESTATUS FETCHSTATUS DECODINGSTATUS INTR_DEC ERRORCODE CORRECTIONCROPSTATUS SUSPEND DNL DRI (only if DECODINGOPTION.MARKERSKIP=0) All Huffman Tables (only if DECODINGOPTION.MARKERSKIP=0)"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RESUME` reader - Writing '1' to this register will activate store related shadow configuration (RWS) and then resume a suspended decoding operation."]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - Writing '1' to this register will activate store related shadow configuration (RWS) and then resume a suspended decoding operation."]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Writing '1' to this register will activate fetch and store related shadow configuration (RWS) and then start a new decoding operation. The following registers are initialized by this operation: ERRORSTATUS STORESTATUS FETCHSTATUS DECODINGSTATUS INTR_DEC ERRORCODE CORRECTIONCROPSTATUS SUSPEND DNL DRI (only if DECODINGOPTION.MARKERSKIP=0) All Huffman Tables (only if DECODINGOPTION.MARKERSKIP=0)"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Writing '1' to this register will activate store related shadow configuration (RWS) and then resume a suspended decoding operation."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing '1' to this register will activate fetch and store related shadow configuration (RWS) and then start a new decoding operation. The following registers are initialized by this operation: ERRORSTATUS STORESTATUS FETCHSTATUS DECODINGSTATUS INTR_DEC ERRORCODE CORRECTIONCROPSTATUS SUSPEND DNL DRI (only if DECODINGOPTION.MARKERSKIP=0) All Huffman Tables (only if DECODINGOPTION.MARKERSKIP=0)"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 8 - Writing '1' to this register will activate store related shadow configuration (RWS) and then resume a suspended decoding operation."]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<8> {
        RESUME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start/Resume triggers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
