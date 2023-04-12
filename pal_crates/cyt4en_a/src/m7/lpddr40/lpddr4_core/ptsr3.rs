#[doc = "Register `PTSR3` reader"]
pub struct R(crate::R<PTSR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR3` writer"]
pub struct W(crate::W<PTSR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR3_SPEC>;
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
impl From<crate::W<PTSR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTS0` reader - Gate setting for slice 0"]
pub type GTS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTS0` writer - Gate setting for slice 0"]
pub type GTS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR3_SPEC, u8, u8, 6, O>;
#[doc = "Field `GTS1` reader - Gate setting for slice 1"]
pub type GTS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTS1` writer - Gate setting for slice 1"]
pub type GTS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR3_SPEC, u8, u8, 6, O>;
#[doc = "Field `GTS2` reader - Gate setting for slice 2"]
pub type GTS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTS2` writer - Gate setting for slice 2"]
pub type GTS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR3_SPEC, u8, u8, 6, O>;
#[doc = "Field `GTS3` reader - Gate setting for slice 3"]
pub type GTS3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTS3` writer - Gate setting for slice 3"]
pub type GTS3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR3_SPEC, u8, u8, 6, O>;
#[doc = "Field `VREFDQWRR` reader - VREF-DQ range according to JESD209-4: 0B: VREF(DQ) Range\\[0\\]
enabled 1B: VREF(DQ) Range\\[1\\]
enabled"]
pub type VREFDQWRR_R = crate::BitReader<bool>;
#[doc = "Field `VREFDQWRR` writer - VREF-DQ range according to JESD209-4: 0B: VREF(DQ) Range\\[0\\]
enabled 1B: VREF(DQ) Range\\[1\\]
enabled"]
pub type VREFDQWRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTSR3_SPEC, bool, O>;
#[doc = "Field `VREFDQWRS` reader - VREF-DQ Setting"]
pub type VREFDQWRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFDQWRS` writer - VREF-DQ Setting"]
pub type VREFDQWRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR3_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Gate setting for slice 0"]
    #[inline(always)]
    pub fn gts0(&self) -> GTS0_R {
        GTS0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Gate setting for slice 1"]
    #[inline(always)]
    pub fn gts1(&self) -> GTS1_R {
        GTS1_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Gate setting for slice 2"]
    #[inline(always)]
    pub fn gts2(&self) -> GTS2_R {
        GTS2_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Gate setting for slice 3"]
    #[inline(always)]
    pub fn gts3(&self) -> GTS3_R {
        GTS3_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - VREF-DQ range according to JESD209-4: 0B: VREF(DQ) Range\\[0\\]
enabled 1B: VREF(DQ) Range\\[1\\]
enabled"]
    #[inline(always)]
    pub fn vrefdqwrr(&self) -> VREFDQWRR_R {
        VREFDQWRR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:30 - VREF-DQ Setting"]
    #[inline(always)]
    pub fn vrefdqwrs(&self) -> VREFDQWRS_R {
        VREFDQWRS_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Gate setting for slice 0"]
    #[inline(always)]
    #[must_use]
    pub fn gts0(&mut self) -> GTS0_W<0> {
        GTS0_W::new(self)
    }
    #[doc = "Bits 6:11 - Gate setting for slice 1"]
    #[inline(always)]
    #[must_use]
    pub fn gts1(&mut self) -> GTS1_W<6> {
        GTS1_W::new(self)
    }
    #[doc = "Bits 12:17 - Gate setting for slice 2"]
    #[inline(always)]
    #[must_use]
    pub fn gts2(&mut self) -> GTS2_W<12> {
        GTS2_W::new(self)
    }
    #[doc = "Bits 18:23 - Gate setting for slice 3"]
    #[inline(always)]
    #[must_use]
    pub fn gts3(&mut self) -> GTS3_W<18> {
        GTS3_W::new(self)
    }
    #[doc = "Bit 24 - VREF-DQ range according to JESD209-4: 0B: VREF(DQ) Range\\[0\\]
enabled 1B: VREF(DQ) Range\\[1\\]
enabled"]
    #[inline(always)]
    #[must_use]
    pub fn vrefdqwrr(&mut self) -> VREFDQWRR_W<24> {
        VREFDQWRR_W::new(self)
    }
    #[doc = "Bits 25:30 - VREF-DQ Setting"]
    #[inline(always)]
    #[must_use]
    pub fn vrefdqwrs(&mut self) -> VREFDQWRS_W<25> {
        VREFDQWRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr3](index.html) module"]
pub struct PTSR3_SPEC;
impl crate::RegisterSpec for PTSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr3::R](R) reader structure"]
impl crate::Readable for PTSR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr3::W](W) writer structure"]
impl crate::Writable for PTSR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR3 to value 0"]
impl crate::Resettable for PTSR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
