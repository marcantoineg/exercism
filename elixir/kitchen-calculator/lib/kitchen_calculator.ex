defmodule KitchenCalculator do
  def get_volume(volume_pair) do
    elem(volume_pair, 1)
  end

  def to_milliliter(volume_pair)
  def to_milliliter({:cup, _} = volume_pair), do: {:milliliter, get_volume(volume_pair) * 240.0}
  def to_milliliter({:fluid_ounce, _} = volume_pair), do: {:milliliter, get_volume(volume_pair) * 30.0}
  def to_milliliter({:teaspoon, _} = volume_pair), do: {:milliliter, get_volume(volume_pair) * 5.0}
  def to_milliliter({:tablespoon, _} = volume_pair), do: {:milliliter, get_volume(volume_pair) * 15.0}
  def to_milliliter({:milliliter, _} = volume_pair), do: volume_pair

  def from_milliliter(volume_pair, unit)
  def from_milliliter({:milliliter, _} = volume_pair, :cup = unit), do: {unit, get_volume(volume_pair) / 240.0}
  def from_milliliter({:milliliter, _} = volume_pair, :fluid_ounce = unit), do: {unit, get_volume(volume_pair) / 30.0}
  def from_milliliter({:milliliter, _} = volume_pair, :teaspoon = unit), do: {unit, get_volume(volume_pair) / 5.0}
  def from_milliliter({:milliliter, _} = volume_pair, :tablespoon = unit), do: {unit, get_volume(volume_pair) / 15.0}
  def from_milliliter({:milliliter, _} = volume_pair, :milliliter = _unit), do: volume_pair

  def convert({:milliliter, _} = volume_pair, unit), do: from_milliliter(volume_pair, unit)
  def convert(volume_pair, unit), do: to_milliliter(volume_pair) |> from_milliliter(unit)
end
