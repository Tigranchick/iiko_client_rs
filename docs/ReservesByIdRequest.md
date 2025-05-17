# ReservesByIdRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID for which an order search will be performed.                Can be obtained by `/api/1/organizations` operation. | 
**reserve_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | IDs of banquets/reserves information on which is required. | 
**source_keys** | Option<**Vec<String>**> | Source keys. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


