#[doc = "Register `TX_INST_CFG1` reader"]
pub type R = crate::R<TX_INST_CFG1_SPEC>;
#[doc = "Register `TX_INST_CFG1` writer"]
pub type W = crate::W<TX_INST_CFG1_SPEC>;
#[doc = "Field `TX_INST` reader - write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG"]
pub type TX_INST_R = crate::FieldReader<u32>;
#[doc = "Field `TX_INST` writer - write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG"]
pub type TX_INST_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG"]
    #[inline(always)]
    pub fn tx_inst(&self) -> TX_INST_R {
        TX_INST_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_INST_CFG1")
            .field("tx_inst", &format_args!("{}", self.tx_inst().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_INST_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG"]
    #[inline(always)]
    #[must_use]
    pub fn tx_inst(&mut self) -> TX_INST_W<TX_INST_CFG1_SPEC> {
        TX_INST_W::new(self, 0)
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
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_inst_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_inst_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_INST_CFG1_SPEC;
impl crate::RegisterSpec for TX_INST_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_inst_cfg1::R`](R) reader structure"]
impl crate::Readable for TX_INST_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_inst_cfg1::W`](W) writer structure"]
impl crate::Writable for TX_INST_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_INST_CFG1 to value 0x04"]
impl crate::Resettable for TX_INST_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}