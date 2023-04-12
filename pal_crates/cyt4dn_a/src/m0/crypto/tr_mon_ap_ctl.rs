#[doc = "Register `TR_MON_AP_CTL` reader"]
pub struct R(crate::R<TR_MON_AP_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_MON_AP_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_MON_AP_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_MON_AP_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_MON_AP_CTL` writer"]
pub struct W(crate::W<TR_MON_AP_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_MON_AP_CTL_SPEC>;
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
impl From<crate::W<TR_MON_AP_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_MON_AP_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CUTOFF_COUNT16` reader - Cutoff count (legal range is \\[1, 65535\\]). '0': Illegal. '1': 1 occurrence. ... '65535': 65535 occurrences."]
pub type CUTOFF_COUNT16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CUTOFF_COUNT16` writer - Cutoff count (legal range is \\[1, 65535\\]). '0': Illegal. '1': 1 occurrence. ... '65535': 65535 occurrences."]
pub type CUTOFF_COUNT16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TR_MON_AP_CTL_SPEC, u16, u16, 16, O>;
#[doc = "Field `WINDOW_SIZE` reader - Window size (minus 1) : '0': 1 bit. ... '65535': 65536 bits."]
pub type WINDOW_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WINDOW_SIZE` writer - Window size (minus 1) : '0': 1 bit. ... '65535': 65536 bits."]
pub type WINDOW_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TR_MON_AP_CTL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Cutoff count (legal range is \\[1, 65535\\]). '0': Illegal. '1': 1 occurrence. ... '65535': 65535 occurrences."]
    #[inline(always)]
    pub fn cutoff_count16(&self) -> CUTOFF_COUNT16_R {
        CUTOFF_COUNT16_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Window size (minus 1) : '0': 1 bit. ... '65535': 65536 bits."]
    #[inline(always)]
    pub fn window_size(&self) -> WINDOW_SIZE_R {
        WINDOW_SIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Cutoff count (legal range is \\[1, 65535\\]). '0': Illegal. '1': 1 occurrence. ... '65535': 65535 occurrences."]
    #[inline(always)]
    #[must_use]
    pub fn cutoff_count16(&mut self) -> CUTOFF_COUNT16_W<0> {
        CUTOFF_COUNT16_W::new(self)
    }
    #[doc = "Bits 16:31 - Window size (minus 1) : '0': 1 bit. ... '65535': 65536 bits."]
    #[inline(always)]
    #[must_use]
    pub fn window_size(&mut self) -> WINDOW_SIZE_W<16> {
        WINDOW_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "True random monitor AP control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_mon_ap_ctl](index.html) module"]
pub struct TR_MON_AP_CTL_SPEC;
impl crate::RegisterSpec for TR_MON_AP_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_mon_ap_ctl::R](R) reader structure"]
impl crate::Readable for TR_MON_AP_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_mon_ap_ctl::W](W) writer structure"]
impl crate::Writable for TR_MON_AP_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_MON_AP_CTL to value 0xffff_ffff"]
impl crate::Resettable for TR_MON_AP_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
