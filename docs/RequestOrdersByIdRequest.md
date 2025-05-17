# RequestOrdersByIdRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID for which an order search will be performed.                Can be obtained by `/api/1/organizations` operation. | 
**order_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | IDs of orders information on which is required.                > Required if \"posOrderIds\" is null. Must be null if \"posOrderIds\" is not null.                > Maximum allowed \"orderIds\" to request - `200`.    The guaranteed order availability  is the last 7 days. To access earlier orders, use the `/api/1/deliveries/history/by_delivery_date_and_phone` method. | [optional]
**source_keys** | Option<**Vec<String>**> | Source keys. | [optional]
**pos_order_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | POS order IDs information on which is required.                > Required if \"orderIds\" is null. Must be null if \"orderIds\" is not null.                > Maximum allowed \"posOrderIds\" to request - `200`.    The guaranteed order availability  is the last 7 days. To access earlier orders, use the `/api/1/deliveries/history/by_delivery_date_and_phone` method. | [optional]
**return_external_data_keys** | Option<**Vec<String>**> | Keys for retrun external data information. | [optional]
**return_locked_by_user** | Option<**bool**> | Whether to check and return LockedByUser property (see FullOrderUpdateRequestEmployeeId). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


