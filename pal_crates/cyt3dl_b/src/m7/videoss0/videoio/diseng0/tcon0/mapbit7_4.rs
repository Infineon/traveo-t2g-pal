#[doc = "Register `MAPBIT7_4` reader"]
pub struct R(crate::R<MAPBIT7_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT7_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT7_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT7_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT7_4` writer"]
pub struct W(crate::W<MAPBIT7_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT7_4_SPEC>;
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
impl From<crate::W<MAPBIT7_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT7_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT4` reader - map bit\\[ 4 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit4 in \\[ 23..0 \\]
=> bit\\[ 4 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit4 \\]; if MapBit4 in \\[29 .. 24\\]
=> bit\\[ 4 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit4 = 31 => bit\\[ 4 \\]
= 0 ; if MapBit4= 30 => bit\\[ 4 \\]
= 1"]
pub type MAPBIT4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT4` writer - map bit\\[ 4 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit4 in \\[ 23..0 \\]
=> bit\\[ 4 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit4 \\]; if MapBit4 in \\[29 .. 24\\]
=> bit\\[ 4 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit4 = 31 => bit\\[ 4 \\]
= 0 ; if MapBit4= 30 => bit\\[ 4 \\]
= 1"]
pub type MAPBIT4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT7_4_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT5` reader - map bit\\[ 5 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit5 in \\[ 23..0 \\]
=> bit\\[ 5 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit5 \\]; if MapBit5 in \\[29 .. 24\\]
=> bit\\[ 5 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit5 = 31 => bit\\[ 5 \\]
= 0 ; if MapBit5= 30 => bit\\[ 5 \\]
= 1"]
pub type MAPBIT5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT5` writer - map bit\\[ 5 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit5 in \\[ 23..0 \\]
=> bit\\[ 5 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit5 \\]; if MapBit5 in \\[29 .. 24\\]
=> bit\\[ 5 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit5 = 31 => bit\\[ 5 \\]
= 0 ; if MapBit5= 30 => bit\\[ 5 \\]
= 1"]
pub type MAPBIT5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT7_4_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT6` reader - map bit\\[ 6 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit6 in \\[ 23..0 \\]
=> bit\\[ 6 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit6 \\]; if MapBit6 in \\[29 .. 24\\]
=> bit\\[ 6 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit6 = 31 => bit\\[ 6 \\]
= 0 ; if MapBit6= 30 => bit\\[ 6 \\]
= 1"]
pub type MAPBIT6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT6` writer - map bit\\[ 6 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit6 in \\[ 23..0 \\]
=> bit\\[ 6 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit6 \\]; if MapBit6 in \\[29 .. 24\\]
=> bit\\[ 6 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit6 = 31 => bit\\[ 6 \\]
= 0 ; if MapBit6= 30 => bit\\[ 6 \\]
= 1"]
pub type MAPBIT6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT7_4_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT7` reader - map bit\\[ 7 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit7 in \\[ 23..0 \\]
=> bit\\[ 7 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit7 \\]; if MapBit7 in \\[29 .. 24\\]
=> bit\\[ 7 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit7 = 31 => bit\\[ 7 \\]
= 0 ; if MapBit7= 30 => bit\\[ 7 \\]
= 1"]
pub type MAPBIT7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT7` writer - map bit\\[ 7 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit7 in \\[ 23..0 \\]
=> bit\\[ 7 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit7 \\]; if MapBit7 in \\[29 .. 24\\]
=> bit\\[ 7 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit7 = 31 => bit\\[ 7 \\]
= 0 ; if MapBit7= 30 => bit\\[ 7 \\]
= 1"]
pub type MAPBIT7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT7_4_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - map bit\\[ 4 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit4 in \\[ 23..0 \\]
=> bit\\[ 4 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit4 \\]; if MapBit4 in \\[29 .. 24\\]
=> bit\\[ 4 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit4 = 31 => bit\\[ 4 \\]
= 0 ; if MapBit4= 30 => bit\\[ 4 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit4(&self) -> MAPBIT4_R {
        MAPBIT4_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - map bit\\[ 5 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit5 in \\[ 23..0 \\]
=> bit\\[ 5 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit5 \\]; if MapBit5 in \\[29 .. 24\\]
=> bit\\[ 5 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit5 = 31 => bit\\[ 5 \\]
= 0 ; if MapBit5= 30 => bit\\[ 5 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit5(&self) -> MAPBIT5_R {
        MAPBIT5_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - map bit\\[ 6 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit6 in \\[ 23..0 \\]
=> bit\\[ 6 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit6 \\]; if MapBit6 in \\[29 .. 24\\]
=> bit\\[ 6 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit6 = 31 => bit\\[ 6 \\]
= 0 ; if MapBit6= 30 => bit\\[ 6 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit6(&self) -> MAPBIT6_R {
        MAPBIT6_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - map bit\\[ 7 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit7 in \\[ 23..0 \\]
=> bit\\[ 7 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit7 \\]; if MapBit7 in \\[29 .. 24\\]
=> bit\\[ 7 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit7 = 31 => bit\\[ 7 \\]
= 0 ; if MapBit7= 30 => bit\\[ 7 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit7(&self) -> MAPBIT7_R {
        MAPBIT7_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - map bit\\[ 4 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit4 in \\[ 23..0 \\]
=> bit\\[ 4 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit4 \\]; if MapBit4 in \\[29 .. 24\\]
=> bit\\[ 4 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit4 = 31 => bit\\[ 4 \\]
= 0 ; if MapBit4= 30 => bit\\[ 4 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit4(&mut self) -> MAPBIT4_W<0> {
        MAPBIT4_W::new(self)
    }
    #[doc = "Bits 8:12 - map bit\\[ 5 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit5 in \\[ 23..0 \\]
=> bit\\[ 5 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit5 \\]; if MapBit5 in \\[29 .. 24\\]
=> bit\\[ 5 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit5 = 31 => bit\\[ 5 \\]
= 0 ; if MapBit5= 30 => bit\\[ 5 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit5(&mut self) -> MAPBIT5_W<8> {
        MAPBIT5_W::new(self)
    }
    #[doc = "Bits 16:20 - map bit\\[ 6 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit6 in \\[ 23..0 \\]
=> bit\\[ 6 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit6 \\]; if MapBit6 in \\[29 .. 24\\]
=> bit\\[ 6 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit6 = 31 => bit\\[ 6 \\]
= 0 ; if MapBit6= 30 => bit\\[ 6 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit6(&mut self) -> MAPBIT6_W<16> {
        MAPBIT6_W::new(self)
    }
    #[doc = "Bits 24:28 - map bit\\[ 7 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit7 in \\[ 23..0 \\]
=> bit\\[ 7 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit7 \\]; if MapBit7 in \\[29 .. 24\\]
=> bit\\[ 7 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit7 = 31 => bit\\[ 7 \\]
= 0 ; if MapBit7= 30 => bit\\[ 7 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit7(&mut self) -> MAPBIT7_W<24> {
        MAPBIT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 4 .. 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit7_4](index.html) module"]
pub struct MAPBIT7_4_SPEC;
impl crate::RegisterSpec for MAPBIT7_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit7_4::R](R) reader structure"]
impl crate::Readable for MAPBIT7_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit7_4::W](W) writer structure"]
impl crate::Writable for MAPBIT7_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT7_4 to value 0x0706_0504"]
impl crate::Resettable for MAPBIT7_4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0706_0504;
}
