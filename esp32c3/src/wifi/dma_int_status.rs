#[doc = "Register `DMA_INT_STATUS` reader"]
pub type R = crate::R<DMA_INT_STATUS_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_STATUS_SPEC;
impl crate::RegisterSpec for DMA_INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_status::R`](R) reader structure"]
impl crate::Readable for DMA_INT_STATUS_SPEC {}
#[doc = "`reset()` method sets DMA_INT_STATUS to value 0"]
impl crate::Resettable for DMA_INT_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
