# ProductInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fat_amount** | Option<**f64**> | Fat per 100g. | [optional]
**proteins_amount** | Option<**f64**> | Protein per 100g. | [optional]
**carbohydrates_amount** | Option<**f64**> | Carbohydrate per 100g. | [optional]
**energy_amount** | Option<**f64**> | Calories per 100g. | [optional]
**fat_full_amount** | Option<**f64**> | Fat per item. | [optional]
**proteins_full_amount** | Option<**f64**> | Protein per item. | [optional]
**carbohydrates_full_amount** | Option<**f64**> | Carbohydrate per item. | [optional]
**energy_full_amount** | Option<**f64**> | Calories per item. | [optional]
**weight** | Option<**f64**> | Item weight. | [optional]
**group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Stock list group in RMS. | [optional]
**product_category_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Product category in RMS. | [optional]
**r#type** | Option<**String**> | dish | good | modifier. | [optional]
**order_item_type** | Option<[**models::OrderItemType**](OrderItemType.md)> | Product or compound. Depends on modifiers scheme existence. | [optional]
**modifier_schema_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Modifier schema's ID. | [optional]
**modifier_schema_name** | Option<**String**> | Modifier schema's name. | [optional]
**splittable** | **bool** | Is product splittable. | 
**measure_unit** | Option<**String**> | Item's unit of measurement. | [optional]
**size_prices** | Option<[**Vec<models::SizePrice>**](SizePrice.md)> | Prices. | [optional]
**modifiers** | Option<[**Vec<models::SimpleModifierInfo>**](SimpleModifierInfo.md)> | Modifiers. | [optional]
**group_modifiers** | Option<[**Vec<models::GroupModifierInfo>**](GroupModifierInfo.md)> | Modifier groups. | [optional]
**image_links** | Option<**Vec<String>**> | Links to images. | [optional]
**do_not_print_in_cheque** | Option<**bool**> | Do not print on bill. | [optional]
**parent_group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | External menu group. | [optional]
**order** | Option<**i32**> | Product's order (priority) in menu. | [optional]
**full_name_english** | Option<**String**> | Full name in a foreign language. | [optional]
**use_balance_for_sell** | **bool** | Weighed product. | 
**can_set_open_price** | **bool** | Open price. | 
**payment_subject** | Option<**String**> | Payment subject. | [optional]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | ID. | 
**code** | Option<**String**> | SKU. | [optional]
**name** | **String** | Name. | 
**description** | Option<**String**> | Description. | [optional]
**additional_info** | Option<**String**> | Additional information. | [optional]
**tags** | Option<**Vec<String>**> | Tags. | [optional]
**is_deleted** | Option<**bool**> | Is-Deleted attribute. | [optional]
**seo_description** | Option<**String**> | SEO description for client. | [optional]
**seo_text** | Option<**String**> | SEO text for robots. | [optional]
**seo_keywords** | Option<**String**> | SEO key words. | [optional]
**seo_title** | Option<**String**> | SEO header. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


