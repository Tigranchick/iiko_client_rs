# RejectItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**terminal_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Terminal group ID.                Can be obtained by `/api/1/terminal_groups` operation. | 
**organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**zone** | Option<**String**> | Delivery zone name which this TerminalGroupId belongs to. | [optional]
**reject_code** | [**models::DeliveryRestrictionRejectCode**](DeliveryRestrictionRejectCode.md) | Reject cause code. | 
**reject_hint** | **String** | Reject hint. | 
**reject_item_data** | Option<[**models::RejectItemData**](RejectItemData.md)> | Reject additional information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


