# TransportItemDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sizes** | Option<[**Vec<models::TransportItemSizeDto>**](TransportItemSizeDto.md)> |  | [optional]
**sku** | Option<**String**> | Product code | [optional]
**name** | Option<**String**> | Product name | [optional]
**description** | Option<**String**> | Product description | [optional]
**allergen_groups** | Option<[**Vec<models::AllergenGroupDto>**](AllergenGroupDto.md)> |  | [optional]
**item_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Product ID | [optional]
**modifier_schema_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Modifier schema ID | [optional]
**tax_category** | Option<[**models::TaxCategoryDto**](TaxCategoryDto.md)> |  | [optional]
**order_item_type** | Option<**String**> | Product or compound. Depends on modifiers scheme existence | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


