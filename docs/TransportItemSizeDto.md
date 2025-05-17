# TransportItemSizeDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prices** | Option<[**Vec<models::TransportPriceDto>**](TransportPriceDto.md)> |  | [optional]
**item_modifier_groups** | Option<[**Vec<models::TransportModifierGroupDto>**](TransportModifierGroupDto.md)> |  | [optional]
**sku** | Option<**String**> | Unique size code, consists of the product code and the name of the size, if the product has one size, then the size code will be equal to the product code | [optional]
**size_code** | Option<**String**> |  | [optional]
**size_name** | Option<**String**> | Name of the product size, the name can be empty if there is only one size in the list | [optional]
**is_default** | Option<**bool**> | Whether it is a default size of the product. If the product has one size, then the parameter will be true, if the product has several sizes, none of them can be default. | [optional]
**portion_weight_grams** | Option<**f32**> | Size's weight | [optional]
**size_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | ID size, can be empty if the default size is selected and it is the only size in the list | [optional]
**nutrition_per_hundred_grams** | Option<[**serde_json::Value**](.md)> |  | [optional]
**button_image_url** | Option<**String**> | links to images | [optional]
**button_image_cropped_url** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


