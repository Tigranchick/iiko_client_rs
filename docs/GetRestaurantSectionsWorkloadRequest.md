# GetRestaurantSectionsWorkloadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**restaurant_section_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Collection of restaurant section ID.                Can be obtained by `/api/1/reserve/available_restaurant_sections` operation. | 
**date_from** | **String** | Estimated start time (Local for the terminal). Lower limit.                Order details are stored for 90 days. | 
**date_to** | Option<**String**> | Estimated start time (Local for the terminal). Upper limit. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


