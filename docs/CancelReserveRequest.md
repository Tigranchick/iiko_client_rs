# CancelReserveRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID of the reserve.                Can be obtained by `/api/1/organizations` operation. | 
**reserve_id** | [**uuid::Uuid**](uuid::Uuid.md) | Reserve ID to cancel. | 
**cancel_reason** | [**models::ReserveCancelReason**](ReserveCancelReason.md) | Reason to cancel planned event. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


