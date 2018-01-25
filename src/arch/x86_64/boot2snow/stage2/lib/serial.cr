# =======================================================================
#  Copyleft SnowFlakeTeam 2018-∞.
#  Distributed under the terms of the BSD 3-clause License.
#  (See accompanying file LICENSE or copy at
#   https://opensource.org/licenses/BSD-3-Clause)
# =======================================================================

# https://github.com/wichtounet/thor-os/blob/develop/init/src/boot_32.cpp

module Serial
    extend self

     @@COM1_PORT : UInt16 = UInt16.new 0x3f8;

    def init
        LibK.outb @@COM1_PORT + 1, UInt8.new 0x00    # Disable all interrupts
        LibK.outb @@COM1_PORT + 3, UInt8.new 0x80    # Enable DLAB
        LibK.outb @@COM1_PORT + 0, UInt8.new 0x03    # 38400 baud
        LibK.outb @@COM1_PORT + 1, UInt8.new 0x00
        LibK.outb @@COM1_PORT + 3, UInt8.new 0x03    # 8 bits, no parity, one stop bit
        LibK.outb @@COM1_PORT + 2, UInt8.new 0xC7    # Enable FIFO, clear them, with 14-byte threshold
        LibK.outb @@COM1_PORT + 4, UInt8.new 0x0B    # IRQs enabled, RTS/DSR set
    end
end