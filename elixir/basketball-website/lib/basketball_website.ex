defmodule BasketballWebsite do
  @spec extract_from_path(map(), String.t()) :: any()
  def extract_from_path(data, path) do
    keys = String.split(path, ".")

    for k <- keys, into: "" do
      extract_from_path(data, k)
    end
  end

  def get_in_path(data, path) do
    Kernel.get_in(data, String.split(path, "."))
  end
end
