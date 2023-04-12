#[doc = "Register `RXF0C` reader"]
pub struct R(crate::R<RXF0C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF0C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF0C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF0C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXF0C` writer"]
pub struct W(crate::W<RXF0C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXF0C_SPEC>;
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
impl From<crate::W<RXF0C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXF0C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0SA` reader - Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address, see Figure 2)."]
pub type F0SA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `F0SA` writer - Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address, see Figure 2)."]
pub type F0SA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXF0C_SPEC, u16, u16, 14, O>;
#[doc = "Field `F0S` reader - Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements 64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1"]
pub type F0S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F0S` writer - Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements 64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1"]
pub type F0S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXF0C_SPEC, u8, u8, 7, O>;
#[doc = "Field `F0WM` reader - Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) 64= Watermark interrupt disabled"]
pub type F0WM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F0WM` writer - Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) 64= Watermark interrupt disabled"]
pub type F0WM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXF0C_SPEC, u8, u8, 7, O>;
#[doc = "Field `F0OM` reader - FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode"]
pub type F0OM_R = crate::BitReader<bool>;
#[doc = "Field `F0OM` writer - FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode"]
pub type F0OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXF0C_SPEC, bool, O>;
impl R {
    #[doc = "Bits 2:15 - Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn f0sa(&self) -> F0SA_R {
        F0SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements 64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1"]
    #[inline(always)]
    pub fn f0s(&self) -> F0S_R {
        F0S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) 64= Watermark interrupt disabled"]
    #[inline(always)]
    pub fn f0wm(&self) -> F0WM_R {
        F0WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode"]
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    #[must_use]
    pub fn f0sa(&mut self) -> F0SA_W<2> {
        F0SA_W::new(self)
    }
    #[doc = "Bits 16:22 - Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements 64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1"]
    #[inline(always)]
    #[must_use]
    pub fn f0s(&mut self) -> F0S_W<16> {
        F0S_W::new(self)
    }
    #[doc = "Bits 24:30 - Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) 64= Watermark interrupt disabled"]
    #[inline(always)]
    #[must_use]
    pub fn f0wm(&mut self) -> F0WM_W<24> {
        F0WM_W::new(self)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode"]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0OM_W<31> {
        F0OM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx FIFO 0 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0c](index.html) module"]
pub struct RXF0C_SPEC;
impl crate::RegisterSpec for RXF0C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf0c::R](R) reader structure"]
impl crate::Readable for RXF0C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxf0c::W](W) writer structure"]
impl crate::Writable for RXF0C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXF0C to value 0"]
impl crate::Resettable for RXF0C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
