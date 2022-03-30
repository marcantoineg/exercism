defmodule Username do

  @spec sanitize([char]) :: [char]
  def sanitize(username = []), do: username
  def sanitize(username) do
    Enum.map(username, fn char ->
      case char do
        char when char == ?_ -> char
        char when char == ?ä -> 'ae'
        char when char == ?ö -> 'oe'
        char when char == ?ü -> 'ue'
        char when char == ?ß -> 'ss'
        char when char >= ?a and char <= ?z -> char # a-z
        _ -> nil
      end
    end)
    |> List.flatten
    |> Enum.filter(& &1)
  end
end
