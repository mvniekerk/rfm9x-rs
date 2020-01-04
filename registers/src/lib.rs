#![doc = "Peripheral access API for RFM9X microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "C" {
    fn ADC0_INTREQ_0();
    fn ADC0_INTREQ_1();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "LoRa (TM) mode"]
pub struct LORA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LORA {}
impl LORA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lora::RegisterBlock {
        0 as *const _
    }
}
impl Deref for LORA {
    type Target = lora::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LORA::ptr() }
    }
}
#[doc = "LoRa (TM) mode"]
pub mod lora;
#[doc = "Device is put in FSM/OSK mode"]
pub struct FSM_OSK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FSM_OSK {}
impl FSM_OSK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lora::RegisterBlock {
        0 as *const _
    }
}
impl Deref for FSM_OSK {
    type Target = lora::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FSM_OSK::ptr() }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "LORA"]
    pub LORA: LORA,
    #[doc = "FSM_OSK"]
    pub FSM_OSK: FSM_OSK,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
//        cortex_m::interrupt::free(|_| {
//            if unsafe { DEVICE_PERIPHERALS } {
//                None
//            } else {
//                Some(unsafe { Peripherals::steal() })
//            }
//        })
        Some(unsafe { Peripherals::steal() })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            LORA: LORA {
                _marker: PhantomData,
            },
            FSM_OSK: FSM_OSK {
                _marker: PhantomData,
            },
        }
    }
}
