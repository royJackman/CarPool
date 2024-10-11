namespace FrontEnd;

using System.Collections.Generic;
using System.Linq;

public class Day
{
    public Day(DateTime? date, Reservation[]? reservations)
    {
        Date = date;
        Reservations = reservations;
    }
    public DateTime? Date;
    public Reservation[]? Reservations;
    public static Task<Day[]> getDays(Reservation[] reservations)
    {
        // get current day
        DateTime now = DateTime.Now;
        DateTime first = new DateTime(now.Year, now.Month, 1);
        // decide which cell it fits in
        int firstCell = (int)(first.DayOfWeek); // weekday number after cast

        // fill cells
        List<Day> calendar = new List<Day>();
        for (int i = 0; i < firstCell; i++)
            calendar.Add(new Day(null, null));

        // render rest of the month in days in order
        for (int d = 0; d <= DateTime.DaysInMonth(now.Year, now.Month) - now.Day; d++)
        {
            DateTime day = now.AddDays(d);
            DateTime dayStart = new DateTime(day.Year, day.Month, day.Day);
            DateTime dayEnd = dayStart.AddHours(24);
            DateTimeOffset dayStartOffset = new DateTimeOffset(dayStart);
            DateTimeOffset dayEndOffset = new DateTimeOffset(dayEnd);

            Reservation[] dayReservations = filterReservations(reservations, dayStartOffset.ToUnixTimeSeconds(),
            dayEndOffset.ToUnixTimeSeconds());
            calendar.Add(new Day(day, dayReservations));
        }

        for (int i = calendar.Count(); i <= 35; i++)
            calendar.Add(new Day(null, null));


        return Task.FromResult(calendar.ToArray());
    }

    public static Reservation[] filterReservations(Reservation[] reservations, long dayStart, long dayEnd)
    {
        if (reservations == null)
            return [];
        return reservations.Where(r => r.Start <= dayEnd && dayStart <= r.End).ToArray();
    }
}