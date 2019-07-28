defmodule FancyExTest do
  use ExUnit.Case
  doctest FancyEx

  test "greets the world" do
    assert FancyEx.hello() == :world
  end
end
