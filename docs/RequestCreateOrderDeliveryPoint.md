# RequestCreateOrderDeliveryPoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**coordinates** | Option<[**models::Coordinates**](Coordinates.md)> | Delivery address coordinates.  > Allowed from version `7.7.3`. | [optional]
**address** | Option<[**models::RequestCreateOrderAddress**](RequestCreateOrderAddress.md)> | Order delivery address.                > The use of type **City** is allowed if the parameter **addressFormatType == City**.                > Can be obtained by `/api/1/organizations` or `/api/1/organizations/settings` operations (`addressFormatType` parameter). | [optional]
**external_cartography_id** | Option<**String**> | Delivery location custom code in customer's API system. | [optional]
**comment** | Option<**String**> | Additional information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


