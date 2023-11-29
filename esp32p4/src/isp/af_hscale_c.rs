#[doc = "Register `AF_HSCALE_C` reader"]
pub type R = crate::R<AF_HSCALE_C_SPEC>;
#[doc = "Register `AF_HSCALE_C` writer"]
pub type W = crate::W<AF_HSCALE_C_SPEC>;
#[doc = "Field `AF_RPOINT_C` reader - this field configures left coordinate of focus window c, must >= 2"]
pub type AF_RPOINT_C_R = crate::FieldReader<u16>;
#[doc = "Field `AF_RPOINT_C` writer - this field configures left coordinate of focus window c, must >= 2"]
pub type AF_RPOINT_C_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AF_LPOINT_C` reader - this field configures top coordinate of focus window c, must >= 2"]
pub type AF_LPOINT_C_R = crate::FieldReader<u16>;
#[doc = "Field `AF_LPOINT_C` writer - this field configures top coordinate of focus window c, must >= 2"]
pub type AF_LPOINT_C_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures left coordinate of focus window c, must >= 2"]
    #[inline(always)]
    pub fn af_rpoint_c(&self) -> AF_RPOINT_C_R {
        AF_RPOINT_C_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures top coordinate of focus window c, must >= 2"]
    #[inline(always)]
    pub fn af_lpoint_c(&self) -> AF_LPOINT_C_R {
        AF_LPOINT_C_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_HSCALE_C")
            .field(
                "af_rpoint_c",
                &format_args!("{}", self.af_rpoint_c().bits()),
            )
            .field(
                "af_lpoint_c",
                &format_args!("{}", self.af_lpoint_c().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AF_HSCALE_C_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures left coordinate of focus window c, must >= 2"]
    #[inline(always)]
    #[must_use]
    pub fn af_rpoint_c(&mut self) -> AF_RPOINT_C_W<AF_HSCALE_C_SPEC> {
        AF_RPOINT_C_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures top coordinate of focus window c, must >= 2"]
    #[inline(always)]
    #[must_use]
    pub fn af_lpoint_c(&mut self) -> AF_LPOINT_C_W<AF_HSCALE_C_SPEC> {
        AF_LPOINT_C_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "v-scale of af window c register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af_hscale_c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_hscale_c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_HSCALE_C_SPEC;
impl crate::RegisterSpec for AF_HSCALE_C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_hscale_c::R`](R) reader structure"]
impl crate::Readable for AF_HSCALE_C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af_hscale_c::W`](W) writer structure"]
impl crate::Writable for AF_HSCALE_C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AF_HSCALE_C to value 0x0001_0080"]
impl crate::Resettable for AF_HSCALE_C_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0080;
}