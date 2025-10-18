defmodule Bitvec.MixProject do
  use Mix.Project

  def project do
    [
      app: :bitvec,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: Mix.compilers(),
      elixirc_paths: elixirc_paths(Mix.env())
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.37.1", runtime: false},
      {:jemalloc_info, "~> 0.3", app: false}
    ]
  end

  defp elixirc_paths(:test), do: elixirc_paths(:default)
  defp elixirc_paths(_), do: ["lib"]
end
