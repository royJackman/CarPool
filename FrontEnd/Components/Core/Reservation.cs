namespace FrontEnd;

public class Reservation
{
    public Reservation(int id, string carName, string userName, long start, long end, string description)
    {
        Id = id;
        CarName = carName;
        UserName = userName;
        Start = start;
        End = end;
        Description = description;
    }

    public int Id { get; }
    public long Start { get; }
    public long End { get; }
    public string Description { get; }
    public string UserName { get; }
    public string CarName { get; }

    public static Task<Reservation[]> FetchReservations()
    {
        // TODO: API call to backend
        Reservation reservation = new Reservation(1, "Minivan", "Jack", 0, int.MaxValue, "For School"); // booked forever
        Reservation[] reservations = [reservation];

        return Task.FromResult(reservations);
    }
}
