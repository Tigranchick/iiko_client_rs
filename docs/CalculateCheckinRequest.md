# CalculateCheckinRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order** | [**models::RequestCreateOrder**](RequestCreateOrder.md) | Order details. | 
**coupon** | Option<**String**> | Number of applied coupon. Can be null. | [optional]
**referrer_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Referrer id. | [optional]
**terminal_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Identifier of a target terminal. Should be used only when auto distribution is off and no call center operator is available. | [optional]
**available_payment_marketing_campaign_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of identifiers of applied campaigns. Should be empty if no payment method is used. | [optional]
**applicable_manual_conditions** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of manually applied to order conditions. | [optional]
**dynamic_discounts** | Option<[**Vec<models::DynamicDiscount>**](DynamicDiscount.md)> | Applicable manual discounts. | [optional]
**is_loyalty_trace_enabled** | Option<**bool**> | Loyalty trace is enabled. | [optional]
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization id. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


