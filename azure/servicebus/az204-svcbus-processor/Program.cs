using Azure.Messaging.ServiceBus;

string connectionString = "Endpoint=sb://sb-learn-dev599.servicebus.windows.net/;SharedAccessKeyName=RootManageSharedAccessKey;SharedAccessKey=o6tVa/g11zatah6C0TieGVXKHQ0x595SyUSq9NCTTUY=";
string queueName = "sbq-learndevquery";

ServiceBusClient client = new(connectionString);
ServiceBusProcessor processor = client.CreateProcessor(queueName, new ServiceBusProcessorOptions());

try
{
    processor.ProcessMessageAsync += ProcessMessageAsync;
    processor.ProcessErrorAsync += ProcessErrorAsync;
    
    await processor.StartProcessingAsync();

    Console.WriteLine("Wait for a minute and then press any key to end the processing");
    Console.ReadKey();

    // stop processing 
    Console.WriteLine("\nStopping the receiver...");
    await processor.StopProcessingAsync();
    Console.WriteLine("Stopped receiving messages");
}
catch (Exception exception)
{
    Console.WriteLine($"Error: {exception.Message}");
}
finally
{
    await processor.DisposeAsync();
    await client.DisposeAsync();
}

async Task ProcessErrorAsync(ProcessErrorEventArgs arg)
{
    Console.Write(arg.Exception.ToString());
    await Task.CompletedTask;
}

async Task ProcessMessageAsync(ProcessMessageEventArgs arg)
{
    string body = arg.Message.Body.ToString();
    Console.WriteLine($"Message: {body}");

    await arg.CompleteMessageAsync(arg.Message);
}