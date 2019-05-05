// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

pub mod gic;
pub mod layout;
pub mod regs;

use std::cmp::min;

use memory_model::{GuestAddress, GuestMemory};

/// Returns a Vec of the valid memory addresses for aarch64.
/// See [`layout`](layout) module for a drawing of the specific memory model for this platform.
pub fn arch_memory_regions(size: usize) -> Vec<(GuestAddress, usize)> {
    let dram_size = min(size, layout::DRAM_MEM_END);
    vec![(GuestAddress(layout::DRAM_MEM_START), dram_size)]
}

/// Stub function that needs to be implemented when aarch64 functionality is added.
pub fn configure_system(
    _guest_mem: &GuestMemory,
    _cmdline_addr: GuestAddress,
    _cmdline_size: usize,
    _num_cpus: u8,
) -> super::Result<()> {
    Ok(())
}

/// Stub function that needs to be implemented when aarch64 functionality is added.
pub fn get_reserved_mem_addr() -> usize {
    0
}

// Auxiliary function to get the address where the device tree blob is loaded.
fn get_fdt_addr(mem: &GuestMemory) -> usize {
    // If the memory allocated is smaller than the size allocated for the FDT,
    // we return the start of the DRAM so that
    // we allow the code to try and load the FDT.

    if let Some(offset) = mem.end_addr().checked_sub(layout::FDT_MAX_SIZE) {
        if mem.address_in_range(offset) {
            return offset.offset();
        }
    }
    layout::DRAM_MEM_START
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regions_lt_1024gb() {
        let regions = arch_memory_regions(1usize << 29);
        assert_eq!(1, regions.len());
        assert_eq!(GuestAddress(super::layout::DRAM_MEM_START), regions[0].0);
        assert_eq!(1usize << 29, regions[0].1);
    }

    #[test]
    fn test_regions_gt_1024gb() {
        let regions = arch_memory_regions(1usize << 41);
        assert_eq!(1, regions.len());
        assert_eq!(GuestAddress(super::layout::DRAM_MEM_START), regions[0].0);
        assert_eq!(super::layout::DRAM_MEM_END, regions[0].1);
    }

    #[test]
    fn test_get_fdt_addr() {
        let regions = arch_memory_regions(layout::FDT_MAX_SIZE - 0x1000);
        let mem = GuestMemory::new(&regions).expect("Cannot initialize memory");
        assert_eq!(get_fdt_addr(&mem), layout::DRAM_MEM_START);

        let regions = arch_memory_regions(layout::FDT_MAX_SIZE);
        let mem = GuestMemory::new(&regions).expect("Cannot initialize memory");
        assert_eq!(get_fdt_addr(&mem), layout::DRAM_MEM_START);

        let regions = arch_memory_regions(layout::FDT_MAX_SIZE + 0x1000);
        let mem = GuestMemory::new(&regions).expect("Cannot initialize memory");
        assert_eq!(get_fdt_addr(&mem), 0x1000 + layout::DRAM_MEM_START);
    }
}
