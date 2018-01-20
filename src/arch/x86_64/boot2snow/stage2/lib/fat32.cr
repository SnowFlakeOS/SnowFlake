# =======================================================================
#  Copyleft SnowFlakeTeam 2018-∞.
#  Distributed under the terms of the BSD 3-clause License.
#  (See accompanying file LICENSE or copy at
#   https://opensource.org/licenses/BSD-3-Clause)
# =======================================================================

require "./terminal"

lib LibFAT32
  @[Packed]
  struct BootSector
    jump : UInt8[3]
    oem_name : Char[8]
    bytes_per_sector : UInt16
    sectors_per_cluster : UInt8
    reserved_sectors : UInt16
    number_of_fat : UInt8
    root_directories_entries : UInt16
    total_sectors : UInt16
    media_descriptor : UInt8
    sectors_per_fat : UInt16
    sectors_per_track : UInt16
    heads : UInt16
    hidden_sectors : UInt32
    total_sectors_long : UInt32
    sectors_per_fat_long : UInt32
    drive_description : UInt16
    version : UInt16
    root_directory_cluster_start : UInt32
    fs_information_sector : UInt16
    boot_sectors_copy_sector : UInt16
    filler : UInt8[12]
    physical_drive_number : UInt8
    reserved : UInt8
    extended_boot_signature : UInt8
    volume_id : UInt32
    volume_label : Char[11]
    file_system_type : Char[8]
    boot_code : UInt8[420]
    signature : UInt16
  end
end

module FAT32
    extend self

    def init
        # TODO
        log "FAT32 initialized!"
    end
end