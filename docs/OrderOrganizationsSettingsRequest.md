# OrderOrganizationsSettingsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Organizations IDs which have to be returned. By default - all organizations from apiLogin. | [optional]
**include_disabled** | Option<**bool**> | Attribute that shows that response contains disabled organizations. | [optional]
**parameters** | Option<[**Vec<models::OrderOrganizationSettingsParameters>**](OrderOrganizationSettingsParameters.md)> | Parameters of information to be present in  | [optional]
**return_external_data** | Option<**Vec<String>**> | External data keys that have to be returned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


