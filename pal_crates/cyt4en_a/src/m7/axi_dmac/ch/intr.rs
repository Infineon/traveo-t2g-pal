#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPLETION` reader - Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
pub type COMPLETION_R = crate::BitReader<bool>;
#[doc = "Field `COMPLETION` writer - Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
pub type COMPLETION_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `SRC_BUS_ERROR` reader - Activated (set to '1') on a bus error for a load from the source."]
pub type SRC_BUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `SRC_BUS_ERROR` writer - Activated (set to '1') on a bus error for a load from the source."]
pub type SRC_BUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `DST_BUS_ERROR` reader - Activated (set to '1') on a bus error for a store to the destination. Also activated when an AXI write response is received although there is no outstanding write transaction."]
pub type DST_BUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `DST_BUS_ERROR` writer - Activated (set to '1') on a bus error for a store to the destination. Also activated when an AXI write response is received although there is no outstanding write transaction."]
pub type DST_BUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `INVALID_DESCR_TYPE` reader - Activated (set to '1') when the descriptor type is invalid."]
pub type INVALID_DESCR_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `INVALID_DESCR_TYPE` writer - Activated (set to '1') when the descriptor type is invalid."]
pub type INVALID_DESCR_TYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `CURR_PTR_NULL` reader - Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1'), CH_CURR_PTR is '0', and the channel is triggered."]
pub type CURR_PTR_NULL_R = crate::BitReader<bool>;
#[doc = "Field `CURR_PTR_NULL` writer - Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1'), CH_CURR_PTR is '0', and the channel is triggered."]
pub type CURR_PTR_NULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `ACTIVE_CH_DISABLED` reader - Activated (set to '1') when the channel is disabled and the data transfer engine is busy, i.e there is a pending trigger. This flag is also set if an input trigger is received at the same time another error occurs (in this case, ACT_CH_DISABLED is set in addition to another error interrupt flag)."]
pub type ACTIVE_CH_DISABLED_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE_CH_DISABLED` writer - Activated (set to '1') when the channel is disabled and the data transfer engine is busy, i.e there is a pending trigger. This flag is also set if an input trigger is received at the same time another error occurs (in this case, ACT_CH_DISABLED is set in addition to another error interrupt flag)."]
pub type ACTIVE_CH_DISABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `DESCR_BUS_ERROR` reader - Activated (set to '1') on a bus error for a load of the descriptor."]
pub type DESCR_BUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `DESCR_BUS_ERROR` writer - Activated (set to '1') on a bus error for a load of the descriptor."]
pub type DESCR_BUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
    #[inline(always)]
    pub fn completion(&self) -> COMPLETION_R {
        COMPLETION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Activated (set to '1') on a bus error for a load from the source."]
    #[inline(always)]
    pub fn src_bus_error(&self) -> SRC_BUS_ERROR_R {
        SRC_BUS_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Activated (set to '1') on a bus error for a store to the destination. Also activated when an AXI write response is received although there is no outstanding write transaction."]
    #[inline(always)]
    pub fn dst_bus_error(&self) -> DST_BUS_ERROR_R {
        DST_BUS_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Activated (set to '1') when the descriptor type is invalid."]
    #[inline(always)]
    pub fn invalid_descr_type(&self) -> INVALID_DESCR_TYPE_R {
        INVALID_DESCR_TYPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1'), CH_CURR_PTR is '0', and the channel is triggered."]
    #[inline(always)]
    pub fn curr_ptr_null(&self) -> CURR_PTR_NULL_R {
        CURR_PTR_NULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Activated (set to '1') when the channel is disabled and the data transfer engine is busy, i.e there is a pending trigger. This flag is also set if an input trigger is received at the same time another error occurs (in this case, ACT_CH_DISABLED is set in addition to another error interrupt flag)."]
    #[inline(always)]
    pub fn active_ch_disabled(&self) -> ACTIVE_CH_DISABLED_R {
        ACTIVE_CH_DISABLED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Activated (set to '1') on a bus error for a load of the descriptor."]
    #[inline(always)]
    pub fn descr_bus_error(&self) -> DESCR_BUS_ERROR_R {
        DESCR_BUS_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
    #[inline(always)]
    #[must_use]
    pub fn completion(&mut self) -> COMPLETION_W<0> {
        COMPLETION_W::new(self)
    }
    #[doc = "Bit 1 - Activated (set to '1') on a bus error for a load from the source."]
    #[inline(always)]
    #[must_use]
    pub fn src_bus_error(&mut self) -> SRC_BUS_ERROR_W<1> {
        SRC_BUS_ERROR_W::new(self)
    }
    #[doc = "Bit 2 - Activated (set to '1') on a bus error for a store to the destination. Also activated when an AXI write response is received although there is no outstanding write transaction."]
    #[inline(always)]
    #[must_use]
    pub fn dst_bus_error(&mut self) -> DST_BUS_ERROR_W<2> {
        DST_BUS_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - Activated (set to '1') when the descriptor type is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn invalid_descr_type(&mut self) -> INVALID_DESCR_TYPE_W<3> {
        INVALID_DESCR_TYPE_W::new(self)
    }
    #[doc = "Bit 5 - Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1'), CH_CURR_PTR is '0', and the channel is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn curr_ptr_null(&mut self) -> CURR_PTR_NULL_W<5> {
        CURR_PTR_NULL_W::new(self)
    }
    #[doc = "Bit 6 - Activated (set to '1') when the channel is disabled and the data transfer engine is busy, i.e there is a pending trigger. This flag is also set if an input trigger is received at the same time another error occurs (in this case, ACT_CH_DISABLED is set in addition to another error interrupt flag)."]
    #[inline(always)]
    #[must_use]
    pub fn active_ch_disabled(&mut self) -> ACTIVE_CH_DISABLED_W<6> {
        ACTIVE_CH_DISABLED_W::new(self)
    }
    #[doc = "Bit 7 - Activated (set to '1') on a bus error for a load of the descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn descr_bus_error(&mut self) -> DESCR_BUS_ERROR_W<7> {
        DESCR_BUS_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
