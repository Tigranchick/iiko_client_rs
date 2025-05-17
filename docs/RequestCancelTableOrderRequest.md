# RequestCancelTableOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**order_id** | [**uuid::Uuid**](uuid::Uuid.md) | Order ID. | 
**removal_type_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Removal type (used during deletion of printed order items). | [optional]
**removal_comment** | Option<**String**> | Comment to the charge-off. | [optional]
**user_id_for_writeoff** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | User for writeoff (used during deletion of printed order items). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


