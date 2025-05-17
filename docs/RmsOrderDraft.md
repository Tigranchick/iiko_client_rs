# RmsOrderDraft

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Order ID. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**locked_by_user** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | ID of the employee, who is editing this draft. | [optional]
**order** | [**models::RmsDeliveryOrderDraft**](RmsDeliveryOrderDraft.md) | Order | 
**terminal_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Terminal group ID. | [optional]
**created_at** | **String** | Draft creation time (UTC). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


