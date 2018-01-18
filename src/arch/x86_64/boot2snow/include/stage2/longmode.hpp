//=======================================================================
// Copyright Baptiste Wicht 2013-2016.
// Distributed under the terms of the MIT License.
// (See accompanying file LICENSE or copy at
//  http://www.opensource.org/licenses/MIT)
//=======================================================================

// https://github.com/wichtounet/thor-os/tree/develop/init

#ifndef LONGMODE_HPP
#define LONGMODE_HPP

#include <types.hpp>
#include <stage2/ealy_memory.hpp>
#include <stage2/console.hpp>

class longmode {
    static const size_t PAGE_SIZE = 4096;
    const uint32_t PML4T = 0x70000;

    inline void clear_tables(uint32_t page){
        auto page_ptr = reinterpret_cast<uint32_t*>(page);

        for(uint32_t i = 0; i < (4 * 4096) / sizeof(uint32_t); ++i){
            *page_ptr++ = 0;
        }
    }

    public:
        void init() {
            console c;

            activate_pae();
            c.log("PAE Activated!");

            setup_paging();
            c.log("Paging configured!");

            enable_long_mode();
            c.log("Long mode enabled!");

            set_pml4t();
            c.log("PML4T set!");

            enable_paging();
            c.log("Paging enabled!");
        }

        void setup_kernel_paging(uint32_t kernel_mib);

    private:
        void activate_pae();
        void setup_paging();
        void enable_long_mode();
        void set_pml4t();
        void enable_paging();
};

#endif