# RequestUpdateOrderChangeDeliveryPointRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**order_id** | [**uuid::Uuid**](uuid::Uuid.md) | Order ID. | 
**new_delivery_point** | [**models::RequestCreateOrderDeliveryPoint**](RequestCreateOrderDeliveryPoint.md) | New address of delivery. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


