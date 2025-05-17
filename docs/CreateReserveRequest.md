# CreateReserveRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID of a new banquet/reserve.                Can be obtained by `/api/1/organizations` operation. | 
**terminal_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Front group ID an banquet/reserve must be sent to.                Can be obtained by `/api/1/terminal_groups` operation. | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Banquet/reserve ID. Must be unique. | [optional]
**external_number** | Option<**String**> | Banquet/reserve external number.   > Allowed from version `8.0.6`. | [optional]
**order** | Option<[**models::RequestReserveOrder**](RequestReserveOrder.md)> | Order Used only at a banquet. | [optional]
**customer** | [**models::RequestCreateOrderRegularCustomer**](RequestCreateOrderRegularCustomer.md) | Customer. | 
**phone** | **String** | Telephone number.  > Must begin with symbol \"+\" and must be at least 8 digits. | 
**guests_count** | Option<**i32**> | Number of guests. | [optional]
**comment** | Option<**String**> | Banquet/reserve comment. | [optional]
**duration_in_minutes** | **i64** | Estimated banquet duration. | 
**should_remind** | **bool** | Whether to remind staff to prepare table beforehand. | 
**table_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Reserved tables. | 
**estimated_start_time** | **String** | Estimated time when reserve will be closed or banquet will be started (Local for the terminal).  Reservation can be made up to 90 days prior to the date. | 
**transport_to_front_timeout** | Option<**i32**> | Timeout in seconds that specifies how much time is given for banquet/reserve to reach iikoFront.   After this time, banquet/reserve is nullified if iikoFront doesn't take it. By default - 8 seconds. | [optional]
**guests** | Option<[**models::GuestsInfo**](GuestsInfo.md)> | Guests information. | [optional]
**event_type** | Option<**String**> | Event type.   > Allowed from version `8.5.6`. | [optional]
**create_reserve_settings** | Option<[**models::CreateOrderSettings**](CreateOrderSettings.md)> | Reserve creation parameters. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


