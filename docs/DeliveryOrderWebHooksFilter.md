# DeliveryOrderWebHooksFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_statuses** | Option<[**Vec<models::DeliveryStatus>**](DeliveryStatus.md)> | Statuses of orders, when changing which need to send a  | [optional]
**item_statuses** | Option<[**Vec<models::OrderItemStatus>**](OrderItemStatus.md)> | Statuses of order items, when changing which need to send a  | [optional]
**errors** | Option<**bool**> | Flag for errors. | [optional]
**returned_external_data_keys** | Option<**Vec<String>**> | Order external data keys to return in a  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


