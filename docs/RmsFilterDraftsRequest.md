# RmsFilterDraftsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Organization ID for which the order drafts search will be performed.                Can be obtained by `/api/1/organizations` operation. | 
**date_from** | Option<**String**> | Draft creation time (UTC). Lower limit. | [optional]
**date_to** | Option<**String**> | Draft creation time (UTC). Upper limit. | [optional]
**phone** | Option<**String**> | Phone number. | [optional]
**limit** | Option<**i32**> | Desirable size of result set (50 by default). | [optional]
**offset** | Option<**i32**> | Offset from the beginning of full result set for paging. | [optional]
**source_keys** | Option<**Vec<String>**> | Delivery sources (DeliveryClub, PH and etc.) | [optional]
**terminal_group_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of terminal groups IDs. | [optional]
**search_text** | Option<**String**> | Value for search. Used for prefix search. | [optional]
**sort_property** | Option<[**models::RmsOrderDraftSortProperty**](RmsOrderDraftSortProperty.md)> | Sorting property. | [optional]
**sort_direction** | Option<[**models::SortDirection**](SortDirection.md)> | Sorting direction. | [optional]
**operator_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of drafts operator IDs. | [optional]
**order_type_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of drafts order type IDs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


