# OrdersByDeliveryDateAndFilterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Organization ID for which an order search will be performed.                Can be obtained by `/api/1/organizations` operation. | 
**terminal_group_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of terminal groups IDs. | [optional]
**delivery_date_from** | Option<**String**> | Order delivery date (Local for delivery terminal). Lower limit.                The guaranteed order availability  is the last 7 days. To access earlier orders, use the `/api/1/deliveries/history/by_delivery_date_and_phone` method. | [optional]
**delivery_date_to** | Option<**String**> | Order delivery date (Local for delivery terminal). Upper limit. | [optional]
**statuses** | Option<[**Vec<models::DeliveryStatus>**](DeliveryStatus.md)> | Allowed order statuses. | [optional]
**has_problem** | Option<**bool**> | If true, delivery has a problem.  > Conditions under which the order has a problem:  > * order.problem.hasProblem is true;  > * order status is Unconfirmed and CookingStartTime before now;  > * order status is ReadyForCooking and (CookingStartTime + timeToCookingErrorTimeout) before now;  > * order status is CookingCompleted or Waiting and (CookingStartTime + cookingTimeout) before now. | [optional]
**order_service_type** | Option<[**models::RequestCreateOrderServiceType**](RequestCreateOrderServiceType.md)> | Order service type. | [optional]
**search_text** | Option<**String**> | Value for search. Used for prefix search. | [optional]
**time_to_cooking_error_timeout** | Option<**i32**> | Error timeout for status time to cooking, in seconds. | [optional]
**cooking_timeout** | Option<**i32**> | Expected cooking time, in seconds. | [optional]
**sort_property** | Option<[**models::RequestOrderSortProperty**](RequestOrderSortProperty.md)> | Sorting property. | [optional]
**sort_direction** | Option<[**models::SortDirection**](SortDirection.md)> | Sorting direction. | [optional]
**rows_count** | Option<**i32**> | Maximum number of items returned. | [optional]
**source_keys** | Option<**Vec<String>**> | Source keys. | [optional]
**order_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Order IDs.                > Must be null if \"posOrderIds\" is not null. | [optional]
**pos_order_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | POS order IDs.                > Must be null if \"orderIds\" is not null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


