#[doc = "Register `PORT_SELECT` reader"]
pub struct R(crate::R<PORT_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_SELECT` writer"]
pub struct W(crate::W<PORT_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_SELECT_SPEC>;
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
impl From<crate::W<PORT_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELECT` reader - This field selects the AXI port of the LPDDR4 controller on which the performance measurement is done."]
pub type SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELECT` writer - This field selects the AXI port of the LPDDR4 controller on which the performance measurement is done."]
pub type SELECT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORT_SELECT_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - This field selects the AXI port of the LPDDR4 controller on which the performance measurement is done."]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - This field selects the AXI port of the LPDDR4 controller on which the performance measurement is done."]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<0> {
        SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI port select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_select](index.html) module"]
pub struct PORT_SELECT_SPEC;
impl crate::RegisterSpec for PORT_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_select::R](R) reader structure"]
impl crate::Readable for PORT_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_select::W](W) writer structure"]
impl crate::Writable for PORT_SELECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORT_SELECT to value 0"]
impl crate::Resettable for PORT_SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
