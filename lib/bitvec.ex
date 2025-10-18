defmodule Bitvec do
  alias Bitvec.{NifBridge, Types}

  @type t :: Types.bit_vec()

  @spec new(capacity :: pos_integer()) :: t() | Types.common_errors()
  def new(capacity) do
    {:ok, resource} = NifBridge.new(capacity)
    resource
  end

  @spec len(resource :: t()) :: pos_integer() | Types.common_errors()
  def len(resource) do
    {:ok, len} = NifBridge.len(resource)
    len
  end

  @doc """
  Returns information about this NIF's memory allocations, as reported by jemalloc.
  """
  defdelegate jemalloc_allocation_info, to: NifBridge
end
