# RequestOrdersByRevisionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_revision** | **i64** | Revision start number beginning from which (but not including) new/edited orders will be returned.                > Maximum revision offset to request - `3 hours`. | 
**organization_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Organization ID for which an order search will be performed.                Can be obtained by `/api/1/organizations` operation. | 
**source_keys** | Option<**Vec<String>**> | Source keys. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


