# NomenclatureResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**correlation_id** | [**uuid::Uuid**](uuid::Uuid.md) | Operation ID. | 
**groups** | [**Vec<models::ProductsGroupInfo>**](ProductsGroupInfo.md) | Stock list group. | 
**product_categories** | [**Vec<models::ProductCategoryInfo>**](ProductCategoryInfo.md) | Menu item category. | 
**products** | [**Vec<models::ProductInfo>**](ProductInfo.md) | Menu items and modifiers. | 
**sizes** | [**Vec<models::Size>**](Size.md) | Item sizes. | 
**revision** | **i64** | The revison (version) of the menu recevied in the response of the request.  This value should be saved by the integration and passed in the `startRevision` field  of the next menu request. If the values in `revision` and `startRevision` are the same,  it means there have been no changes to the menu since the previous request.  In this case, the `groups`, `productCategories`, `products` and `sizes` fields  will not contain any data. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


