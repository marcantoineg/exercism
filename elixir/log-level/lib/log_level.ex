defmodule LogLevel do
  def to_label(level, legacy?) do
    case {level, legacy?} do
      {0, false} -> :trace
      {0, true} -> :unknown
      {1, _} -> :debug
      {2, _} -> :info
      {3, _} -> :warning
      {4, _} -> :error
      {5, false} -> :fatal
      {5, true} -> :unknown
      _ -> :unknown
    end
  end

  def alert_recipient(level, legacy?) do
    case {to_label(level, legacy?), legacy?} do
      {:error, _} -> :ops
      {:fatal, _} -> :ops
      {:unknown, true} -> :dev1
      {:unknown, false} -> :dev2
      _ -> false
    end
  end
end
