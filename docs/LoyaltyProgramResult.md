# LoyaltyProgramResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketing_campaign_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Program marketing campaign id. | [optional]
**name** | Option<**String**> | Program name. | [optional]
**discounts** | Option<[**Vec<models::DiscountOperation>**](DiscountOperation.md)> | Discount operations applied to order items. | [optional]
**upsales** | Option<[**Vec<models::Upsale>**](Upsale.md)> | Suggested items to add or advices for customer. | [optional]
**free_products** | Option<[**Vec<models::FreeProductsGroup>**](FreeProductsGroup.md)> | Program free products. | [optional]
**available_combo_specifications** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Ids of combo specification available in current order. | [optional]
**available_combos** | Option<[**Vec<models::AvailableCombo>**](AvailableCombo.md)> | Partially added combos, available for assembly. | [optional]
**need_to_activate_certificate** | Option<**bool**> | Certificate number is required for activation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


