defmodule Bitvec.NifBridge do
  use Rustler, otp_app: :bitvec, crate: "bitvec_nif"
  use JemallocInfo.RustlerMixin

  alias Bitvec
  alias Bitvec.Types

  @doc """
  Creates a new BitVec.
  """
  @spec new(capacity :: pos_integer(), ordering :: Types.ordering()) :: {:ok, Bitvec.t()}
  def new(_capacity, _ordering), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Gets the length of the BitVec.
  """
  @spec len(resource :: Bitvec.t()) :: {:ok, pos_integer()} | Types.common_errors()
  def len(_resource), do: :erlang.nif_error(:nif_not_loaded)
end
