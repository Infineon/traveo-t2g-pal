#[doc = "Register `FRF` reader"]
pub struct R(crate::R<FRF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRF` writer"]
pub struct W(crate::W<FRF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRF_SPEC>;
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
impl From<crate::W<FRF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH` reader - Channel Filter 11 = no reception 10 = receive only on channel A 01 = receive only on channel B 00 = receive on both channels"]
pub type CH_R = crate::FieldReader<u8, CH_A>;
#[doc = "Channel Filter 11 = no reception 10 = receive only on channel A 01 = receive only on channel B 00 = receive on both channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH_A {
    #[doc = "0: N/A"]
    RECEIVE_ON_BOTH_CHANNELS = 0,
    #[doc = "1: N/A"]
    RECEIVE_ONLY_ON_CHANNEL_B = 1,
    #[doc = "2: N/A"]
    RECEIVE_ONLY_ON_CHANNEL_A = 2,
    #[doc = "3: N/A"]
    NO_RECEPTION = 3,
}
impl From<CH_A> for u8 {
    #[inline(always)]
    fn from(variant: CH_A) -> Self {
        variant as _
    }
}
impl CH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_A {
        match self.bits {
            0 => CH_A::RECEIVE_ON_BOTH_CHANNELS,
            1 => CH_A::RECEIVE_ONLY_ON_CHANNEL_B,
            2 => CH_A::RECEIVE_ONLY_ON_CHANNEL_A,
            3 => CH_A::NO_RECEPTION,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVE_ON_BOTH_CHANNELS`"]
    #[inline(always)]
    pub fn is_receive_on_both_channels(&self) -> bool {
        *self == CH_A::RECEIVE_ON_BOTH_CHANNELS
    }
    #[doc = "Checks if the value of the field is `RECEIVE_ONLY_ON_CHANNEL_B`"]
    #[inline(always)]
    pub fn is_receive_only_on_channel_b(&self) -> bool {
        *self == CH_A::RECEIVE_ONLY_ON_CHANNEL_B
    }
    #[doc = "Checks if the value of the field is `RECEIVE_ONLY_ON_CHANNEL_A`"]
    #[inline(always)]
    pub fn is_receive_only_on_channel_a(&self) -> bool {
        *self == CH_A::RECEIVE_ONLY_ON_CHANNEL_A
    }
    #[doc = "Checks if the value of the field is `NO_RECEPTION`"]
    #[inline(always)]
    pub fn is_no_reception(&self) -> bool {
        *self == CH_A::NO_RECEPTION
    }
}
#[doc = "Field `CH` writer - Channel Filter 11 = no reception 10 = receive only on channel A 01 = receive only on channel B 00 = receive on both channels"]
pub type CH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FRF_SPEC, u8, CH_A, 2, O>;
impl<'a, const O: u8> CH_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn receive_on_both_channels(self) -> &'a mut W {
        self.variant(CH_A::RECEIVE_ON_BOTH_CHANNELS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn receive_only_on_channel_b(self) -> &'a mut W {
        self.variant(CH_A::RECEIVE_ONLY_ON_CHANNEL_B)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn receive_only_on_channel_a(self) -> &'a mut W {
        self.variant(CH_A::RECEIVE_ONLY_ON_CHANNEL_A)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_reception(self) -> &'a mut W {
        self.variant(CH_A::NO_RECEPTION)
    }
}
#[doc = "Field `FID_` reader - Frame ID Filter Determines the frame ID to be rejected by the FIFO. With the additional configuration of register FRFM, the corresponding frame ID filter bits are ignored, which results in further rejected frame IDs. When FRFM.MFID\\[10:0\\]
is zero, a frame ID filter value of zero means that no frame ID is rejected. 0...2047 = Frame ID filter values"]
pub type FID__R = crate::FieldReader<u16, u16>;
#[doc = "Field `FID_` writer - Frame ID Filter Determines the frame ID to be rejected by the FIFO. With the additional configuration of register FRFM, the corresponding frame ID filter bits are ignored, which results in further rejected frame IDs. When FRFM.MFID\\[10:0\\]
is zero, a frame ID filter value of zero means that no frame ID is rejected. 0...2047 = Frame ID filter values"]
pub type FID__W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRF_SPEC, u16, u16, 11, O>;
#[doc = "Field `CYF` reader - Cycle Counter Filter The 7-bit cycle counter filter determines the cycle set to which frame ID and channel rejection filter are applied. In cycles not belonging to the cycle set specified by CYF\\[6:0\\], all frames are rejected. For details about the configuration of the cycle counter filter see \\[01\\]Section 5.7.2 Cycle Counter Filtering."]
pub type CYF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CYF` writer - Cycle Counter Filter The 7-bit cycle counter filter determines the cycle set to which frame ID and channel rejection filter are applied. In cycles not belonging to the cycle set specified by CYF\\[6:0\\], all frames are rejected. For details about the configuration of the cycle counter filter see \\[01\\]Section 5.7.2 Cycle Counter Filtering."]
pub type CYF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRF_SPEC, u8, u8, 7, O>;
#[doc = "Field `RSS` reader - Reject in Static Segment If this bit is set, the FIFO is used only for the dynamic segment. 1 = Reject messages in static segment 0 = FIFO also used for static segment"]
pub type RSS_R = crate::BitReader<RSS_A>;
#[doc = "Reject in Static Segment If this bit is set, the FIFO is used only for the dynamic segment. 1 = Reject messages in static segment 0 = FIFO also used for static segment\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSS_A {
    #[doc = "0: N/A"]
    FIFO_STATIC_SEGMENT_STORAGE_ENABLED = 0,
    #[doc = "1: N/A"]
    FIFO_STATIC_SEGMENT_STORAGE_DISABLED = 1,
}
impl From<RSS_A> for bool {
    #[inline(always)]
    fn from(variant: RSS_A) -> Self {
        variant as u8 != 0
    }
}
impl RSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSS_A {
        match self.bits {
            false => RSS_A::FIFO_STATIC_SEGMENT_STORAGE_ENABLED,
            true => RSS_A::FIFO_STATIC_SEGMENT_STORAGE_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_STATIC_SEGMENT_STORAGE_ENABLED`"]
    #[inline(always)]
    pub fn is_fifo_static_segment_storage_enabled(&self) -> bool {
        *self == RSS_A::FIFO_STATIC_SEGMENT_STORAGE_ENABLED
    }
    #[doc = "Checks if the value of the field is `FIFO_STATIC_SEGMENT_STORAGE_DISABLED`"]
    #[inline(always)]
    pub fn is_fifo_static_segment_storage_disabled(&self) -> bool {
        *self == RSS_A::FIFO_STATIC_SEGMENT_STORAGE_DISABLED
    }
}
#[doc = "Field `RSS` writer - Reject in Static Segment If this bit is set, the FIFO is used only for the dynamic segment. 1 = Reject messages in static segment 0 = FIFO also used for static segment"]
pub type RSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRF_SPEC, RSS_A, O>;
impl<'a, const O: u8> RSS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn fifo_static_segment_storage_enabled(self) -> &'a mut W {
        self.variant(RSS_A::FIFO_STATIC_SEGMENT_STORAGE_ENABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn fifo_static_segment_storage_disabled(self) -> &'a mut W {
        self.variant(RSS_A::FIFO_STATIC_SEGMENT_STORAGE_DISABLED)
    }
}
#[doc = "Field `RNF` reader - Reject Null Frames If this bit is set, received null frames are not stored in the FIFO. 1 = Reject all null frames 0 = Null frames are stored in the FIFO"]
pub type RNF_R = crate::BitReader<RNF_A>;
#[doc = "Reject Null Frames If this bit is set, received null frames are not stored in the FIFO. 1 = Reject all null frames 0 = Null frames are stored in the FIFO\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNF_A {
    #[doc = "0: N/A"]
    NULL_FRAMES_FIFO_STORAGE_ENABLED = 0,
    #[doc = "1: N/A"]
    NULL_FRAMES_FIFO_STORAGE_DISABLED = 1,
}
impl From<RNF_A> for bool {
    #[inline(always)]
    fn from(variant: RNF_A) -> Self {
        variant as u8 != 0
    }
}
impl RNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNF_A {
        match self.bits {
            false => RNF_A::NULL_FRAMES_FIFO_STORAGE_ENABLED,
            true => RNF_A::NULL_FRAMES_FIFO_STORAGE_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NULL_FRAMES_FIFO_STORAGE_ENABLED`"]
    #[inline(always)]
    pub fn is_null_frames_fifo_storage_enabled(&self) -> bool {
        *self == RNF_A::NULL_FRAMES_FIFO_STORAGE_ENABLED
    }
    #[doc = "Checks if the value of the field is `NULL_FRAMES_FIFO_STORAGE_DISABLED`"]
    #[inline(always)]
    pub fn is_null_frames_fifo_storage_disabled(&self) -> bool {
        *self == RNF_A::NULL_FRAMES_FIFO_STORAGE_DISABLED
    }
}
#[doc = "Field `RNF` writer - Reject Null Frames If this bit is set, received null frames are not stored in the FIFO. 1 = Reject all null frames 0 = Null frames are stored in the FIFO"]
pub type RNF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRF_SPEC, RNF_A, O>;
impl<'a, const O: u8> RNF_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn null_frames_fifo_storage_enabled(self) -> &'a mut W {
        self.variant(RNF_A::NULL_FRAMES_FIFO_STORAGE_ENABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn null_frames_fifo_storage_disabled(self) -> &'a mut W {
        self.variant(RNF_A::NULL_FRAMES_FIFO_STORAGE_DISABLED)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel Filter 11 = no reception 10 = receive only on channel A 01 = receive only on channel B 00 = receive on both channels"]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:12 - Frame ID Filter Determines the frame ID to be rejected by the FIFO. With the additional configuration of register FRFM, the corresponding frame ID filter bits are ignored, which results in further rejected frame IDs. When FRFM.MFID\\[10:0\\]
is zero, a frame ID filter value of zero means that no frame ID is rejected. 0...2047 = Frame ID filter values"]
    #[inline(always)]
    pub fn fid_(&self) -> FID__R {
        FID__R::new(((self.bits >> 2) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - Cycle Counter Filter The 7-bit cycle counter filter determines the cycle set to which frame ID and channel rejection filter are applied. In cycles not belonging to the cycle set specified by CYF\\[6:0\\], all frames are rejected. For details about the configuration of the cycle counter filter see \\[01\\]Section 5.7.2 Cycle Counter Filtering."]
    #[inline(always)]
    pub fn cyf(&self) -> CYF_R {
        CYF_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Reject in Static Segment If this bit is set, the FIFO is used only for the dynamic segment. 1 = Reject messages in static segment 0 = FIFO also used for static segment"]
    #[inline(always)]
    pub fn rss(&self) -> RSS_R {
        RSS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reject Null Frames If this bit is set, received null frames are not stored in the FIFO. 1 = Reject all null frames 0 = Null frames are stored in the FIFO"]
    #[inline(always)]
    pub fn rnf(&self) -> RNF_R {
        RNF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel Filter 11 = no reception 10 = receive only on channel A 01 = receive only on channel B 00 = receive on both channels"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> CH_W<0> {
        CH_W::new(self)
    }
    #[doc = "Bits 2:12 - Frame ID Filter Determines the frame ID to be rejected by the FIFO. With the additional configuration of register FRFM, the corresponding frame ID filter bits are ignored, which results in further rejected frame IDs. When FRFM.MFID\\[10:0\\]
is zero, a frame ID filter value of zero means that no frame ID is rejected. 0...2047 = Frame ID filter values"]
    #[inline(always)]
    #[must_use]
    pub fn fid_(&mut self) -> FID__W<2> {
        FID__W::new(self)
    }
    #[doc = "Bits 16:22 - Cycle Counter Filter The 7-bit cycle counter filter determines the cycle set to which frame ID and channel rejection filter are applied. In cycles not belonging to the cycle set specified by CYF\\[6:0\\], all frames are rejected. For details about the configuration of the cycle counter filter see \\[01\\]Section 5.7.2 Cycle Counter Filtering."]
    #[inline(always)]
    #[must_use]
    pub fn cyf(&mut self) -> CYF_W<16> {
        CYF_W::new(self)
    }
    #[doc = "Bit 23 - Reject in Static Segment If this bit is set, the FIFO is used only for the dynamic segment. 1 = Reject messages in static segment 0 = FIFO also used for static segment"]
    #[inline(always)]
    #[must_use]
    pub fn rss(&mut self) -> RSS_W<23> {
        RSS_W::new(self)
    }
    #[doc = "Bit 24 - Reject Null Frames If this bit is set, received null frames are not stored in the FIFO. 1 = Reject all null frames 0 = Null frames are stored in the FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rnf(&mut self) -> RNF_W<24> {
        RNF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Rejection Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frf](index.html) module"]
pub struct FRF_SPEC;
impl crate::RegisterSpec for FRF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frf::R](R) reader structure"]
impl crate::Readable for FRF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frf::W](W) writer structure"]
impl crate::Writable for FRF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRF to value 0x0180_0000"]
impl crate::Resettable for FRF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0180_0000;
}
