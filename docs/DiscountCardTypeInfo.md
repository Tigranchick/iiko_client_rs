# DiscountCardTypeInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Discount ID in RMS. | 
**name** | Option<**String**> | Discount name. | 
**percent** | **f64** | Total discount rate.  > Ignored if \"isCategorisedDiscount\" specified. | 
**is_categorised_discount** | **bool** | Whether it is category discount or not.  > If true, \"productCategoryDiscounts\" discounts will apply. | 
**product_category_discounts** | [**Vec<models::ProductCategoryDiscount>**](ProductCategoryDiscount.md) | Category discount. | 
**comment** | Option<**String**> | Comment. | [optional]
**can_be_applied_selectively** | **bool** | Whether discount allows for selected application to individual items at user's discretion. | 
**min_order_sum** | Option<**f64**> | Minimum order amount required for discount application.  If order amount is less than specified threshold, discount does not apply. | [optional]
**mode** | [**models::DiscountCardMode**](DiscountCardMode.md) | Discount type.     Can be obtained by `/api/1/discounts` operation. | 
**sum** | **f64** | Fixed amount.  > Triggers if fixed amount has been specified. | 
**can_apply_by_card_number** | **bool** | Can be applied by card No.  > If true, it's enough to enter discount card No. (card swiping not required) | 
**is_manual** | **bool** | Created manually. | 
**is_card** | **bool** | Executed by card. | 
**is_automatic** | **bool** | Created automatically. | 
**is_deleted** | Option<**bool**> | IsDeleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


