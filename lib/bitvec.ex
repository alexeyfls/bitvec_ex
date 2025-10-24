defmodule Bitvec do
  alias Bitvec.{NifBridge, Types}

  @type t :: Types.bit_vec()

  @spec new() :: Types.unwrapped_result(Types.bit_vec())
  def new() do
    with {:ok, response} <- NifBridge.new(), do: response
  end

  @spec with_capacity(capacity :: non_neg_integer()) :: Types.unwrapped_result(Types.bit_vec())
  def with_capacity(capacity) do
    with {:ok, response} <- NifBridge.with_capacity(capacity), do: response
  end

  @spec capacity(resource :: Types.bit_vec()) :: Types.unwrapped_result(non_neg_integer())
  def capacity(resource) do
    with {:ok, response} <- NifBridge.capacity(resource), do: response
  end

  @spec reserve(resource :: Types.bit_vec(), additional :: non_neg_integer()) ::
          Types.unwrapped_result(Types.bit_vec())
  def reserve(resource, additional) do
    with {:ok} <- NifBridge.reserve(resource, additional), do: resource
  end

  @spec reserve_exact(resource :: Types.bit_vec(), additional :: non_neg_integer()) ::
          Types.unwrapped_result(Types.bit_vec())
  def reserve_exact(resource, additional) do
    with {:ok} <- NifBridge.reserve(resource, additional), do: resource
  end

  @spec shrink_to_fit(resource :: Types.bit_vec()) :: Types.unwrapped_result(Types.bit_vec())
  def shrink_to_fit(resource) do
    with {:ok} <- NifBridge.shrink_to_fit(resource), do: resource
  end

  @spec truncate(resource :: Types.bit_vec(), new_len :: non_neg_integer()) ::
          Types.unwrapped_result(Types.bit_vec())
  def truncate(resource, new_len) do
    with {:ok} <- NifBridge.truncate(resource, new_len), do: resource
  end

  @spec swap_remove(resource :: Types.bit_vec(), index :: non_neg_integer()) ::
          Types.unwrapped_result(boolean())
  def swap_remove(resource, index) do
    with {:ok, response} <- NifBridge.swap_remove(resource, index), do: response
  end

  @spec insert(resource :: Types.bit_vec(), index :: non_neg_integer(), value :: boolean()) ::
          Types.unwrapped_result(Types.bit_vec())
  def insert(resource, index, value) do
    with {:ok} <- NifBridge.insert(resource, index, value), do: resource
  end

  @spec remove(resource :: Types.bit_vec(), index :: non_neg_integer()) ::
          Types.unwrapped_result(boolean())
  def remove(resource, index) do
    with {:ok, response} <- NifBridge.remove(resource, index), do: response
  end

  @spec push(resource :: Types.bit_vec(), value :: boolean()) ::
          Types.unwrapped_result(Types.bit_vec())
  def push(resource, value) do
    with {:ok} <- NifBridge.push(resource, value), do: resource
  end

  @spec pop(resource :: Types.bit_vec()) :: Types.option(boolean()) | Types.common_errors()
  def pop(resource) do
    with {:ok, response} <- NifBridge.pop(resource), do: response
  end

  @spec append(target :: Types.bit_vec(), other :: Types.bit_vec()) ::
          Types.unwrapped_result(Types.bit_vec())
  def append(target, other) do
    with {:ok} <- NifBridge.append(target, other), do: target
  end

  @spec clear(resource :: Types.bit_vec()) :: Types.unwrapped_result(Types.bit_vec())
  def clear(resource) do
    with {:ok} <- NifBridge.clear(resource), do: resource
  end

  @spec len(resource :: Types.bit_vec()) :: Types.unwrapped_result(non_neg_integer())
  def len(resource) do
    with {:ok, response} <- NifBridge.len(resource), do: response
  end

  @spec is_empty(resource :: Types.bit_vec()) :: Types.unwrapped_result(boolean())
  def is_empty(resource) do
    with {:ok, response} <- NifBridge.is_empty(resource), do: response
  end

  @spec split_off(resource :: Types.bit_vec(), at :: non_neg_integer()) ::
          Types.unwrapped_result(Types.bit_vec())
  def split_off(resource, at) do
    with {:ok, response} <- NifBridge.split_off(resource, at), do: response
  end

  @spec resize(
          resource :: Types.bit_vec(),
          new_len :: non_neg_integer(),
          value :: boolean()
        ) ::
          Types.unwrapped_result(Types.bit_vec())
  def resize(resource, new_len, value) do
    with {:ok} <- NifBridge.resize(resource, new_len, value), do: resource
  end

  @spec repeat(bit :: boolean(), len :: non_neg_integer()) ::
          Types.unwrapped_result(Types.bit_vec())
  def repeat(bit, len) do
    with {:ok, response} <- NifBridge.repeat(bit, len), do: response
  end

  @spec from_vec(vec :: [non_neg_integer()]) :: Types.unwrapped_result(Types.bit_vec())
  def from_vec(vec) do
    with {:ok, response} <- NifBridge.from_vec(vec), do: response
  end

  @spec set_uninitialized(resource :: Types.bit_vec(), value :: boolean()) ::
          Types.unwrapped_result(Types.bit_vec())
  def set_uninitialized(resource, value) do
    with {:ok} <- NifBridge.set_uninitialized(resource, value), do: resource
  end

  @spec force_align(resource :: Types.bit_vec()) :: Types.unwrapped_result(Types.bit_vec())
  def force_align(resource) do
    with {:ok} <- NifBridge.force_align(resource), do: resource
  end

  @doc """
  Returns information about this NIF's memory allocations, as reported by jemalloc.
  """
  defdelegate jemalloc_allocation_info, to: NifBridge
end
