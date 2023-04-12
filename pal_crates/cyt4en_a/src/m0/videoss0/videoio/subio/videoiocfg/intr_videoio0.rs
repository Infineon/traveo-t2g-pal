#[doc = "Register `INTR_VIDEOIO0` reader"]
pub struct R(crate::R<INTR_VIDEOIO0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_VIDEOIO0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_VIDEOIO0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_VIDEOIO0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_VIDEOIO0` writer"]
pub struct W(crate::W<INTR_VIDEOIO0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_VIDEOIO0_SPEC>;
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
impl From<crate::W<INTR_VIDEOIO0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_VIDEOIO0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_VIDEOIO0` reader - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_VIDEOIO0_SET is written. By writing 1 the corresponding bit is cleared. Bit 0 = ExtDst0_ShdLoad Bit 1 = ExtDst0_FrameComplete Bit 2 = ExtDst0_SeqComplete Bit 3 = ExtDst4_ShdLoad Bit 4 = ExtDst4_FrameComplete Bit 5 = ExtDst4_SeqComplete Bit 6 = ExtDst1_ShdLoad Bit 7 = ExtDst1_FrameComplete Bit 8 = ExtDst1_SeqComplete Bit 9 = ExtDst5_ShdLoad Bit 10 = ExtDst5_FrameComplete Bit 11 = ExtDst5_SeqComplete Bit 12 = Store4_ShdLoad Bit 13 = Store4_FrameComplete Bit 14 = Store4_SeqComplete Bit 15 = Histogram4_Valid Bit 16 = FrameDump_Error Bit 17 = FrameCap0_Sync_On Bit 18 = FrameCap0_Sync_Off Bit 19 = ItuIfc0_Error Bit 20 = GeneralPurpose0 Bit 21 = GeneralPurpose1 Bit 22 = GeneralPurpose2 Bit 23 = GeneralPurpose3"]
pub type INTR_VIDEOIO0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTR_VIDEOIO0` writer - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_VIDEOIO0_SET is written. By writing 1 the corresponding bit is cleared. Bit 0 = ExtDst0_ShdLoad Bit 1 = ExtDst0_FrameComplete Bit 2 = ExtDst0_SeqComplete Bit 3 = ExtDst4_ShdLoad Bit 4 = ExtDst4_FrameComplete Bit 5 = ExtDst4_SeqComplete Bit 6 = ExtDst1_ShdLoad Bit 7 = ExtDst1_FrameComplete Bit 8 = ExtDst1_SeqComplete Bit 9 = ExtDst5_ShdLoad Bit 10 = ExtDst5_FrameComplete Bit 11 = ExtDst5_SeqComplete Bit 12 = Store4_ShdLoad Bit 13 = Store4_FrameComplete Bit 14 = Store4_SeqComplete Bit 15 = Histogram4_Valid Bit 16 = FrameDump_Error Bit 17 = FrameCap0_Sync_On Bit 18 = FrameCap0_Sync_Off Bit 19 = ItuIfc0_Error Bit 20 = GeneralPurpose0 Bit 21 = GeneralPurpose1 Bit 22 = GeneralPurpose2 Bit 23 = GeneralPurpose3"]
pub type INTR_VIDEOIO0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_VIDEOIO0_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_VIDEOIO0_SET is written. By writing 1 the corresponding bit is cleared. Bit 0 = ExtDst0_ShdLoad Bit 1 = ExtDst0_FrameComplete Bit 2 = ExtDst0_SeqComplete Bit 3 = ExtDst4_ShdLoad Bit 4 = ExtDst4_FrameComplete Bit 5 = ExtDst4_SeqComplete Bit 6 = ExtDst1_ShdLoad Bit 7 = ExtDst1_FrameComplete Bit 8 = ExtDst1_SeqComplete Bit 9 = ExtDst5_ShdLoad Bit 10 = ExtDst5_FrameComplete Bit 11 = ExtDst5_SeqComplete Bit 12 = Store4_ShdLoad Bit 13 = Store4_FrameComplete Bit 14 = Store4_SeqComplete Bit 15 = Histogram4_Valid Bit 16 = FrameDump_Error Bit 17 = FrameCap0_Sync_On Bit 18 = FrameCap0_Sync_Off Bit 19 = ItuIfc0_Error Bit 20 = GeneralPurpose0 Bit 21 = GeneralPurpose1 Bit 22 = GeneralPurpose2 Bit 23 = GeneralPurpose3"]
    #[inline(always)]
    pub fn intr_videoio0(&self) -> INTR_VIDEOIO0_R {
        INTR_VIDEOIO0_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_VIDEOIO0_SET is written. By writing 1 the corresponding bit is cleared. Bit 0 = ExtDst0_ShdLoad Bit 1 = ExtDst0_FrameComplete Bit 2 = ExtDst0_SeqComplete Bit 3 = ExtDst4_ShdLoad Bit 4 = ExtDst4_FrameComplete Bit 5 = ExtDst4_SeqComplete Bit 6 = ExtDst1_ShdLoad Bit 7 = ExtDst1_FrameComplete Bit 8 = ExtDst1_SeqComplete Bit 9 = ExtDst5_ShdLoad Bit 10 = ExtDst5_FrameComplete Bit 11 = ExtDst5_SeqComplete Bit 12 = Store4_ShdLoad Bit 13 = Store4_FrameComplete Bit 14 = Store4_SeqComplete Bit 15 = Histogram4_Valid Bit 16 = FrameDump_Error Bit 17 = FrameCap0_Sync_On Bit 18 = FrameCap0_Sync_Off Bit 19 = ItuIfc0_Error Bit 20 = GeneralPurpose0 Bit 21 = GeneralPurpose1 Bit 22 = GeneralPurpose2 Bit 23 = GeneralPurpose3"]
    #[inline(always)]
    #[must_use]
    pub fn intr_videoio0(&mut self) -> INTR_VIDEOIO0_W<0> {
        INTR_VIDEOIO0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_videoio0](index.html) module"]
pub struct INTR_VIDEOIO0_SPEC;
impl crate::RegisterSpec for INTR_VIDEOIO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_videoio0::R](R) reader structure"]
impl crate::Readable for INTR_VIDEOIO0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_videoio0::W](W) writer structure"]
impl crate::Writable for INTR_VIDEOIO0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_VIDEOIO0 to value 0"]
impl crate::Resettable for INTR_VIDEOIO0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
