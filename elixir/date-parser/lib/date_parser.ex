defmodule DateParser do
  def day(), do: "\\d{1,2}"
  def month(), do: "\\d{1,2}"
  def year(), do: "\\d{4}"

  def day_names(), do: "(Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday)"

  def month_names(),
    do: "(January|February|March|April|May|June|July|August|September|October|November|December)"

  defp create_capture_group(group_name, group_value), do: "(?<#{group_name}>#{group_value})"
  def capture_day(), do: create_capture_group("day", day())
  def capture_month(), do: create_capture_group("month", month())
  def capture_year(), do: create_capture_group("year", year())
  def capture_day_name(), do: create_capture_group("day_name", day_names())
  def capture_month_name(), do: create_capture_group("month_name", month_names())

  def capture_numeric_date(), do: "#{capture_day()}/#{capture_month()}/#{capture_year()}"
  def capture_month_name_date(), do: "#{capture_month_name()} #{capture_day()}, #{capture_year()}"

  def capture_day_month_name_date(),
    do: "#{capture_day_name()}, #{capture_month_name()} #{capture_day()}, #{capture_year()}"

  defp add_prefix_and_suffix(regex_string), do: "^#{regex_string}$"

  def match_numeric_date(),
    do: capture_numeric_date() |> add_prefix_and_suffix() |> Regex.compile!()

  def match_month_name_date(),
    do: capture_month_name_date() |> add_prefix_and_suffix() |> Regex.compile!()

  def match_day_month_name_date(),
    do: capture_day_month_name_date() |> add_prefix_and_suffix() |> Regex.compile!()
end
