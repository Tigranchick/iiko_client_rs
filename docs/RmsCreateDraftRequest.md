# RmsCreateDraftRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID of the new order.                Can be obtained by `/api/1/organizations` operation. | 
**order** | [**models::RmsDeliveryOrderDraft**](RmsDeliveryOrderDraft.md) | Order item. | 
**terminal_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Front group ID the order must be sent to.                Can be obtained by `/api/1/terminal_groups` operation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


