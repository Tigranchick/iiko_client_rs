# GetAllowedRestrictionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**correlation_id** | [**uuid::Uuid**](uuid::Uuid.md) | Operation ID. | 
**is_allowed** | **bool** | A sign of successful verification. | 
**reject_cause** | Option<**String**> | Reject cause. | 
**address_external_id** | Option<**String**> | Delivery address ID in external mapping system. | 
**location** | Option<[**models::OrderLocation**](OrderLocation.md)> | Coordinates returned by geocoding service. | 
**allowed_items** | [**Vec<models::AllowedItemWithDuration>**](AllowedItemWithDuration.md) | Suitable terminal groups with a delivery duration for them. | 
**rejected_items** | [**Vec<models::RejectItem>**](RejectItem.md) | Rejected items with cause. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


