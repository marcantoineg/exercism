defmodule TakeANumber do
  def start, do: spawn(TakeANumber, :worker, [0])

  @doc false
  def worker(ticket_number) do
    receive do
      {:report_state, sender_id} ->
        send(sender_id, ticket_number)
        worker(ticket_number)
      {:take_a_number, sender_id} ->
        send(sender_id, ticket_number + 1)
        worker(ticket_number + 1)
      :stop -> :ok
      _msg -> worker(ticket_number)
    end
  end
end
