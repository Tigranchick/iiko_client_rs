# DeliveryOrderType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Order type ID in RMS. | 
**name** | **String** | Order type name. | 
**order_service_type** | [**models::DeliveryOrderServiceType**](DeliveryOrderServiceType.md) | Service type. | 
**is_deleted** | Option<**bool**> | IsDeleted attribute of order type. | [optional]
**external_revision** | Option<**i64**> | External system revision number. | [optional]
**is_default** | Option<**bool**> | IsDefault attribute of order type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


