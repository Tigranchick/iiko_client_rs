# CreateOrderSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transport_to_front_timeout** | Option<**i32**> | Timeout in seconds that specifies how much time is given for order to reach iikoFront.   After this time, order is nullified if iikoFront doesn't take it. By default - 8 seconds. | [optional]
**check_stop_list** | Option<**bool**> | Flag indicating whether there's need to check order items in out-of-stock list.                Unable if `terminalGroupId` is null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


