# ProductsGroupInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image_links** | **Vec<String>** | Links to images. | 
**parent_group** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Parent group. | [optional]
**order** | **i32** | Group's order (priority) in menu. | 
**is_included_in_menu** | **bool** | On-the-menu attribute. | 
**is_group_modifier** | **bool** | Is group modifier attribute.  * true - group modifier.  * false - external menu group. | 
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


