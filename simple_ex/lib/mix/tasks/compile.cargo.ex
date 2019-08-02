defmodule Mix.Tasks.Compile.Cargo do
  use Mix.Task

  @shortdoc "Builds Rust part."

  def run(_args) do
    Mix.shell().print_app()
    build()
  end

  defp build do
    System.cmd("cargo", build_cmd_args(), [
      stderr_to_stdout: true,
      into: IO.stream(:stdio, :line)
    ])
  end

  defp build_cmd_args do
    [
      "build",
      "--bin",
      "rust_ex",
      "--release",
      "--target-dir",
      priv_path()
    ]
  end

  defp current_env, do: Application.get_env(:simple_ex, :env)

  defp priv_path do
    "_build/#{current_env()}/lib/simple_ex/priv"
  end
end
