#[doc = "Register `MAPBIT19_16` reader"]
pub struct R(crate::R<MAPBIT19_16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT19_16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT19_16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT19_16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT19_16` writer"]
pub struct W(crate::W<MAPBIT19_16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT19_16_SPEC>;
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
impl From<crate::W<MAPBIT19_16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT19_16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT16` reader - map bit\\[ 16 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit16 in \\[ 23..0 \\]
=> bit\\[ 16 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit16 \\]; if MapBit16 in \\[29 .. 24\\]
=> bit\\[ 16 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit16 = 31 => bit\\[ 16 \\]
= 0 ; if MapBit16= 30 => bit\\[ 16 \\]
= 1"]
pub type MAPBIT16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT16` writer - map bit\\[ 16 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit16 in \\[ 23..0 \\]
=> bit\\[ 16 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit16 \\]; if MapBit16 in \\[29 .. 24\\]
=> bit\\[ 16 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit16 = 31 => bit\\[ 16 \\]
= 0 ; if MapBit16= 30 => bit\\[ 16 \\]
= 1"]
pub type MAPBIT16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT19_16_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT17` reader - map bit\\[ 17 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit17 in \\[ 23..0 \\]
=> bit\\[ 17 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit17 \\]; if MapBit17 in \\[29 .. 24\\]
=> bit\\[ 17 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit17 = 31 => bit\\[ 17 \\]
= 0 ; if MapBit17= 30 => bit\\[ 17 \\]
= 1"]
pub type MAPBIT17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT17` writer - map bit\\[ 17 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit17 in \\[ 23..0 \\]
=> bit\\[ 17 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit17 \\]; if MapBit17 in \\[29 .. 24\\]
=> bit\\[ 17 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit17 = 31 => bit\\[ 17 \\]
= 0 ; if MapBit17= 30 => bit\\[ 17 \\]
= 1"]
pub type MAPBIT17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT19_16_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT18` reader - map bit\\[ 18 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit18 in \\[ 23..0 \\]
=> bit\\[ 18 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit18 \\]; if MapBit18 in \\[29 .. 24\\]
=> bit\\[ 18 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit18 = 31 => bit\\[ 18 \\]
= 0 ; if MapBit18= 30 => bit\\[ 18 \\]
= 1"]
pub type MAPBIT18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT18` writer - map bit\\[ 18 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit18 in \\[ 23..0 \\]
=> bit\\[ 18 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit18 \\]; if MapBit18 in \\[29 .. 24\\]
=> bit\\[ 18 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit18 = 31 => bit\\[ 18 \\]
= 0 ; if MapBit18= 30 => bit\\[ 18 \\]
= 1"]
pub type MAPBIT18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT19_16_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT19` reader - map bit\\[ 19 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit19 in \\[ 23..0 \\]
=> bit\\[ 19 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit19 \\]; if MapBit19 in \\[29 .. 24\\]
=> bit\\[ 19 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit19 = 31 => bit\\[ 19 \\]
= 0 ; if MapBit19= 30 => bit\\[ 19 \\]
= 1"]
pub type MAPBIT19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT19` writer - map bit\\[ 19 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit19 in \\[ 23..0 \\]
=> bit\\[ 19 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit19 \\]; if MapBit19 in \\[29 .. 24\\]
=> bit\\[ 19 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit19 = 31 => bit\\[ 19 \\]
= 0 ; if MapBit19= 30 => bit\\[ 19 \\]
= 1"]
pub type MAPBIT19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT19_16_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - map bit\\[ 16 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit16 in \\[ 23..0 \\]
=> bit\\[ 16 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit16 \\]; if MapBit16 in \\[29 .. 24\\]
=> bit\\[ 16 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit16 = 31 => bit\\[ 16 \\]
= 0 ; if MapBit16= 30 => bit\\[ 16 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit16(&self) -> MAPBIT16_R {
        MAPBIT16_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - map bit\\[ 17 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit17 in \\[ 23..0 \\]
=> bit\\[ 17 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit17 \\]; if MapBit17 in \\[29 .. 24\\]
=> bit\\[ 17 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit17 = 31 => bit\\[ 17 \\]
= 0 ; if MapBit17= 30 => bit\\[ 17 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit17(&self) -> MAPBIT17_R {
        MAPBIT17_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - map bit\\[ 18 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit18 in \\[ 23..0 \\]
=> bit\\[ 18 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit18 \\]; if MapBit18 in \\[29 .. 24\\]
=> bit\\[ 18 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit18 = 31 => bit\\[ 18 \\]
= 0 ; if MapBit18= 30 => bit\\[ 18 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit18(&self) -> MAPBIT18_R {
        MAPBIT18_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - map bit\\[ 19 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit19 in \\[ 23..0 \\]
=> bit\\[ 19 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit19 \\]; if MapBit19 in \\[29 .. 24\\]
=> bit\\[ 19 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit19 = 31 => bit\\[ 19 \\]
= 0 ; if MapBit19= 30 => bit\\[ 19 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit19(&self) -> MAPBIT19_R {
        MAPBIT19_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - map bit\\[ 16 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit16 in \\[ 23..0 \\]
=> bit\\[ 16 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit16 \\]; if MapBit16 in \\[29 .. 24\\]
=> bit\\[ 16 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit16 = 31 => bit\\[ 16 \\]
= 0 ; if MapBit16= 30 => bit\\[ 16 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit16(&mut self) -> MAPBIT16_W<0> {
        MAPBIT16_W::new(self)
    }
    #[doc = "Bits 8:12 - map bit\\[ 17 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit17 in \\[ 23..0 \\]
=> bit\\[ 17 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit17 \\]; if MapBit17 in \\[29 .. 24\\]
=> bit\\[ 17 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit17 = 31 => bit\\[ 17 \\]
= 0 ; if MapBit17= 30 => bit\\[ 17 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit17(&mut self) -> MAPBIT17_W<8> {
        MAPBIT17_W::new(self)
    }
    #[doc = "Bits 16:20 - map bit\\[ 18 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit18 in \\[ 23..0 \\]
=> bit\\[ 18 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit18 \\]; if MapBit18 in \\[29 .. 24\\]
=> bit\\[ 18 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit18 = 31 => bit\\[ 18 \\]
= 0 ; if MapBit18= 30 => bit\\[ 18 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit18(&mut self) -> MAPBIT18_W<16> {
        MAPBIT18_W::new(self)
    }
    #[doc = "Bits 24:28 - map bit\\[ 19 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit19 in \\[ 23..0 \\]
=> bit\\[ 19 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit19 \\]; if MapBit19 in \\[29 .. 24\\]
=> bit\\[ 19 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
} ; If MapBit19 = 31 => bit\\[ 19 \\]
= 0 ; if MapBit19= 30 => bit\\[ 19 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit19(&mut self) -> MAPBIT19_W<24> {
        MAPBIT19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 16 .. 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit19_16](index.html) module"]
pub struct MAPBIT19_16_SPEC;
impl crate::RegisterSpec for MAPBIT19_16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit19_16::R](R) reader structure"]
impl crate::Readable for MAPBIT19_16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit19_16::W](W) writer structure"]
impl crate::Writable for MAPBIT19_16_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT19_16 to value 0x1312_1110"]
impl crate::Resettable for MAPBIT19_16_SPEC {
    const RESET_VALUE: Self::Ux = 0x1312_1110;
}
