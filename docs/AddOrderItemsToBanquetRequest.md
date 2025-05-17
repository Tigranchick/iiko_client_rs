# AddOrderItemsToBanquetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reserve_id** | [**uuid::Uuid**](uuid::Uuid.md) | Banquet ID. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**items** | [**Vec<models::RequestCreateOrderItem>**](RequestCreateOrderItem.md) | Order items (may include ProductOrderItem or CompoundOrderItem). | 
**combos** | Option<[**Vec<models::RequestCreateOrderCombo>**](RequestCreateOrderCombo.md)> | Combos.   > Allowed from version `7.6.1`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


