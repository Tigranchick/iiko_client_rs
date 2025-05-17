# RequestCreateOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID of the new order.                Can be obtained by `/api/1/organizations` operation. | 
**terminal_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Front group ID the order must be sent to.    Can be obtained by `/api/1/terminal_groups` operation. | [optional]
**create_order_settings** | Option<[**models::CreateOrderSettings**](CreateOrderSettings.md)> | Order creation parameters. | [optional]
**order** | [**models::RequestCreateOrderDeliveryOrder**](RequestCreateOrderDeliveryOrder.md) | Order | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


