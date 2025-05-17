# RequestOrdersHistoryByDeliveryDateAndPhoneRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone** | **String** | Delivery order phone number. | 
**delivery_date_from** | Option<**String**> | Order delivery date (Local for delivery terminal). Lower limit.                Order details are stored for 90 days. | [optional]
**delivery_date_to** | Option<**String**> | Order delivery date (Local for delivery terminal). Upper limit.                Order details are stored for 90 days. | [optional]
**organization_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Organization ID for which an order search will be performed.                Can be obtained by `/api/1/organizations` operation. | 
**start_revision** | Option<**i64**> | Revision start number beginning from which (but not including) orders will be returned.                > Maximum revision offset to request - `3 hours`. | [optional]
**source_keys** | Option<**Vec<String>**> | Source keys. | [optional]
**rows_count** | **i32** | Maximum number of items returned.                > Maximum numbers of items to request - `200`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


