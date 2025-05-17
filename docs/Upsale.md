# Upsale

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_action_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Id of action that caused the suggestion. | [optional]
**suggestion_text** | Option<**String**> | Suggestion text. | [optional]
**description_for_user** | Option<**String**> | Description for user. | [optional]
**product_codes** | Option<**Vec<String>**> | Codes of products that suggested to be added to order. | [optional]
**products** | Option<[**Vec<models::UpsaleProduct>**](UpsaleProduct.md)> | Products that suggested to be added to order. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


