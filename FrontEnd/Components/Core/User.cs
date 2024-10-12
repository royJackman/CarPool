namespace FrontEnd;

public class User
{
    public User(int id, string name)
    {
        Id = id;
        Name = name;
    }

    public int Id { get; set; }
    public string Name { get; set; }

    public static Task<User[]> FetchUsers()
    {
        // TODO: API call to backend
        User user = new User(1, "Jack");
        User[] users = [user];

        return Task.FromResult(users);
    }
}
