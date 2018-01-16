//=======================================================================
// Copyright Baptiste Wicht 2013-2016.
// Distributed under the terms of the MIT License.
// (See accompanying file LICENSE or copy at
//  http://www.opensource.org/licenses/MIT)
//=======================================================================

// https://github.com/wichtounet/thor-os/tree/develop/init

#include <stage2/longmode.hpp>

void longmode::activate_pae() {
    asm volatile("mov eax, cr4; or eax, 1 << 5; mov cr4, eax");
}

void longmode::setup_paging() {
    //Clear all tables
    clear_tables(PML4T);

    //Link tables (0x3 means Writeable and Supervisor)

    //PML4T[0] -> PDPT
    *reinterpret_cast<uint32_t*>(PML4T) = PML4T + PAGE_SIZE + 0x7;

    //PDPT[0] -> PDT
    *reinterpret_cast<uint32_t*>(PML4T + 1 * PAGE_SIZE) = PML4T + 2 * PAGE_SIZE + 0x7;

    //PD[0] -> PT
    *reinterpret_cast<uint32_t*>(PML4T + 2 * PAGE_SIZE) = PML4T + 3 * PAGE_SIZE + 0x7;

    //Map the first MiB

    auto page_table_ptr = reinterpret_cast<uint32_t*>(PML4T + 3 * PAGE_SIZE);
    auto phys = 0x3;
    for(uint32_t i = 0; i < 256; ++i){
        *page_table_ptr = phys;

        phys += PAGE_SIZE;

        //A page entry is 64 bit in size
        page_table_ptr += 2;
    }
}

void longmode::enable_long_mode() {
    asm volatile(
        "mov ecx, 0xC0000080 \t\n"
        "rdmsr \t\n"
        "or eax, 0b100000000 \t\n"
        "wrmsr \t\n");
}

void longmode::set_pml4t() {
    asm volatile(
        "mov eax, 0x70000 \t\n"  // Bass address of PML4
        "mov cr3, eax \t\n"); // load page-map level-4 base
}

void longmode::enable_paging(){
    asm volatile(
        "mov eax, cr0 \t\n"
        "or eax, 0b10000000000000000000000000000000 \t\n"
        "mov cr0, eax \t\n");
}

void longmode::setup_kernel_paging(uint32_t kernel_mib){
    static_assert(early::kernel_address == 0x100000, "Only 0x100000 has been implemented");

    //Map all the kernel

    auto current_pt = 0;

    auto page_table_ptr = reinterpret_cast<uint32_t*>(PML4T + 3 * PAGE_SIZE + 256 * 8);
    auto phys = 0x100003;
    for(uint32_t i = 256; i < (1 + kernel_mib) * 256; ++i){
        *page_table_ptr = phys;

        phys += PAGE_SIZE;

        //A page entry is 64 bit in size
        page_table_ptr += 2;

        if(i % 511 == 0){
            ++current_pt;

            //PD[current_pt] -> PT
            *reinterpret_cast<uint32_t*>(PML4T + 2 * PAGE_SIZE + current_pt * 8) = PML4T + (3 + current_pt) * PAGE_SIZE + 0x7;
        }
    }
}