using StackExchange.Redis;

string connectionString = "redis-learn-dev.redis.cache.windows.net:6380,password=IrISx0bBzvwJHnkSNiybAw0SbGLX86YawAzCaJ0lbts=,ssl=True,abortConnect=False";

using (var cache = ConnectionMultiplexer.Connect(connectionString))
{
    IDatabase database = cache.GetDatabase();

    var result = await database.ExecuteAsync("ping");
    Console.WriteLine($"ping = {result.Type} : {result}");

    bool setValue = await database.StringSetAsync("test:key", "100");
    Console.WriteLine($"set: {setValue}");

    string getValue = await database.StringGetAsync("test:key");
    Console.WriteLine($"get: {getValue}"); 
}