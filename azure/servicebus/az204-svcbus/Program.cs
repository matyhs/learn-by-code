using Azure.Messaging.ServiceBus;

string connectionString = "Endpoint=sb://sb-learn-dev599.servicebus.windows.net/;SharedAccessKeyName=RootManageSharedAccessKey;SharedAccessKey=o6tVa/g11zatah6C0TieGVXKHQ0x595SyUSq9NCTTUY=";
string queueName = "sbq-learndevquery";

ServiceBusClient client = new(connectionString);
ServiceBusSender sender = client.CreateSender(queueName);

using ServiceBusMessageBatch messageBatch = await sender.CreateMessageBatchAsync();

for (int i = 0; i <= 3; i++)
{
    if (!messageBatch.TryAddMessage(new ServiceBusMessage($"Message {i}")))
    {
        throw new Exception();
    }
}

try
{
    await sender.SendMessagesAsync(messageBatch);
    Console.WriteLine($"A batch of {messageBatch.Count} has been sent to the queue.");
}
finally
{
    await sender.DisposeAsync();
    await client.DisposeAsync();
}

