# RequestCancelOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**order_id** | [**uuid::Uuid**](uuid::Uuid.md) | Order ID. | 
**moved_order_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Fill this field with id of the new order if current order has been moved to the new RMS/terminal group. | [optional]
**cancel_cause_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Cancel order dictionary item id.   > Allowed from version `7.7.1`. | [optional]
**cancel_comment** | Option<**String**> | Comment to the delivery cancellation.   > Allowed from version `8.7.6`. | [optional]
**removal_type_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Removal type (for delete printed order items).   > Allowed from version `7.7.1`. | [optional]
**removal_comment** | Option<**String**> | Comment to the charge-off.   > Allowed from version `8.7.6`. | [optional]
**user_id_for_writeoff** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | User for writeoff (for delete printed order items).   > Allowed from version `7.7.1`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


