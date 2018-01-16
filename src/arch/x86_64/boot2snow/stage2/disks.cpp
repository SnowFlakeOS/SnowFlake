// https://github.com/wichtounet/thor-os/tree/develop/init
// http://wiki.osdev.org/User:Requimrar/FAT32

#include <stage2/disks.hpp>

void disks::init() {
    get();

    printf("\nTotal Sectors : %d\n", fat_bs->total_sectors);
    printf("Sectors Per Cluster : %d\n", fat_bs->sectors_per_cluster);
    printf("Root Directory Cluster Start : %d\n", fat_bs->root_directory_cluster_start);
    
    printf("clusterToLBA(fat_bs->root_directory_cluster_start) : %d\n", clusterToLBA(fat_bs->root_directory_cluster_start));
}

void disks::get() {
    auto bytes_per_sector = *((uint16_t*)((size_t) BOOTSECTOR_BASE + 0x0b));
    auto sectors_per_cluster = *((uint8_t*)((size_t) BOOTSECTOR_BASE + 0x0d));
    auto reserved_sectors = *((uint16_t*)((size_t) BOOTSECTOR_BASE + 0x0e));
    auto number_of_fat = *((uint8_t*)((size_t) BOOTSECTOR_BASE + 0x10));number_of_fat = *((uint8_t*)((size_t) BOOTSECTOR_BASE + 0x10));
    auto total_sectors_16 = *((uint16_t*)((size_t) BOOTSECTOR_BASE + 0x13));
    auto total_sectors_32 = *((uint32_t*)((size_t) BOOTSECTOR_BASE + 0x20));
    auto sectors_per_fat_16 = *((uint32_t*)((size_t) BOOTSECTOR_BASE + 0x16));
    auto sectors_per_fat_32 = *((uint32_t*)((size_t) BOOTSECTOR_BASE + 0x24));
    auto root_directory_cluster_start = *((uint32_t*)((size_t) BOOTSECTOR_BASE + 0x2C));

    auto total_sectors = (total_sectors_16 == 0)? total_sectors_32 : total_sectors_16;
    auto sectors_per_fat = (sectors_per_fat_16 == 0)? sectors_per_fat_32: sectors_per_fat_16;

    fat_bs->bytes_per_sector = bytes_per_sector;
    fat_bs->sectors_per_cluster = sectors_per_cluster;
    fat_bs->reserved_sectors = reserved_sectors;
    fat_bs->number_of_fat = number_of_fat;
    fat_bs->total_sectors = total_sectors;
    fat_bs->total_sectors_long = total_sectors_32;
    fat_bs->sectors_per_fat = sectors_per_fat;
    fat_bs->sectors_per_fat_long = sectors_per_fat_32;
    fat_bs->root_directory_cluster_start = root_directory_cluster_start;
}

uint64_t disks::clusterToLBA(uint32_t cluster) {
    auto root_directory_sectors = ((fat_bs->root_directory_cluster_start * 32) + (fat_bs->bytes_per_sector - 1)) / fat_bs->bytes_per_sector;
    printf("\nauto root_directory_sectors = %d\n", root_directory_sectors);
    auto data_sectors = fat_bs->total_sectors - (fat_bs->reserved_sectors + (fat_bs->number_of_fat * fat_bs->sectors_per_fat) + root_directory_sectors);
    printf("auto data_sectors = %d\n", data_sectors);
    auto total_clusters = data_sectors / fat_bs->sectors_per_cluster;
    printf("auto total_clusters = %d\n", total_clusters);
    auto first_data_sector = fat_bs->reserved_sectors + (fat_bs->number_of_fat * fat_bs->sectors_per_fat) + root_directory_sectors;
    printf("auto first_data_sector = %d\n", first_data_sector);
    auto first_sector_of_cluster = ((cluster - 2) * fat_bs->sectors_per_cluster) + first_data_sector;
    printf("auto first_sector_of_cluster = %d\n\n", first_sector_of_cluster);
    return first_sector_of_cluster + cluster * fat_bs->sectors_per_cluster - (2 * fat_bs->sectors_per_cluster);
}