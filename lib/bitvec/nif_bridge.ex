defmodule Bitvec.NifBridge do
  use Rustler, otp_app: :bitvec, crate: "bitvec_nif"
  use JemallocInfo.RustlerMixin

  alias Bitvec.Types

  @doc """
  Constructs a new, empty, bit-vector.
  """
  @spec new() :: Types.result(Types.bit_vec())
  def new(), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Allocates a new, empty, bit-vector with space for at least capacity bits before reallocating.
  """
  @spec with_capacity(capacity :: non_neg_integer()) :: Types.result(Types.bit_vec())
  def with_capacity(_capacity), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Gets the allocation capacity, measured in bits.
  """
  @spec capacity(resource :: Types.bit_vec()) :: Types.result(non_neg_integer())
  def capacity(_resource), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Ensures that the bit-vector has allocation capacity for at least additional more bits to be appended to it.
  """
  @spec reserve(resource :: Types.bit_vec(), additional :: non_neg_integer()) :: Types.result(:ok)
  def reserve(_resource, _additional), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Ensures that the bit-vector has allocation capacity for at least additional more bits to be appended to it.
  """
  @spec reserve_exact(resource :: Types.bit_vec(), additional :: non_neg_integer()) ::
          Types.result(:ok)
  def reserve_exact(_resource, _additional), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Releases excess capacity back to the allocator.
  """
  @spec shrink_to_fit(resource :: Types.bit_vec()) :: Types.result(:ok)
  def shrink_to_fit(_resource), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Shortens the bit-vector, keeping the first new_len bits and discarding the rest.
  """
  @spec truncate(resource :: Types.bit_vec(), new_len :: non_neg_integer()) :: Types.result(:ok)
  def truncate(_resource, _new_len), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Takes a bit out of the bit-vector.
  """
  @spec swap_remove(resource :: Types.bit_vec(), index :: non_neg_integer()) ::
          Types.result(boolean())
  def swap_remove(_resource, _index), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Inserts a bit at a given position, shifting all bits after it one spot to the right.
  """
  @spec insert(resource :: Types.bit_vec(), index :: non_neg_integer(), value :: boolean()) ::
          Types.result(:ok)
  def insert(_resource, _index, _value), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Removes a bit at a given position, shifting all bits after it one spot to the left.
  """
  @spec remove(resource :: Types.bit_vec(), index :: non_neg_integer()) ::
          Types.result(boolean())
  def remove(_resource, _index), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Appends a single bit to the vector.
  """
  @spec push(resource :: Types.bit_vec(), value :: boolean()) :: Types.result(:ok)
  def push(_resource, _value), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Attempts to remove the trailing bit from the bit-vector.
  """
  @spec pop(resource :: Types.bit_vec()) :: Types.result(Types.option(boolean()))
  def pop(_resource), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Moves all the bits out of other into the back of target.
  """
  @spec append(target :: Types.bit_vec(), other :: Types.bit_vec()) :: Types.result(:ok)
  def append(_target, _other), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Empties the bit-vector.
  """
  @spec clear(resource :: Types.bit_vec()) :: Types.result(:ok)
  def clear(_resource), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Gets the length of the BitVec.
  """
  @spec len(resource :: Types.bit_vec()) :: Types.result(non_neg_integer())
  def len(_resource), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Tests if the bit-vector is empty.
  """
  @spec is_empty(resource :: Types.bit_vec()) :: Types.result(boolean())
  def is_empty(_resource), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Splits the bit-vector in half at an index, moving resource[at ..] out into a new bit-vector.
  """
  @spec split_off(resource :: Types.bit_vec(), at :: non_neg_integer()) ::
          Types.result(Types.bit_vec())
  def split_off(_resource, _at), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Resizes the bit-vector to a new length. New bits are initialized to value.
  """
  @spec resize(resource :: Types.bit_vec(), new_len :: non_neg_integer(), value :: boolean()) ::
          Types.result(:ok)
  def resize(_resource, _new_len, _value), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Creates a new bit-vector by repeating a bit for the desired length.
  """
  @spec repeat(bit :: boolean(), len :: non_neg_integer()) :: Types.result(Types.bit_vec())
  def repeat(_bit, _len), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Converts a regular vector in-place into a bit-vector.
  """
  @spec from_vec(vec :: [non_neg_integer()]) :: Types.result(Types.bit_vec())
  def from_vec(_vec), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Converts a bit-vector into a Vec of its underlying storage.
  """
  @spec into_vec(resource :: Types.bit_vec()) :: Types.result([non_neg_integer()])
  def into_vec(_resource), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Sets the uninitialized bits of a bit-vector to a known value.
  """
  @spec set_uninitialized(resource :: Types.bit_vec(), value :: boolean()) :: Types.result(:ok)
  def set_uninitialized(_resource, _value), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Ensures that the live region of the bit-vector’s contents begin at the front edge of the buffer.
  """
  @spec force_align(resource :: Types.bit_vec()) :: Types.result(:ok)
  def force_align(_resource), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Exchanges the bit values at two indices.
  """
  @spec swap(resource :: Types.bit_vec(), a :: non_neg_integer(), b :: non_neg_integer()) ::
          Types.result(:ok)
  def swap(_resource, _a, _b), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Reverses the order of bits in a bit-slice.
  """
  @spec reverse(resource :: Types.bit_vec()) :: Types.result(:ok)
  def reverse(_resource), do: :erlang.nif_error(:nif_not_loaded)
end
