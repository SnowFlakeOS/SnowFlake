class Object
  # abstract def ==(other)

  def !=(other)
    !(self == other)
  end

  def !~(other)
    !(self =~ other)
  end

  def ===(other)
    self == other
  end

  def =~(other)
    nil
  end
end
