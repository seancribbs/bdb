defmodule BdbTest do
  use ExUnit.Case
  doctest Bdb

  test "greets the world" do
    assert Bdb.hello() == :world
  end
end
