# OrderGetOrganizationsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Organizations IDs which have to be returned. By default - all organizations from apiLogin.                Can be obtained by `/api/1/organizations` operation. | [optional]
**return_additional_info** | Option<**bool**> | A sign whether additional information about the organization should be returned (RMS version, country, restaurantAddress, etc.),    or only minimal information should be returned (id and name). | [optional]
**include_disabled** | Option<**bool**> | Attribute that shows that response contains disabled organizations. | [optional]
**return_external_data** | Option<**Vec<String>**> | External data keys that have to be returned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


