# RequestUpdateOrderChangeCompleteBeforeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**order_id** | [**uuid::Uuid**](uuid::Uuid.md) | Order ID. | 
**new_complete_before** | **String** | New time when client wants the order to be delivered (Local for delivery terminal). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


