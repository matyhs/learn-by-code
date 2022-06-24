@minLength(3)
@maxLength(24)
param storageAccountName string
param storageAccountKind string = 'StorageV2'
param skuName string = 'Premium_LRS'

var location = resourceGroup().location

resource storageAccount 'Microsoft.Storage/storageAccounts@2021-06-01' = {
  name: storageAccountName
  location: location
  sku: {
    name: skuName
  }
  kind: storageAccountKind
}
