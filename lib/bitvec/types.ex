defmodule Bitvec.Types do
  @type bit_vec() :: reference()

  @typedoc """
  There are common errors that can be returned from any SortedSet operation, the common_errors
  type enumerates them.

  `{:error, :bad_reference}` is returned any time a reference is passed to the NIF but that
  reference does not identify a SortedSet.

  `{:error, :lock_fail}` is returned when the NIF can not guarantee concurrency safety.  NIFs are
  not bound by the same guarantees as Erlang / Elixir code executing in the BEAM VM, to safe guard
  against multiple threads of execution mutating the same SortedSet concurrently a Mutex is used
  internally to lock the data structure during all operations.

  `{:error, :unsupported_type}` is returned any time an item is passed to the SortedSet that is
  either in whole or in part an unsupported type.  The following types are not supported in
  SortedSet, Reference, Function, Port, and Pid.  Unsupported types poison other types, so a list
  containing a single element (regardless of nesting) of an unsupported type is unsupported, same
  for tuples.
  """
  @type common_errors ::
          {:error, :bad_reference} | {:error, :lock_fail} | {:error, :unsupported_type}

  @type ordering() :: {:msb0 | :lsb0}
end
