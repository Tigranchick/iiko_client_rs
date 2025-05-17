# RequestCreateOrderAddressLegacy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**street** | [**models::RequestCreateOrderStreet**](RequestCreateOrderStreet.md) | Street.  > It's required specify only \"classifierId\" or \"id\" or \"name\" and \"city\". | 
**index** | Option<**String**> | Postcode. | [optional]
**house** | **String** | House. | 
**building** | Option<**String**> | Building. | [optional]
**flat** | Option<**String**> | Apartment.  > In case useUaeAddressingSystem enabled max length - 100, otherwise - 10. | [optional]
**entrance** | Option<**String**> | Entrance. | [optional]
**floor** | Option<**String**> | Floor. | [optional]
**doorphone** | Option<**String**> | Intercom. | [optional]
**region_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Delivery area ID. | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


