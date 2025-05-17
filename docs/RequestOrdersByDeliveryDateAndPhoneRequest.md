# RequestOrdersByDeliveryDateAndPhoneRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone** | Option<**String**> | Delivery order phone number. | 
**delivery_date_from** | Option<**String**> | Order delivery date (Local for delivery terminal). Lower limit.                The guaranteed order availability  is the last 7 days. To access earlier orders, use the `/api/1/deliveries/history/by_delivery_date_and_phone` method. | [optional]
**delivery_date_to** | Option<**String**> | Order delivery date (Local for delivery terminal). Upper limit. | [optional]
**organization_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Organization ID for which an order search will be performed.                Can be obtained by `/api/1/organizations` operation. | 
**start_revision** | Option<**i64**> | Revision start number beginning from which (but not including) new/edited orders will be returned. | [optional]
**source_keys** | Option<**Vec<String>**> | Source keys. | [optional]
**rows_count** | Option<**i32**> | Maximum number of items returned.  <remarks>  If null, all items will be returned.  </remarks> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


