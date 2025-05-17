# ReserveInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Banquet/reserve ID. | 
**external_number** | Option<**String**> | Banquet/reserve external number. | [optional]
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**timestamp** | **i64** | Timestamp of most recent banquet/reserve change that took place on iikoTransport server. | 
**creation_status** | [**models::OrderCreationStatus**](OrderCreationStatus.md) | Banquet/reserve creation status. In case of asynchronous creation, it allows to track the instance an banquet/reserve was validated/created in iikoFront. | 
**error_info** | Option<[**models::ErrorInfo**](ErrorInfo.md)> | Banquet/reserve creation error details.  > Required only if \"creationStatus\"=\"Error\". | [optional]
**is_deleted** | **bool** | Banquet/reserve is deleted. | 
**reserve** | Option<[**models::Reserve**](Reserve.md)> | Banquet/reserve. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


