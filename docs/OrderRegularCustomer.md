# OrderRegularCustomer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Customer ID. | 
**name** | **String** | Name. | 
**surname** | Option<**String**> | Last name. | [optional]
**comment** | Option<**String**> | Comment. | [optional]
**gender** | Option<[**models::Gender**](Gender.md)> | Sex. | [optional]
**in_blacklist** | Option<**bool**> | Is client in blacklist. | [optional]
**blacklist_reason** | Option<**String**> | Reason why client was added to blacklist. | [optional]
**birthdate** | Option<**String**> | Date of birth.   > Allowed from version `7.6.1`. | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


