# =======================================================================
#  Copyleft SnowFlakeTeam 2018-∞.
#  Distributed under the terms of the BSD 3-clause License.
#  (See accompanying file LICENSE or copy at
#   https://opensource.org/licenses/BSD-3-Clause)
# =======================================================================

# https://github.com/wichtounet/thor-os.git

require "../runtime/src/prelude"

lib LibATA
  # IDE Controllers
  ATA_PRINMARY = 0x1F0
  ATA_SECENDARY = 0x170

  # I/O Controllers ports
  ATA_DATA = 0
  ATA_ERROR = 1
  ATA_NSECTOR = 2
  ATA_SECTOR = 3
  ATA_LCYL = 4
  ATA_HCYL = 5
  ATA_DRV_HEAD = 6
  ATA_STATUS = 7
  ATA_COMMAND = 7
  ATA_DEV_CTL = 0x206

  # Status bits
  ATA_STATUS_BSY = 0x80
  ATA_STATUS_DRDY = 0x40
  ATA_STATUS_DRQ = 0x08
  ATA_STATUS_ERR = 0x01
  ATA_STATUS_DF = 0x20

  # Commands
  ATA_IDENTIFY = 0xEC
  ATAPI_IDENTIFY = 0xA1
  ATA_READ_BLOCK = 0x20
  ATA_WRITE_BLOCK = 0x30

  ATA_CTL_SRST = 0x04
  ATA_CTL_nIEN = 0x02

  # Master / Slave on devices
  MASTER_BIT = 0
  SLAVE_BIT = 1
end

module Disks
    extend self

    def init
        detect_disks
        # TODO
        log "Disks initialized!"
    end

    def detect_disks
        LibK.outb UInt16.new(LibATA::ATA_PRINMARY + LibATA::ATA_DEV_CTL), UInt8.new LibATA::ATA_CTL_nIEN
        LibK.outb UInt16.new(LibATA::ATA_SECENDARY + LibATA::ATA_DEV_CTL), UInt8.new LibATA::ATA_CTL_nIEN
        
        LibK.outb UInt16.new(LibATA::ATA_PRINMARY + LibATA::ATA_NSECTOR), UInt8.new 0xAB
        if LibK.inb(UInt16.new(LibATA::ATA_PRINMARY + LibATA::ATA_NSECTOR)) != 0xAB
            suspend "The primary ATA controller is not enabled"
        end

        block_buffer : Char[512]

        # if !read_sector 0, block_buffer
            # suspend "Unable to read MBR"
        # end
    end

    # def read_sector(start : UInt64)
end