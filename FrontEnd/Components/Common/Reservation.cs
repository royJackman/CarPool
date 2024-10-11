using System.Text.Json.Nodes;

public class Reservation
{
    public Reservation(int id, int start, int end, string description)
    {
        Id = id;
        Start = start;
        End = end;
        Description = description;
    }

    public int Id { get; set; }
    public int Start { get; set; }
    public int End { get; set; }
    public string Description { get; set; } = "";

    public static Task<Reservation[]> FetchReservations()
    {
        // TODO: API call to backend
        Reservation reservation = new Reservation(1, 0, int.MaxValue, "Minivan"); // booked forever
        Reservation[] reservations = [reservation];

        return Task.FromResult(reservations);
    }
}