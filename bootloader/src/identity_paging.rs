use arch_amd64::paging::PagingTable;

const EMPTY_TABLE: PagingTable = PagingTable::new();
static IDENTITY_PAGING_TABLE: [PagingTable; 2] = [EMPTY_TABLE; 2];

const PRESENT_FLAG: u64 = 1;
const PAGE_SIZE_FLAG: u64 = 1 << 7;
const RW_FLAG: u64 = 1 << 1;

pub fn setup_identity_paging() {
    let table = &IDENTITY_PAGING_TABLE;

    for idx in 0..PagingTable::MAX_INDEX {
        let addr = (idx as u64) << 30;
        table[1].store(idx, addr | PRESENT_FLAG | RW_FLAG | PAGE_SIZE_FLAG);
    }

    table[0].store(
        0,
        core::ptr::from_ref(&table[1]) as u64 | PRESENT_FLAG | RW_FLAG,
    );

    unsafe {
        core::arch::asm!(
            "mov cr3, {t}",
            t = in(reg) &table[0]
        );
    }
}
