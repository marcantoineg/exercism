defmodule RationalNumbers do
  @type rational :: {integer, integer}

  @doc """
  Add two rational numbers
  (a₁ * b₂ + a₂ * b₁) / (b₁ * b₂)
  """
  @spec add(a :: rational, b :: rational) :: rational
  def add(a, b) do
    {
      elem(a, 0) * elem(b, 1) + elem(a, 1) * elem(b, 0),
      elem(b, 0) * elem(b, 1)
    }
    |> reduce
  end

  @doc """
  Subtract two rational numbers
  (a₁ * b₂ - a₂ * b₁) / (b₁ * b₂)
  """
  @spec subtract(a :: rational, b :: rational) :: rational
  def subtract(a, b) do
    {
      (elem(a, 0) * elem(b, 1)) - (elem(a, 1) * elem(b, 0)),
      elem(b, 0) * elem(b, 1)
    }
    |> reduce
  end

  @doc """
  Multiply two rational numbers
  (a₁ * a₂) / (b₁ * b₂)
  """
  @spec multiply(a :: rational, b :: rational) :: rational
  def multiply(a, b) do
  end

  @doc """
  Divide two rational numbers
  (a₁ * b₂) / (a₂ * b₁) if a₂ is not zero.
  """
  @spec divide_by(num :: rational, den :: rational) :: rational
  def divide_by(num, den) do
  end

  @doc """
  Absolute value of a rational number
  """
  @spec abs(a :: rational) :: rational
  def abs(a) do
    {
      Kernel.abs(elem(a, 0)),
      Kernel.abs(elem(a, 1))
    }
    |> reduce
  end

  @doc """
  Exponentiation of a rational number by an integer
  """
  @spec pow_rational(a :: rational, n :: integer) :: rational
  def pow_rational(a, n) do
  end

  @doc """
  Exponentiation of a real number by a rational number
  """
  @spec pow_real(x :: integer, n :: rational) :: float
  def pow_real(x, n) do
  end

  @doc """
  Reduce a rational number to its lowest terms
  """
  @spec reduce(a :: rational) :: rational
  def reduce(a) do
    gcd = Integer.gcd(elem(a, 0), elem(a, 1))

    reduced_a = {
      trunc(elem(a, 0) / gcd),
      trunc(elem(a, 1) / gcd)
    }

    case reduced_a do
      {a1, a2} when a2 < 0 -> {a1 * -1, a2 * -1}
      _ -> reduced_a
    end
  end
end
