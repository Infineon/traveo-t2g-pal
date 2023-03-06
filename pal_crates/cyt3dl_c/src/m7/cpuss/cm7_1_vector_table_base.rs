#[doc = "Register `CM7_1_VECTOR_TABLE_BASE` reader"]
pub struct R(crate::R<CM7_1_VECTOR_TABLE_BASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM7_1_VECTOR_TABLE_BASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM7_1_VECTOR_TABLE_BASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM7_1_VECTOR_TABLE_BASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM7_1_VECTOR_TABLE_BASE` writer"]
pub struct W(crate::W<CM7_1_VECTOR_TABLE_BASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM7_1_VECTOR_TABLE_BASE_SPEC>;
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
impl From<crate::W<CM7_1_VECTOR_TABLE_BASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM7_1_VECTOR_TABLE_BASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR25` reader - The default CM7 vector table base address is 0x0100:0000 (CM7 VTOR and reset exception handler address after reset). This is the location of the system ROM memory. The system ROM is mirrored at 0x0000:0000. This is required for the CM0+ boot process (the CM0+ VTOR and reset exception handler address is at fixed address 0x0000:00000 after reset). The system ROM requires mirroring as the CM7 CPUs' ITCMs are located at base address 0x0000:0000 and effectively cut a hole in the system ROM mirror address space (but the 0x0100:0000 address space is not affected)."]
pub type ADDR25_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR25` writer - The default CM7 vector table base address is 0x0100:0000 (CM7 VTOR and reset exception handler address after reset). This is the location of the system ROM memory. The system ROM is mirrored at 0x0000:0000. This is required for the CM0+ boot process (the CM0+ VTOR and reset exception handler address is at fixed address 0x0000:00000 after reset). The system ROM requires mirroring as the CM7 CPUs' ITCMs are located at base address 0x0000:0000 and effectively cut a hole in the system ROM mirror address space (but the 0x0100:0000 address space is not affected)."]
pub type ADDR25_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CM7_1_VECTOR_TABLE_BASE_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bits 7:31 - The default CM7 vector table base address is 0x0100:0000 (CM7 VTOR and reset exception handler address after reset). This is the location of the system ROM memory. The system ROM is mirrored at 0x0000:0000. This is required for the CM0+ boot process (the CM0+ VTOR and reset exception handler address is at fixed address 0x0000:00000 after reset). The system ROM requires mirroring as the CM7 CPUs' ITCMs are located at base address 0x0000:0000 and effectively cut a hole in the system ROM mirror address space (but the 0x0100:0000 address space is not affected)."]
    #[inline(always)]
    pub fn addr25(&self) -> ADDR25_R {
        ADDR25_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 7:31 - The default CM7 vector table base address is 0x0100:0000 (CM7 VTOR and reset exception handler address after reset). This is the location of the system ROM memory. The system ROM is mirrored at 0x0000:0000. This is required for the CM0+ boot process (the CM0+ VTOR and reset exception handler address is at fixed address 0x0000:00000 after reset). The system ROM requires mirroring as the CM7 CPUs' ITCMs are located at base address 0x0000:0000 and effectively cut a hole in the system ROM mirror address space (but the 0x0100:0000 address space is not affected)."]
    #[inline(always)]
    #[must_use]
    pub fn addr25(&mut self) -> ADDR25_W<7> {
        ADDR25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM7 1 vector table base\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_1_vector_table_base](index.html) module"]
pub struct CM7_1_VECTOR_TABLE_BASE_SPEC;
impl crate::RegisterSpec for CM7_1_VECTOR_TABLE_BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm7_1_vector_table_base::R](R) reader structure"]
impl crate::Readable for CM7_1_VECTOR_TABLE_BASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm7_1_vector_table_base::W](W) writer structure"]
impl crate::Writable for CM7_1_VECTOR_TABLE_BASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM7_1_VECTOR_TABLE_BASE to value 0x0100_0000"]
impl crate::Resettable for CM7_1_VECTOR_TABLE_BASE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
