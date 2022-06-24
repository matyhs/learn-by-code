using Microsoft.Identity.Client;

string clientId = "7d9003b9-f2f5-49c0-bdc7-3ab698370ca3";
string tenantId = "8bc2ba01-5996-4cd7-b2dd-00b8ee9cfe81";

var app = PublicClientApplicationBuilder.Create(clientId)
                                        .WithAuthority(AzureCloudInstance.AzurePublic, tenantId)
                                        .WithRedirectUri("http://localhost")
                                        .Build();

var authenticationResult = await app.AcquireTokenInteractive(new string[] { "user.read" })
                                    .ExecuteAsync();

Console.WriteLine($"Access token: {authenticationResult.AccessToken}");
  
