#[doc = "Register `UNDERSIZE_FRAMES` reader"]
pub struct R(crate::R<UNDERSIZE_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNDERSIZE_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNDERSIZE_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNDERSIZE_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_UNDERSIZE` reader - Undersize frames received - a 10 bit register counting the number of frames received less than 64 bytes in length (10/100 mode or gigabit mode, full duplex) that do not have either a CRC error or an alignment error. In gigabit mode, half duplex, this register counts either frames not conforming to the minimum slot time of 512 bytes or frames not conforming to the minimum frame size once bursting is active."]
pub type COUNT_UNDERSIZE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Undersize frames received - a 10 bit register counting the number of frames received less than 64 bytes in length (10/100 mode or gigabit mode, full duplex) that do not have either a CRC error or an alignment error. In gigabit mode, half duplex, this register counts either frames not conforming to the minimum slot time of 512 bytes or frames not conforming to the minimum frame size once bursting is active."]
    #[inline(always)]
    pub fn count_undersize(&self) -> COUNT_UNDERSIZE_R {
        COUNT_UNDERSIZE_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Undersized Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [undersize_frames](index.html) module"]
pub struct UNDERSIZE_FRAMES_SPEC;
impl crate::RegisterSpec for UNDERSIZE_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [undersize_frames::R](R) reader structure"]
impl crate::Readable for UNDERSIZE_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UNDERSIZE_FRAMES to value 0"]
impl crate::Resettable for UNDERSIZE_FRAMES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
