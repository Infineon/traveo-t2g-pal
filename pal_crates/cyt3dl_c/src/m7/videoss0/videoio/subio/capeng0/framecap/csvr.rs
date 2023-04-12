#[doc = "Register `CSVR` reader"]
pub struct R(crate::R<CSVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSVR` writer"]
pub struct W(crate::W<CSVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSVR_SPEC>;
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
impl From<crate::W<CSVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSVENABLE` reader - CsvEnable=1, start a capture clock supervision."]
pub type CSVENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CSVENABLE` writer - CsvEnable=1, start a capture clock supervision."]
pub type CSVENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSVR_SPEC, bool, O>;
#[doc = "Field `CSVTIMEOUT` reader - setting a time limitation for waiting the recovery of capture clock. The value should be bigger than 11 or equal to 11, otherwise it reports fake clk_cap loss."]
pub type CSVTIMEOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CSVTIMEOUT` writer - setting a time limitation for waiting the recovery of capture clock. The value should be bigger than 11 or equal to 11, otherwise it reports fake clk_cap loss."]
pub type CSVTIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSVR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - CsvEnable=1, start a capture clock supervision."]
    #[inline(always)]
    pub fn csvenable(&self) -> CSVENABLE_R {
        CSVENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - setting a time limitation for waiting the recovery of capture clock. The value should be bigger than 11 or equal to 11, otherwise it reports fake clk_cap loss."]
    #[inline(always)]
    pub fn csvtimeout(&self) -> CSVTIMEOUT_R {
        CSVTIMEOUT_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - CsvEnable=1, start a capture clock supervision."]
    #[inline(always)]
    #[must_use]
    pub fn csvenable(&mut self) -> CSVENABLE_W<0> {
        CSVENABLE_W::new(self)
    }
    #[doc = "Bits 1:10 - setting a time limitation for waiting the recovery of capture clock. The value should be bigger than 11 or equal to 11, otherwise it reports fake clk_cap loss."]
    #[inline(always)]
    #[must_use]
    pub fn csvtimeout(&mut self) -> CSVTIMEOUT_W<1> {
        CSVTIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Framecap control register for capture clock supervision\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csvr](index.html) module"]
pub struct CSVR_SPEC;
impl crate::RegisterSpec for CSVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csvr::R](R) reader structure"]
impl crate::Readable for CSVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csvr::W](W) writer structure"]
impl crate::Writable for CSVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSVR to value 0x22"]
impl crate::Resettable for CSVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x22;
}
