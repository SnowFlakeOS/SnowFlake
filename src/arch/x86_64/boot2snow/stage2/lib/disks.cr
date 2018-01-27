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
  ATA_PRIMARY = 0x1F0
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

    @[AlwaysInline]
    private def ata_400ns_delay(controller : UInt16)
        LibK.inb controller + LibATA::ATA_STATUS
        LibK.inb controller + LibATA::ATA_STATUS
        LibK.inb controller + LibATA::ATA_STATUS
        LibK.inb controller + LibATA::ATA_STATUS
        LibK.inb controller + LibATA::ATA_STATUS
        LibK.inb controller + LibATA::ATA_STATUS
        LibK.inb controller + LibATA::ATA_STATUS
        LibK.inb controller + LibATA::ATA_STATUS
        LibK.inb controller + LibATA::ATA_STATUS
    end

    def init
        detect_disks
        # TODO
        log "Disks initialized!"
    end

    private def detect_disks
        LibK.outb UInt16.new(LibATA::ATA_PRIMARY + LibATA::ATA_DEV_CTL), UInt8.new LibATA::ATA_CTL_nIEN
        LibK.outb UInt16.new(LibATA::ATA_SECENDARY + LibATA::ATA_DEV_CTL), UInt8.new LibATA::ATA_CTL_nIEN
        
        LibK.outb UInt16.new(LibATA::ATA_PRIMARY + LibATA::ATA_NSECTOR), UInt8.new 0xAB
        if LibK.inb(UInt16.new(LibATA::ATA_PRIMARY + LibATA::ATA_NSECTOR)) != 0xAB
            suspend "The primary ATA controller is not enabled"
        end

        if !read_sector UInt16.new 0
           suspend "Unable to read MBR"
        end
    end

    private def read_sector(start : UInt16) : Bool
        # Select the device
        if !ata_select_device
            false
        end

        sc : UInt8 = UInt8.new(start & 0xFF)
        cl : UInt8 = UInt8.new((start >> 8) & 0xFF)
        ch : UInt8 = UInt8.new((start >> 16) & 0xFF)
        hd : UInt8 = UInt8.new((start >> 24) & 0x0F)

        # Process the command
        LibK.outb UInt16.new(LibATA::ATA_PRIMARY + LibATA::ATA_NSECTOR), UInt8.new 1
        LibK.outb UInt16.new(LibATA::ATA_PRIMARY + LibATA::ATA_SECTOR), sc
        LibK.outb UInt16.new(LibATA::ATA_PRIMARY + LibATA::ATA_LCYL), cl
        LibK.outb UInt16.new(LibATA::ATA_PRIMARY + LibATA::ATA_HCYL), ch
        LibK.outb UInt16.new(LibATA::ATA_PRIMARY + LibATA::ATA_DRV_HEAD), UInt8.new((1 << 6) | (LibATA::MASTER_BIT << 4) | hd)
        LibK.outb UInt16.new(LibATA::ATA_PRIMARY + LibATA::ATA_COMMAND), UInt8.new LibATA::ATA_READ_BLOCK

        # Wait at most 30 seconds for BSY flag to be cleared
        if !wait_for_controller UInt16.new(LibATA::ATA_PRIMARY), UInt8.new(LibATA::ATA_STATUS_BSY), UInt8.new(0), UInt16.new 30000
            false
        end

        # Verify if there are errors
        if LibK.inb(UInt16.new LibATA::ATA_PRIMARY + LibATA::ATA_STATUS) & LibATA::ATA_STATUS_ERR
            false
        end

        # Polling
        while true
            ata_400ns_delay UInt16.new LibATA::ATA_PRIMARY

            status = LibK.inb UInt16.new(LibATA::ATA_PRIMARY + LibATA::ATA_STATUS)

            if status & LibATA::ATA_STATUS_BSY

            end

            if status & LibATA::ATA_STATUS_ERR || status & LibATA::ATA_STATUS_DF
                false
            end

            if status & LibATA::ATA_STATUS_DRQ
                break
            end
        end

        # Read the disk sectors
        i = 0
        while i < 256
            # @@block_buffer[i] = LibK.inw UInt16.new(LibATA::ATA_PRIMARY + LibATA::ATA_DATA)
            i += 1
        end

        true
    end

    private def ata_select_device : Bool
        wait_mask = LibATA::ATA_STATUS_BSY | LibATA::ATA_STATUS_DRQ

        if !wait_for_controller UInt16.new(LibATA::ATA_PRIMARY), UInt8.new(wait_mask), UInt8.new(0), UInt16.new(10000)
            false
        end

        #Indicate the selected devic
        LibK.outb UInt16.new(LibATA::ATA_PRIMARY + LibATA::ATA_DRV_HEAD), UInt8.new(0xA0 | (LibATA::MASTER_BIT << 4))

        if !wait_for_controller UInt16.new(LibATA::ATA_PRIMARY), UInt8.new(wait_mask), UInt8.new(0), UInt16.new(10000)
            false
        end

        true
    end

    private def wait_for_controller(controller : UInt16, mask : UInt8, value : UInt8, timeout : UInt16)
        status : UInt8 = UInt8.new 0;
        while (status & mask) != value && (timeout -= 1)
            # Sleep at least 400ns before reading the status register
            ata_400ns_delay controller

            # Final read of the controller status
            status = LibK.inb controller + LibATA::ATA_STATUS
        end
        timeout
    end
end