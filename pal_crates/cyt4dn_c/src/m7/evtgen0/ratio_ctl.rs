#[doc = "Register `RATIO_CTL` reader"]
pub struct R(crate::R<RATIO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RATIO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RATIO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RATIO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RATIO_CTL` writer"]
pub struct W(crate::W<RATIO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RATIO_CTL_SPEC>;
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
impl From<crate::W<RATIO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RATIO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DYNAMIC_MODE` reader - Weighted average calculation (only used when DYNAMIC is '1'): '0': new RATIO value = (RATIO + measurement + 1) / 2. '1': new RATIO value = (3*RATIO + measurement + 2) / 4. '2': new RATIO value = (7*RATIO + measurement + 4) / 8. '3': new RATIO value = (15*RATIO + measurement + 8) / 16. '4': new RATIO value = (31*RATIO + measurement + 16) / 32. '5': new RATIO value = (63*RATIO + measurement + 32) / 64. '6': new RATIO value = (127*RATIO + measurement + 64) / 128. '7': new RATIO value = (255*RATIO + measurement + 128) / 256. Note: 'measurement' (integer component only) is defined as: 256 * 'number of measured clk_ref_div cycles per clk_lf cycle'. The RATIO value (integer and fractional component) is defined as: 256*RATIO.INT16 + RATIO.FRAC8 (RATIO.INT16 = RATIO >> 8 and RATIO.FRAC8 = RATIO percent 256)."]
pub type DYNAMIC_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DYNAMIC_MODE` writer - Weighted average calculation (only used when DYNAMIC is '1'): '0': new RATIO value = (RATIO + measurement + 1) / 2. '1': new RATIO value = (3*RATIO + measurement + 2) / 4. '2': new RATIO value = (7*RATIO + measurement + 4) / 8. '3': new RATIO value = (15*RATIO + measurement + 8) / 16. '4': new RATIO value = (31*RATIO + measurement + 16) / 32. '5': new RATIO value = (63*RATIO + measurement + 32) / 64. '6': new RATIO value = (127*RATIO + measurement + 64) / 128. '7': new RATIO value = (255*RATIO + measurement + 128) / 256. Note: 'measurement' (integer component only) is defined as: 256 * 'number of measured clk_ref_div cycles per clk_lf cycle'. The RATIO value (integer and fractional component) is defined as: 256*RATIO.INT16 + RATIO.FRAC8 (RATIO.INT16 = RATIO >> 8 and RATIO.FRAC8 = RATIO percent 256)."]
pub type DYNAMIC_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RATIO_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DYNAMIC` reader - Specifies if RATIO_CTL.VALID and RATIO are under SW or HW control: '0': SW control. '1: HW control. Auto calibration is used to derive the RATIO value. HW measures the number of clk_ref_div cycles per clk_lf cycle. This measurement is combined with the current ratio value to calculate a new ratio value."]
pub type DYNAMIC_R = crate::BitReader<bool>;
#[doc = "Field `DYNAMIC` writer - Specifies if RATIO_CTL.VALID and RATIO are under SW or HW control: '0': SW control. '1: HW control. Auto calibration is used to derive the RATIO value. HW measures the number of clk_ref_div cycles per clk_lf cycle. This measurement is combined with the current ratio value to calculate a new ratio value."]
pub type DYNAMIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RATIO_CTL_SPEC, bool, O>;
#[doc = "Field `VALID` reader - Ratio value valid: '0': Invalid. '1': Valid. The RATIO register fields INT16 and FRAC8 are only valid when VALID is '1'."]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - Ratio value valid: '0': Invalid. '1': Valid. The RATIO register fields INT16 and FRAC8 are only valid when VALID is '1'."]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, RATIO_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 16:18 - Weighted average calculation (only used when DYNAMIC is '1'): '0': new RATIO value = (RATIO + measurement + 1) / 2. '1': new RATIO value = (3*RATIO + measurement + 2) / 4. '2': new RATIO value = (7*RATIO + measurement + 4) / 8. '3': new RATIO value = (15*RATIO + measurement + 8) / 16. '4': new RATIO value = (31*RATIO + measurement + 16) / 32. '5': new RATIO value = (63*RATIO + measurement + 32) / 64. '6': new RATIO value = (127*RATIO + measurement + 64) / 128. '7': new RATIO value = (255*RATIO + measurement + 128) / 256. Note: 'measurement' (integer component only) is defined as: 256 * 'number of measured clk_ref_div cycles per clk_lf cycle'. The RATIO value (integer and fractional component) is defined as: 256*RATIO.INT16 + RATIO.FRAC8 (RATIO.INT16 = RATIO >> 8 and RATIO.FRAC8 = RATIO percent 256)."]
    #[inline(always)]
    pub fn dynamic_mode(&self) -> DYNAMIC_MODE_R {
        DYNAMIC_MODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 30 - Specifies if RATIO_CTL.VALID and RATIO are under SW or HW control: '0': SW control. '1: HW control. Auto calibration is used to derive the RATIO value. HW measures the number of clk_ref_div cycles per clk_lf cycle. This measurement is combined with the current ratio value to calculate a new ratio value."]
    #[inline(always)]
    pub fn dynamic(&self) -> DYNAMIC_R {
        DYNAMIC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Ratio value valid: '0': Invalid. '1': Valid. The RATIO register fields INT16 and FRAC8 are only valid when VALID is '1'."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18 - Weighted average calculation (only used when DYNAMIC is '1'): '0': new RATIO value = (RATIO + measurement + 1) / 2. '1': new RATIO value = (3*RATIO + measurement + 2) / 4. '2': new RATIO value = (7*RATIO + measurement + 4) / 8. '3': new RATIO value = (15*RATIO + measurement + 8) / 16. '4': new RATIO value = (31*RATIO + measurement + 16) / 32. '5': new RATIO value = (63*RATIO + measurement + 32) / 64. '6': new RATIO value = (127*RATIO + measurement + 64) / 128. '7': new RATIO value = (255*RATIO + measurement + 128) / 256. Note: 'measurement' (integer component only) is defined as: 256 * 'number of measured clk_ref_div cycles per clk_lf cycle'. The RATIO value (integer and fractional component) is defined as: 256*RATIO.INT16 + RATIO.FRAC8 (RATIO.INT16 = RATIO >> 8 and RATIO.FRAC8 = RATIO percent 256)."]
    #[inline(always)]
    #[must_use]
    pub fn dynamic_mode(&mut self) -> DYNAMIC_MODE_W<16> {
        DYNAMIC_MODE_W::new(self)
    }
    #[doc = "Bit 30 - Specifies if RATIO_CTL.VALID and RATIO are under SW or HW control: '0': SW control. '1: HW control. Auto calibration is used to derive the RATIO value. HW measures the number of clk_ref_div cycles per clk_lf cycle. This measurement is combined with the current ratio value to calculate a new ratio value."]
    #[inline(always)]
    #[must_use]
    pub fn dynamic(&mut self) -> DYNAMIC_W<30> {
        DYNAMIC_W::new(self)
    }
    #[doc = "Bit 31 - Ratio value valid: '0': Invalid. '1': Valid. The RATIO register fields INT16 and FRAC8 are only valid when VALID is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<31> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ratio control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratio_ctl](index.html) module"]
pub struct RATIO_CTL_SPEC;
impl crate::RegisterSpec for RATIO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ratio_ctl::R](R) reader structure"]
impl crate::Readable for RATIO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ratio_ctl::W](W) writer structure"]
impl crate::Writable for RATIO_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RATIO_CTL to value 0"]
impl crate::Resettable for RATIO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
