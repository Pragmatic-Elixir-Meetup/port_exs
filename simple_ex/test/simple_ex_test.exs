defmodule SimpleExTest do
  use ExUnit.Case
  doctest SimpleEx

  test "greets the world" do
    assert SimpleEx.hello() == :world
  end
end
