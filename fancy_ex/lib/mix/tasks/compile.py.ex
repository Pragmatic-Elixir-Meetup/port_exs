defmodule Mix.Tasks.Compile.Py do
  use Mix.Task

  @shortdoc "Builds Python part."

  def run(_args) do
    Mix.shell().print_app()
    build()
  end

  defp build do
    mkdir()
    copy()
  end

  def copy, do: File.cp_r!(src_path(), dest_path())

  defp current_env, do: Application.get_env(:fancy_ex, :env)

  defp dest_path do
    "_build/#{current_env()}/lib/fancy_ex/priv/py"
  end

  defp mkdir, do: File.mkdir_p!(dest_path())

  def src_path, do: "py"
end
