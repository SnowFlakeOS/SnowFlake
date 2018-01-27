struct Char
  ZERO = '\0'
  MAX_CODEPOINT = 127

  def -(other : Char)
    ord - other.ord
  end

  def -(other : Int)
    (ord - other).chr
  end

  def +(other : Int)
    (ord + other).chr
  end

  def ===(byte : Int)
    ord === byte
  end

  def <=>(other : Char)
    self - other
  end

  def lowercase?
    'a' <= self <= 'z'
  end

  def uppercase?
    'A' <= self <= 'Z'
  end

  def digit?
    '0' <= self <= '9'
  end

  def letter?
    ascii_lowercase? || ascii_uppercase?
  end

  def whitespace?
    self == ' ' || 0 <= ord <= 13
  end

  def control?
    ord < 0x20 || (0x7F <= ord <= 0x9F)
  end

  def each_byte
    c = ord
    yield c.to_u8
  end
end
