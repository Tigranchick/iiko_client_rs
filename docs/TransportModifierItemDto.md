# TransportModifierItemDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prices** | Option<[**Vec<models::TransportPriceDto>**](TransportPriceDto.md)> |  | [optional]
**sku** | Option<**String**> | Modifier's code | [optional]
**name** | Option<**String**> | Modifier's name | [optional]
**description** | Option<**String**> | Modifier's description | [optional]
**button_image** | Option<**String**> | Links to images | [optional]
**restrictions** | Option<[**models::ModifierRestrictionsDto**](ModifierRestrictionsDto.md)> |  | [optional]
**allergen_groups** | Option<[**Vec<models::AllergenGroupDto>**](AllergenGroupDto.md)> |  | [optional]
**nutrition_per_hundred_grams** | Option<[**serde_json::Value**](.md)> |  | [optional]
**portion_weight_grams** | Option<**f32**> | Modifier's weight in gramms | [optional]
**tags** | Option<[**Vec<models::TagDto>**](TagDto.md)> |  | [optional]
**item_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Modifier's Id | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


