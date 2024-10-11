namespace FrontEnd;

public class Car
{
    public Car(int id, string name)
    {
        Id = id;
        Name = name;
    }

    public int Id { get; set; }
    public string Name { get; set; }

    public static Task<Car[]> FetchCars()
    {
        // TODO: API call to backend
        Car car = new Car(1, "Minivan");
        Car[] cars = [car];

        return Task.FromResult(cars);
    }
}