#[doc = "Register `TREG14` reader"]
pub struct R(crate::R<TREG14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG14` writer"]
pub struct W(crate::W<TREG14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG14_SPEC>;
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
impl From<crate::W<TREG14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_ODTUP` reader - N/A"]
pub type T_ODTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_ODTUP` writer - N/A"]
pub type T_ODTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG14_SPEC, u8, u8, 8, O>;
#[doc = "Field `T_VREFTIMESHORT` reader - VREF changing time for single steps. >= RU(tVREFca_Short/tfs_period)"]
pub type T_VREFTIMESHORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_VREFTIMESHORT` writer - VREF changing time for single steps. >= RU(tVREFca_Short/tfs_period)"]
pub type T_VREFTIMESHORT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TREG14_SPEC, u8, u8, 8, O>;
#[doc = "Field `T_VREFTIMELONG` reader - VREF changing time for multiple steps >= RU(tVREFca_Long/tfs_period)"]
pub type T_VREFTIMELONG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_VREFTIMELONG` writer - VREF changing time for multiple steps >= RU(tVREFca_Long/tfs_period)"]
pub type T_VREFTIMELONG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TREG14_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn t_odtup(&self) -> T_ODTUP_R {
        T_ODTUP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - VREF changing time for single steps. >= RU(tVREFca_Short/tfs_period)"]
    #[inline(always)]
    pub fn t_vreftimeshort(&self) -> T_VREFTIMESHORT_R {
        T_VREFTIMESHORT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - VREF changing time for multiple steps >= RU(tVREFca_Long/tfs_period)"]
    #[inline(always)]
    pub fn t_vreftimelong(&self) -> T_VREFTIMELONG_R {
        T_VREFTIMELONG_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn t_odtup(&mut self) -> T_ODTUP_W<0> {
        T_ODTUP_W::new(self)
    }
    #[doc = "Bits 8:15 - VREF changing time for single steps. >= RU(tVREFca_Short/tfs_period)"]
    #[inline(always)]
    #[must_use]
    pub fn t_vreftimeshort(&mut self) -> T_VREFTIMESHORT_W<8> {
        T_VREFTIMESHORT_W::new(self)
    }
    #[doc = "Bits 16:24 - VREF changing time for multiple steps >= RU(tVREFca_Long/tfs_period)"]
    #[inline(always)]
    #[must_use]
    pub fn t_vreftimelong(&mut self) -> T_VREFTIMELONG_W<16> {
        T_VREFTIMELONG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg14](index.html) module"]
pub struct TREG14_SPEC;
impl crate::RegisterSpec for TREG14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg14::R](R) reader structure"]
impl crate::Readable for TREG14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg14::W](W) writer structure"]
impl crate::Writable for TREG14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG14 to value 0x00c8_5020"]
impl crate::Resettable for TREG14_SPEC {
    const RESET_VALUE: Self::Ux = 0x00c8_5020;
}
