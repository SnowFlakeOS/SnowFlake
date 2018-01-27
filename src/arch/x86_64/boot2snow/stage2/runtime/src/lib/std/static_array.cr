struct StaticArray(T, N)
  include Indexable(T)
  
  macro [](*args)
    %array = uninitialized StaticArray(typeof({{*args}}), {{args.size}})
    {% for arg, i in args %}
      %array.to_unsafe[{{i}}] = {{arg}}
    {% end %}
    %array
  end

  def self.new(&block : Int32 -> T)
    array = uninitialized self
    N.times do |i|
      array.to_unsafe[i] = yield i
    end
    array
  end

  def self.new(value : T)
    new { value }
  end

  private def initialize
    new self
  end

  def ==(other : StaticArray)
    return false unless size == other.size
    each_with_index do |e, i|
      return false unless e == other[i]
    end
    true
  end

  def ==(other)
    false
  end

  @[AlwaysInline]
  def unsafe_at(index : Int)
    to_unsafe[index]
  end

  @[AlwaysInline]
  def [](index : Int)
    to_unsafe[index]
  end

  @[AlwaysInline]
  def []=(index : Int, value : T)
    index = check_index_out_of_bounds index
    to_unsafe[index] = value
  end

  def update(index : Int)
    index = check_index_out_of_bounds index
    to_unsafe[index] = yield to_unsafe[index]
  end

  def size
    N
  end

  def []=(value : T)
    size.times do |i|
      to_unsafe[i] = value
    end
  end

  def to_unsafe : Pointer(T)
    pointerof(@buffer)
  end

  def clone
    array = uninitialized self
    N.times do |i|
      array.to_unsafe[i] = to_unsafe[i].clone
    end
    array
  end

  private def check_index_out_of_bounds(index)
    check_index_out_of_bounds(index) { raise "Index Error" }
  end

  private def check_index_out_of_bounds(index)
    index += size if index < 0
    if 0 <= index < size
      index
    else
      yield
    end
  end
end
