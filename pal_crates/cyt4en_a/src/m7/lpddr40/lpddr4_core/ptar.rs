#[doc = "Register `PTAR` reader"]
pub struct R(crate::R<PTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTAR` writer"]
pub struct W(crate::W<PTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTAR_SPEC>;
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
impl From<crate::W<PTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BA` reader - Test bank address to be used in Read-Write Sanity Check feature to test Read/Write functions after trainings. This feature is enabled by setting sanchken and phyinit field in POM register."]
pub type BA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BA` writer - Test bank address to be used in Read-Write Sanity Check feature to test Read/Write functions after trainings. This feature is enabled by setting sanchken and phyinit field in POM register."]
pub type BA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTAR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ROW` reader - Test row address to be used in Read-Write Sanity Check feature to test Read/Write functions after trainings. This feature is enabled by setting sanchken and phyinit field in POM register."]
pub type ROW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ROW` writer - Test row address to be used in Read-Write Sanity Check feature to test Read/Write functions after trainings. This feature is enabled by setting sanchken and phyinit field in POM register."]
pub type ROW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTAR_SPEC, u16, u16, 15, O>;
#[doc = "Field `COL` reader - Test column address to be used in Read-Write Sanity Check feature to test Read/Write functions after trainings. This feature is enabled by setting sanchken and phyinit field in POM register."]
pub type COL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COL` writer - Test column address to be used in Read-Write Sanity Check feature to test Read/Write functions after trainings. This feature is enabled by setting sanchken and phyinit field in POM register."]
pub type COL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTAR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:2 - Test bank address to be used in Read-Write Sanity Check feature to test Read/Write functions after trainings. This feature is enabled by setting sanchken and phyinit field in POM register."]
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:17 - Test row address to be used in Read-Write Sanity Check feature to test Read/Write functions after trainings. This feature is enabled by setting sanchken and phyinit field in POM register."]
    #[inline(always)]
    pub fn row(&self) -> ROW_R {
        ROW_R::new(((self.bits >> 3) & 0x7fff) as u16)
    }
    #[doc = "Bits 18:27 - Test column address to be used in Read-Write Sanity Check feature to test Read/Write functions after trainings. This feature is enabled by setting sanchken and phyinit field in POM register."]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 18) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Test bank address to be used in Read-Write Sanity Check feature to test Read/Write functions after trainings. This feature is enabled by setting sanchken and phyinit field in POM register."]
    #[inline(always)]
    #[must_use]
    pub fn ba(&mut self) -> BA_W<0> {
        BA_W::new(self)
    }
    #[doc = "Bits 3:17 - Test row address to be used in Read-Write Sanity Check feature to test Read/Write functions after trainings. This feature is enabled by setting sanchken and phyinit field in POM register."]
    #[inline(always)]
    #[must_use]
    pub fn row(&mut self) -> ROW_W<3> {
        ROW_W::new(self)
    }
    #[doc = "Bits 18:27 - Test column address to be used in Read-Write Sanity Check feature to test Read/Write functions after trainings. This feature is enabled by setting sanchken and phyinit field in POM register."]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> COL_W<18> {
        COL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Sanity Check Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptar](index.html) module"]
pub struct PTAR_SPEC;
impl crate::RegisterSpec for PTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptar::R](R) reader structure"]
impl crate::Readable for PTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptar::W](W) writer structure"]
impl crate::Writable for PTAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTAR to value 0"]
impl crate::Resettable for PTAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
