# DeliveryRestrictionItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_sum** | Option<**f64**> | The minimum order amount for a given point in a given time interval in this delivery zone. | 
**terminal_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Terminal group ID.                Can be obtained by `/api/1/terminal_groups` operation. | 
**organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Organization ID.                Can be obtained by `/api/1/organizations` operation. | 
**zone** | Option<**String**> | Name of delivery zone from cartography. | 
**week_map** | **i32** | Days of the week. | 
**from** | Option<**i32**> | The time from which the point can process orders from the selected zone, in minutes from the beginning of the day. | 
**to** | Option<**i32**> | The maximum time at which a point can carry an order to a given zone, in minutes from the beginning of the day. | 
**priority** | **i32** | Priority of point in delivery zone. | 
**delivery_duration_in_minutes** | **i64** | Delivery duration in delivery zone. | 
**delivery_service_product_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Link to \"delivery service payment\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


