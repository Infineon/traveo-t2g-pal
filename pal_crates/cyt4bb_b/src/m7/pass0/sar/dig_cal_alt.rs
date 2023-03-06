#[doc = "Register `DIG_CAL_ALT` reader"]
pub struct R(crate::R<DIG_CAL_ALT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_CAL_ALT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_CAL_ALT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_CAL_ALT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIG_CAL_ALT` writer"]
pub struct W(crate::W<DIG_CAL_ALT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_CAL_ALT_SPEC>;
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
impl From<crate::W<DIG_CAL_ALT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_CAL_ALT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOFFSET` reader - See corresponding DIG_CAL field"]
pub type DOFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DOFFSET` writer - See corresponding DIG_CAL field"]
pub type DOFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIG_CAL_ALT_SPEC, u16, u16, 12, O>;
#[doc = "Field `DGAIN` reader - See corresponding DIG_CAL field"]
pub type DGAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DGAIN` writer - See corresponding DIG_CAL field"]
pub type DGAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIG_CAL_ALT_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:11 - See corresponding DIG_CAL field"]
    #[inline(always)]
    pub fn doffset(&self) -> DOFFSET_R {
        DOFFSET_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21 - See corresponding DIG_CAL field"]
    #[inline(always)]
    pub fn dgain(&self) -> DGAIN_R {
        DGAIN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - See corresponding DIG_CAL field"]
    #[inline(always)]
    #[must_use]
    pub fn doffset(&mut self) -> DOFFSET_W<0> {
        DOFFSET_W::new(self)
    }
    #[doc = "Bits 16:21 - See corresponding DIG_CAL field"]
    #[inline(always)]
    #[must_use]
    pub fn dgain(&mut self) -> DGAIN_W<16> {
        DGAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate digital calibration values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_cal_alt](index.html) module"]
pub struct DIG_CAL_ALT_SPEC;
impl crate::RegisterSpec for DIG_CAL_ALT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_cal_alt::R](R) reader structure"]
impl crate::Readable for DIG_CAL_ALT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_cal_alt::W](W) writer structure"]
impl crate::Writable for DIG_CAL_ALT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIG_CAL_ALT to value 0"]
impl crate::Resettable for DIG_CAL_ALT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
