#ifndef DISKS_HPP
#define DISKS_HPP

#include <types.hpp>
#include <string.h>
#include <stage2/io.hpp>
#include <stage2/console.hpp>
#include <stage2/ealy_memory.hpp>
#include <stage2/longmode.hpp>
#include <stage2/fat32_specs.hpp>

using namespace fat32;

class disks {
    static constexpr const uint32_t BOOTSECTOR_BASE = 0x7C00;

    static constexpr const uint32_t CLUSTER_FREE = 0x0;
    static constexpr const uint32_t CLUSTER_RESERVED= 0x1;
    static constexpr const uint32_t CLUSTER_CORRUPTED = 0x0FFFFFF7;
    static constexpr const uint32_t CLUSTER_END = 0x0FFFFFF8;

    public:
        void init();

    private:
        void get();

        uint64_t clusterToLBA(uint32_t cluster);

        console c;
        io sys;
        longmode lm;

        fat_bs_t *fat_bs = nullptr;
};

#endif