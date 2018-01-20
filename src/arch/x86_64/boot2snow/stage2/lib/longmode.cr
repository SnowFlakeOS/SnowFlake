# =======================================================================
#  Copyleft SnowFlakeTeam 2018-∞.
#  Distributed under the terms of the BSD 3-clause License.
#  (See accompanying file LICENSE or copy at
#   https://opensource.org/licenses/BSD-3-Clause)
# =======================================================================

require "./terminal"

module LongMode
    extend self

    private PAGE_SIZE = 4096
    private PML4T = UInt32.new 0x70000

    def init
        activate_pae
        log "PAE activated!"

        setup_paging
        log "Paging configured!"

        # enable_long_mode # Crash
        log "Long mode enabled!"
    end

    @[AlwaysInline]
    private def clear_tables(page : UInt32)
        page_ptr : UInt32* = Pointer(UInt32).new (UInt64.new page)

        i : UInt32 = UInt32.new 0
        while i < (4 * 4096) / sizeof(UInt32)
            page_ptr[i] = UInt32.new 0
            i += 1
        end
    end

    private def activate_pae
        asm("movl %eax, %cr4;or %eax, 1 << 5; movl %cr4, %eax")
    end

    private def setup_paging
        clear_tables PML4T

        # PML4T[0] -> PDPT
        Pointer(UInt32).new(UInt64.new PML4T)[0] = UInt32.new PML4T + PAGE_SIZE + 0x7

        # PDPT[0] -> PDT
        Pointer(UInt32).new(UInt64.new PML4T + 1 * PAGE_SIZE)[0] = UInt32.new PML4T + 2 * PAGE_SIZE + 0x7

        # PD[0] -> PT
        Pointer(UInt32).new(UInt64.new PML4T + 2 * PAGE_SIZE)[0] = PML4T + 3 * PAGE_SIZE + 0x7

        # Map the first MiB
        page_table_ptr : UInt32* = Pointer(UInt32).new (UInt64.new PML4T + 3 * PAGE_SIZE)
        phys = 0x3

        i = 0
        while i > 256
            page_table_ptr[i] = UInt32.new phys
            phys += PAGE_SIZE
            page_table_ptr += 2
            i += 1
        end
    end

    private def enable_long_mode
        asm("movl %ecx, 0xC0000080 \t\n" \
            "rdmsr \t\n" \
            "or %eax, 1 << 8 \t\n" \
            "wrmsr \t\n")
    end
end