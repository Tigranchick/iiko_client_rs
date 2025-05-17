# RequestChangeDriverInfoRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**order_id** | [**uuid::Uuid**](uuid::Uuid.md) | Order ID. | 
**driver_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Driver ID.                Can be obtained by `/api/1/employees/couriers` operation. | [optional]
**estimated_time** | Option<**String**> | Delivery estimated time. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


