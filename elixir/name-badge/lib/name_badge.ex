defmodule NameBadge do
  def print(id, name, department),
    do: format_id(id) <> name <> " - " <> format_department(department)

  defp format_id(id), do: if(id, do: "[#{id}] - ", else: "")

  defp format_department(department),
    do: if(department, do: String.upcase(department), else: "OWNER")
end
