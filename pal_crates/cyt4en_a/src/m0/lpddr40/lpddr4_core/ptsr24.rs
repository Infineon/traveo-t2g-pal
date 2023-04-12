#[doc = "Register `PTSR24` reader"]
pub struct R(crate::R<PTSR24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR24` writer"]
pub struct W(crate::W<PTSR24_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR24_SPEC>;
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
impl From<crate::W<PTSR24_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR24_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDDLY_CKE0` reader - MEM_CKE\\[0\\]
delay setting"]
pub type CMDDLY_CKE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDDLY_CKE0` writer - MEM_CKE\\[0\\]
delay setting"]
pub type CMDDLY_CKE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR24_SPEC, u8, u8, 6, O>;
#[doc = "Field `CMDDLY_CKE1` reader - MEM_CKE\\[1\\]
delay setting"]
pub type CMDDLY_CKE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDDLY_CKE1` writer - MEM_CKE\\[1\\]
delay setting"]
pub type CMDDLY_CKE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR24_SPEC, u8, u8, 6, O>;
#[doc = "Field `PSCK` reader - PHY-set 1CK exceed for each slice"]
pub type PSCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSCK` writer - PHY-set 1CK exceed for each slice"]
pub type PSCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR24_SPEC, u8, u8, 4, O>;
#[doc = "Field `SANPAT` reader - Sanity check pattern. Each of 16 bits sanpat\\[15:0\\]
is used for one data transfer. Therefore, 16 bits sanpat\\[15:0\\]
correspond to one DRAM burst (LPDDR4 - BL16). The values of all bits in a data transfer are the same and equal to sanpat bit of corresponding transfer."]
pub type SANPAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SANPAT` writer - Sanity check pattern. Each of 16 bits sanpat\\[15:0\\]
is used for one data transfer. Therefore, 16 bits sanpat\\[15:0\\]
correspond to one DRAM burst (LPDDR4 - BL16). The values of all bits in a data transfer are the same and equal to sanpat bit of corresponding transfer."]
pub type SANPAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR24_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:5 - MEM_CKE\\[0\\]
delay setting"]
    #[inline(always)]
    pub fn cmddly_cke0(&self) -> CMDDLY_CKE0_R {
        CMDDLY_CKE0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - MEM_CKE\\[1\\]
delay setting"]
    #[inline(always)]
    pub fn cmddly_cke1(&self) -> CMDDLY_CKE1_R {
        CMDDLY_CKE1_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:15 - PHY-set 1CK exceed for each slice"]
    #[inline(always)]
    pub fn psck(&self) -> PSCK_R {
        PSCK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Sanity check pattern. Each of 16 bits sanpat\\[15:0\\]
is used for one data transfer. Therefore, 16 bits sanpat\\[15:0\\]
correspond to one DRAM burst (LPDDR4 - BL16). The values of all bits in a data transfer are the same and equal to sanpat bit of corresponding transfer."]
    #[inline(always)]
    pub fn sanpat(&self) -> SANPAT_R {
        SANPAT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - MEM_CKE\\[0\\]
delay setting"]
    #[inline(always)]
    #[must_use]
    pub fn cmddly_cke0(&mut self) -> CMDDLY_CKE0_W<0> {
        CMDDLY_CKE0_W::new(self)
    }
    #[doc = "Bits 6:11 - MEM_CKE\\[1\\]
delay setting"]
    #[inline(always)]
    #[must_use]
    pub fn cmddly_cke1(&mut self) -> CMDDLY_CKE1_W<6> {
        CMDDLY_CKE1_W::new(self)
    }
    #[doc = "Bits 12:15 - PHY-set 1CK exceed for each slice"]
    #[inline(always)]
    #[must_use]
    pub fn psck(&mut self) -> PSCK_W<12> {
        PSCK_W::new(self)
    }
    #[doc = "Bits 16:31 - Sanity check pattern. Each of 16 bits sanpat\\[15:0\\]
is used for one data transfer. Therefore, 16 bits sanpat\\[15:0\\]
correspond to one DRAM burst (LPDDR4 - BL16). The values of all bits in a data transfer are the same and equal to sanpat bit of corresponding transfer."]
    #[inline(always)]
    #[must_use]
    pub fn sanpat(&mut self) -> SANPAT_W<16> {
        SANPAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr24](index.html) module"]
pub struct PTSR24_SPEC;
impl crate::RegisterSpec for PTSR24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr24::R](R) reader structure"]
impl crate::Readable for PTSR24_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr24::W](W) writer structure"]
impl crate::Writable for PTSR24_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR24 to value 0"]
impl crate::Resettable for PTSR24_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
