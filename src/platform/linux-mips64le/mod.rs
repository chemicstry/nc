pub mod call;
pub mod consts;
pub mod errno;
pub mod sysno;
pub mod types;

use errno::Errno;
use sysno::Sysno;

const MAX_ERRNO: i32 = 4095;

#[inline(always)]
pub fn check_errno(ret: usize) -> Result<usize, Errno> {
    let reti = ret as isize;
    if reti < 0 && reti >= (-MAX_ERRNO) as isize {
        let reti = (-reti) as Errno;
        Err(reti)
    } else {
        Ok(ret)
    }
}

#[inline(always)]
pub unsafe fn syscall0(n: Sysno) -> Result<usize, Errno> {
    let ret: usize;
    let mut n = n;
    asm!("syscall"
         : "+&{r2}"(n), "={r7}"(ret)
         :
         : "$1", "$3", "$10", "$11", "$12", "$13", "$14", "$15", "$24", "$25", "hi", "lo", "memory"
         : "volatile");
    check_errno(n)
}

#[inline(always)]
pub unsafe fn syscall1(n: Sysno, a1: usize) -> Result<usize, Errno> {
    let ret: usize;
    let mut n = n;
    asm!("syscall"
         : "+&{r2}"(n), "={r7}"(ret)
         : "{r4}"(a1)
         : "$1", "$3", "$10", "$11", "$12", "$13", "$14", "$15", "$24", "$25", "hi", "lo", "memory"
         : "volatile");
    check_errno(n)
}

#[inline(always)]
pub unsafe fn syscall2(n: Sysno, a1: usize, a2: usize) -> Result<usize, Errno> {
    let ret: usize;
    let mut n = n;
    asm!("syscall"
         : "+&{r2}"(n), "={r7}"(ret)
         : "{r4}"(a1),
           "{r5}"(a2)
         : "$1", "$3", "$10", "$11", "$12", "$13", "$14", "$15", "$24", "$25", "hi", "lo", "memory"
         : "volatile");
    check_errno(n)
}

#[inline(always)]
pub unsafe fn syscall3(n: Sysno, a1: usize, a2: usize, a3: usize) -> Result<usize, Errno> {
    let ret: usize;
    let mut n = n;
    asm!("syscall"
         : "+&{r2}"(n), "={r7}"(ret)
         : "{r4}"(a1),
           "{r5}"(a2),
           "{r6}"(a3)
         : "$1", "$3", "$10", "$11", "$12", "$13", "$14", "$15", "$24", "$25", "hi", "lo", "memory"
         : "volatile");
    check_errno(n)
}

#[inline(always)]
pub unsafe fn syscall4(
    n: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
) -> Result<usize, Errno> {
    let mut n = n;
    let mut a4 = a4;
    asm!("syscall"
         : "+&{r2}"(n), "+{r7}"(a4)
         : "{r4}"(a1),
           "{r5}"(a2),
           "{r6}"(a3)
         : "$1", "$3", "$10", "$11", "$12", "$13", "$14", "$15", "$24", "$25", "hi", "lo", "memory"
         : "volatile");
    check_errno(n)
}

#[inline(always)]
pub unsafe fn syscall5(
    n: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> Result<usize, Errno> {
    let mut n = n;
    let mut a4 = a4;
    asm!("syscall"
         : "+&{r2}"(n), "+{r7}"(a4)
         : "{r4}"(a1),
           "{r5}"(a2),
           "{r6}"(a3),
           "{r8}"(a5)
         : "$1", "$3", "$10", "$11", "$12", "$13", "$14", "$15", "$24", "$25", "hi", "lo", "memory"
         : "volatile");
    check_errno(n)
}

#[inline(always)]
pub unsafe fn syscall6(
    n: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> Result<usize, Errno> {
    let mut n = n;
    let mut a4 = a4;
    asm!("syscall"
         : "+&{r2}"(n), "+{r7}"(a4)
         : "{r4}"(a1),
           "{r5}"(a2),
           "{r6}"(a3),
           "{r8}"(a5),
           "{r9}"(a6)
         : "$1", "$3", "$10", "$11", "$12", "$13", "$14", "$15", "$24", "$25", "hi", "lo", "memory"
         : "volatile");
    check_errno(n)
}
