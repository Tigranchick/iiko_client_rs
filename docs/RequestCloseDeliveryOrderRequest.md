# RequestCloseDeliveryOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delivery_date** | Option<**String**> | Actual delivery time. If empty local iikoFront time will used.   > Allowed from version `8.0.6`. | [optional]
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**order_id** | [**uuid::Uuid**](uuid::Uuid.md) | Order ID. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


