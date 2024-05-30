#[doc = "Register `DMA_INT_CLEAR` reader"]
pub type R = crate::R<DMA_INT_CLEAR_SPEC>;
#[doc = "Register `DMA_INT_CLEAR` writer"]
pub type W = crate::W<DMA_INT_CLEAR_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_CLEAR_SPEC;
impl crate::RegisterSpec for DMA_INT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_clear::R`](R) reader structure"]
impl crate::Readable for DMA_INT_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_int_clear::W`](W) writer structure"]
impl crate::Writable for DMA_INT_CLEAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_INT_CLEAR to value 0"]
impl crate::Resettable for DMA_INT_CLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
