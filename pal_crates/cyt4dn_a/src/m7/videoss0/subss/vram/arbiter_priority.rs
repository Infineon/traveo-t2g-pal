#[doc = "Register `ARBITER_PRIORITY` reader"]
pub struct R(crate::R<ARBITER_PRIORITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARBITER_PRIORITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARBITER_PRIORITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARBITER_PRIORITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARBITER_PRIORITY` writer"]
pub struct W(crate::W<ARBITER_PRIORITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARBITER_PRIORITY_SPEC>;
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
impl From<crate::W<ARBITER_PRIORITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARBITER_PRIORITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIORITY_WR0` reader - Priority for write slave interfaces 0."]
pub type PRIORITY_WR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY_WR0` writer - Priority for write slave interfaces 0."]
pub type PRIORITY_WR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ARBITER_PRIORITY_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRIORITY_RD0` reader - Priority for read slave interface 0."]
pub type PRIORITY_RD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY_RD0` writer - Priority for read slave interface 0."]
pub type PRIORITY_RD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ARBITER_PRIORITY_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRIORITY_RD1` reader - Priority for read slave interface 1."]
pub type PRIORITY_RD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY_RD1` writer - Priority for read slave interface 1."]
pub type PRIORITY_RD1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ARBITER_PRIORITY_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRIORITY_RD2` reader - Priority for read slave interface 2."]
pub type PRIORITY_RD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY_RD2` writer - Priority for read slave interface 2."]
pub type PRIORITY_RD2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ARBITER_PRIORITY_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRIORITY_RD3` reader - Priority for read slave interface 3."]
pub type PRIORITY_RD3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY_RD3` writer - Priority for read slave interface 3."]
pub type PRIORITY_RD3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ARBITER_PRIORITY_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRIORITY_RD4` reader - Priority for read slave interface 4."]
pub type PRIORITY_RD4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY_RD4` writer - Priority for read slave interface 4."]
pub type PRIORITY_RD4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ARBITER_PRIORITY_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Priority for write slave interfaces 0."]
    #[inline(always)]
    pub fn priority_wr0(&self) -> PRIORITY_WR0_R {
        PRIORITY_WR0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Priority for read slave interface 0."]
    #[inline(always)]
    pub fn priority_rd0(&self) -> PRIORITY_RD0_R {
        PRIORITY_RD0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Priority for read slave interface 1."]
    #[inline(always)]
    pub fn priority_rd1(&self) -> PRIORITY_RD1_R {
        PRIORITY_RD1_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Priority for read slave interface 2."]
    #[inline(always)]
    pub fn priority_rd2(&self) -> PRIORITY_RD2_R {
        PRIORITY_RD2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Priority for read slave interface 3."]
    #[inline(always)]
    pub fn priority_rd3(&self) -> PRIORITY_RD3_R {
        PRIORITY_RD3_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Priority for read slave interface 4."]
    #[inline(always)]
    pub fn priority_rd4(&self) -> PRIORITY_RD4_R {
        PRIORITY_RD4_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Priority for write slave interfaces 0."]
    #[inline(always)]
    #[must_use]
    pub fn priority_wr0(&mut self) -> PRIORITY_WR0_W<0> {
        PRIORITY_WR0_W::new(self)
    }
    #[doc = "Bits 8:9 - Priority for read slave interface 0."]
    #[inline(always)]
    #[must_use]
    pub fn priority_rd0(&mut self) -> PRIORITY_RD0_W<8> {
        PRIORITY_RD0_W::new(self)
    }
    #[doc = "Bits 12:13 - Priority for read slave interface 1."]
    #[inline(always)]
    #[must_use]
    pub fn priority_rd1(&mut self) -> PRIORITY_RD1_W<12> {
        PRIORITY_RD1_W::new(self)
    }
    #[doc = "Bits 16:17 - Priority for read slave interface 2."]
    #[inline(always)]
    #[must_use]
    pub fn priority_rd2(&mut self) -> PRIORITY_RD2_W<16> {
        PRIORITY_RD2_W::new(self)
    }
    #[doc = "Bits 20:21 - Priority for read slave interface 3."]
    #[inline(always)]
    #[must_use]
    pub fn priority_rd3(&mut self) -> PRIORITY_RD3_W<20> {
        PRIORITY_RD3_W::new(self)
    }
    #[doc = "Bits 24:25 - Priority for read slave interface 4."]
    #[inline(always)]
    #[must_use]
    pub fn priority_rd4(&mut self) -> PRIORITY_RD4_W<24> {
        PRIORITY_RD4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "N/A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arbiter_priority](index.html) module"]
pub struct ARBITER_PRIORITY_SPEC;
impl crate::RegisterSpec for ARBITER_PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arbiter_priority::R](R) reader structure"]
impl crate::Readable for ARBITER_PRIORITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arbiter_priority::W](W) writer structure"]
impl crate::Writable for ARBITER_PRIORITY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARBITER_PRIORITY to value 0"]
impl crate::Resettable for ARBITER_PRIORITY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
