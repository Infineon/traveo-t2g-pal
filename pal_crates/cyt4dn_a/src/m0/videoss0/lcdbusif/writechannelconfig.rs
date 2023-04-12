#[doc = "Register `WRITECHANNELCONFIG` reader"]
pub struct R(crate::R<WRITECHANNELCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITECHANNELCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITECHANNELCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITECHANNELCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRITECHANNELCONFIG` writer"]
pub struct W(crate::W<WRITECHANNELCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITECHANNELCONFIG_SPEC>;
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
impl From<crate::W<WRITECHANNELCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITECHANNELCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITECHANNELPACKING` reader - Set write channel packing configuration."]
pub type WRITECHANNELPACKING_R = crate::FieldReader<u8, WRITECHANNELPACKING_A>;
#[doc = "Set write channel packing configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRITECHANNELPACKING_A {
    #[doc = "0: Write LCDBus read data as mapped into reception fifo."]
    DISABLED = 0,
    #[doc = "1: Pack 16-bit pixel data into 32-bit memory transfers."]
    BPP16 = 1,
    #[doc = "2: Pack 8-bit pixel data into 32-bit memory transfers."]
    BPP8 = 2,
    #[doc = "3: N/A"]
    RSVD = 3,
    #[doc = "4: Pack 18-bit pixel data into 32-bit memory transfers."]
    BPP18TIGHT = 4,
}
impl From<WRITECHANNELPACKING_A> for u8 {
    #[inline(always)]
    fn from(variant: WRITECHANNELPACKING_A) -> Self {
        variant as _
    }
}
impl WRITECHANNELPACKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WRITECHANNELPACKING_A> {
        match self.bits {
            0 => Some(WRITECHANNELPACKING_A::DISABLED),
            1 => Some(WRITECHANNELPACKING_A::BPP16),
            2 => Some(WRITECHANNELPACKING_A::BPP8),
            3 => Some(WRITECHANNELPACKING_A::RSVD),
            4 => Some(WRITECHANNELPACKING_A::BPP18TIGHT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WRITECHANNELPACKING_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `BPP16`"]
    #[inline(always)]
    pub fn is_bpp16(&self) -> bool {
        *self == WRITECHANNELPACKING_A::BPP16
    }
    #[doc = "Checks if the value of the field is `BPP8`"]
    #[inline(always)]
    pub fn is_bpp8(&self) -> bool {
        *self == WRITECHANNELPACKING_A::BPP8
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == WRITECHANNELPACKING_A::RSVD
    }
    #[doc = "Checks if the value of the field is `BPP18TIGHT`"]
    #[inline(always)]
    pub fn is_bpp18tight(&self) -> bool {
        *self == WRITECHANNELPACKING_A::BPP18TIGHT
    }
}
#[doc = "Field `WRITECHANNELPACKING` writer - Set write channel packing configuration."]
pub type WRITECHANNELPACKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WRITECHANNELCONFIG_SPEC, u8, WRITECHANNELPACKING_A, 3, O>;
impl<'a, const O: u8> WRITECHANNELPACKING_W<'a, O> {
    #[doc = "Write LCDBus read data as mapped into reception fifo."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WRITECHANNELPACKING_A::DISABLED)
    }
    #[doc = "Pack 16-bit pixel data into 32-bit memory transfers."]
    #[inline(always)]
    pub fn bpp16(self) -> &'a mut W {
        self.variant(WRITECHANNELPACKING_A::BPP16)
    }
    #[doc = "Pack 8-bit pixel data into 32-bit memory transfers."]
    #[inline(always)]
    pub fn bpp8(self) -> &'a mut W {
        self.variant(WRITECHANNELPACKING_A::BPP8)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(WRITECHANNELPACKING_A::RSVD)
    }
    #[doc = "Pack 18-bit pixel data into 32-bit memory transfers."]
    #[inline(always)]
    pub fn bpp18tight(self) -> &'a mut W {
        self.variant(WRITECHANNELPACKING_A::BPP18TIGHT)
    }
}
#[doc = "Field `WRITECHANNELMAXBURSTLENGTH` reader - Default burstlength for write channel requests. Will always be used except for the last access if remaining words is less than WriteChannelMaxBurstLength."]
pub type WRITECHANNELMAXBURSTLENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRITECHANNELMAXBURSTLENGTH` writer - Default burstlength for write channel requests. Will always be used except for the last access if remaining words is less than WriteChannelMaxBurstLength."]
pub type WRITECHANNELMAXBURSTLENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WRITECHANNELCONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `WRITECHANNELENABLE` reader - Enable write channel interface. Locks the RxFifo from configuration interface."]
pub type WRITECHANNELENABLE_R = crate::BitReader<bool>;
#[doc = "Field `WRITECHANNELENABLE` writer - Enable write channel interface. Locks the RxFifo from configuration interface."]
pub type WRITECHANNELENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WRITECHANNELCONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Set write channel packing configuration."]
    #[inline(always)]
    pub fn writechannelpacking(&self) -> WRITECHANNELPACKING_R {
        WRITECHANNELPACKING_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 24:28 - Default burstlength for write channel requests. Will always be used except for the last access if remaining words is less than WriteChannelMaxBurstLength."]
    #[inline(always)]
    pub fn writechannelmaxburstlength(&self) -> WRITECHANNELMAXBURSTLENGTH_R {
        WRITECHANNELMAXBURSTLENGTH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enable write channel interface. Locks the RxFifo from configuration interface."]
    #[inline(always)]
    pub fn writechannelenable(&self) -> WRITECHANNELENABLE_R {
        WRITECHANNELENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set write channel packing configuration."]
    #[inline(always)]
    #[must_use]
    pub fn writechannelpacking(&mut self) -> WRITECHANNELPACKING_W<0> {
        WRITECHANNELPACKING_W::new(self)
    }
    #[doc = "Bits 24:28 - Default burstlength for write channel requests. Will always be used except for the last access if remaining words is less than WriteChannelMaxBurstLength."]
    #[inline(always)]
    #[must_use]
    pub fn writechannelmaxburstlength(&mut self) -> WRITECHANNELMAXBURSTLENGTH_W<24> {
        WRITECHANNELMAXBURSTLENGTH_W::new(self)
    }
    #[doc = "Bit 31 - Enable write channel interface. Locks the RxFifo from configuration interface."]
    #[inline(always)]
    #[must_use]
    pub fn writechannelenable(&mut self) -> WRITECHANNELENABLE_W<31> {
        WRITECHANNELENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Channel Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writechannelconfig](index.html) module"]
pub struct WRITECHANNELCONFIG_SPEC;
impl crate::RegisterSpec for WRITECHANNELCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [writechannelconfig::R](R) reader structure"]
impl crate::Readable for WRITECHANNELCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [writechannelconfig::W](W) writer structure"]
impl crate::Writable for WRITECHANNELCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRITECHANNELCONFIG to value 0x0800_0000"]
impl crate::Resettable for WRITECHANNELCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800_0000;
}
