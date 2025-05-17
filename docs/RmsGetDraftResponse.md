# RmsGetDraftResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**correlation_id** | [**uuid::Uuid**](uuid::Uuid.md) | Operation ID. | 
**order** | [**models::RmsDeliveryOrderDraft**](RmsDeliveryOrderDraft.md) | Order draft object. | 
**locked_by_user** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | ID of the employee who is currently editing this draft. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID. | 
**terminal_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Terminal group ID. | [optional]
**created_at** | **String** | Draft creation time (UTC). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


