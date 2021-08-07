// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! From arch/x86/include/asm/pgtable-3level_types.h

pub type pteval_t = u64;
pub type pmdval_t = u64;
pub type pudval_t = u64;
pub type p4dval_t = u64;
pub type pgdval_t = u64;
pub type pgprotval_t = u64;

#[repr(C)]
pub union pte_t {
    pub pte_range: struct {
        pub pte_low: usize,
        pub pte_high: usize,
    },
    pub pte: pteval_t,
}

//#define ARCH_PAGE_TABLE_SYNC_MASK	(SHARED_KERNEL_PMD ? 0 : PGTBL_PMD_MODIFIED)

/// PGDIR_SHIFT determines what a top-level page table entry can map
pub const PGDIR_SHIFT: i32 = 30;
pub const PTRS_PER_PGD: i32 = 4;

/// PMD_SHIFT determines the size of the area a middle-level page table can map
pub const PMD_SHIFT: i32 = 21;
pub const PTRS_PER_PMD: i32 = 512;

/// entries per page directory level
pub const PTRS_PER_PTE: i32 = 512;

pub const MAX_POSSIBLE_PHYSMEM_BITS: i32 = 36;
pub const PGD_KERNEL_START: usize = CONFIG_PAGE_OFFSET >> PGDIR_SHIFT;