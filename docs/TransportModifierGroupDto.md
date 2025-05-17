# TransportModifierGroupDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**items** | Option<[**Vec<models::TransportModifierItemDto>**](TransportModifierItemDto.md)> |  | [optional]
**name** | Option<**String**> | Modifiers group name | [optional]
**description** | Option<**String**> | Modifiers group description | [optional]
**restrictions** | Option<[**models::ModifierRestrictionsDto**](ModifierRestrictionsDto.md)> |  | [optional]
**can_be_divided** | Option<**bool**> | Whether the modifier can be splitted | [optional]
**item_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Modifiers group id | [optional]
**child_modifiers_have_min_max_restrictions** | Option<**bool**> | Whether child modifiers can have their own restrictions, or only group ones | [optional]
**sku** | Option<**String**> | Modifiers group code | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


