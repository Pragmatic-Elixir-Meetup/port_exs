defmodule FancyServer do
  use GenServer

  def start do
    GenServer.start_link(__MODULE__, nil)
  end

  def stop do
    GenServer.stop(__MODULE__)
  end

  @impl true
  def init(nil) do


    # gupeng

    port = Port.open({:spawn_executable, executable}, [
      {:packet, 2},
      :binary,
      :exit_status,
      :use_stdio
      ])

    state = %{port: port}

    {:ok, state}
  end

  @impl true
  def handle_info({_port, {:data, << data :: binary >>}}, state) do
    code = :erlang.binary_to_term(data)

    IO.puts "elixir_FancyServer: Receives - #{code}, invoking method `fun` with message in `py_ex.py`"

    call_python(:py_ex, :fun, [code])

    {:noreply, state}
  end
end
