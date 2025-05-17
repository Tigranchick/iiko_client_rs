# RequestCreateOrderDiscountsInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**card** | Option<[**models::RequestCreateOrderDiscountCard**](RequestCreateOrderDiscountCard.md)> | Track of discount card to be applied to order. | [optional]
**discounts** | Option<[**Vec<models::RequestCreateOrderDiscount>**](RequestCreateOrderDiscount.md)> | Discounts/surcharges.   > Type **iikoCard** allowed from version `7.4.4`. | [optional]
**fixed_loyalty_discounts** | Option<**bool**> | Whether loyalty discounts should be fixed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


