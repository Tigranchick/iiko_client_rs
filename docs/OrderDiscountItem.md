# OrderDiscountItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**discount_type** | [**models::OrderDiscountType**](OrderDiscountType.md) | Discount type.                 Can be obtained by `/api/1/discounts` operation. | 
**sum** | **f64** | Total. | 
**selective_positions** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Order item positions. | [optional]
**selective_positions_with_sum** | Option<[**Vec<models::OrderPositionWithSum>**](OrderPositionWithSum.md)> | Order item positions with position discount sum.   > Allowed from version `8.5.6`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


