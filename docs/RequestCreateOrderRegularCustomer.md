# RequestCreateOrderRegularCustomer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Existing customer ID in RMS.   > If null - the phone number is searched in database, otherwise the new customer is created in RMS. | [optional]
**name** | Option<**String**> | Name of customer.  > Required for new customers (i.e. if \"id\" == null)  > Not required if \"id\" specified. | [optional]
**surname** | Option<**String**> | Last name. | [optional]
**comment** | Option<**String**> | Comment. | [optional]
**birthdate** | Option<**String**> | Date of birth. | [optional]
**email** | Option<**String**> | Email. | [optional]
**should_receive_promo_actions_info** | Option<**bool**> | Deprecated, use \"shouldReceiveOrderStatusNotifications\" instead. | [optional]
**should_receive_order_status_notifications** | Option<**bool**> | Whether customer receives order status notification messages. | [optional]
**gender** | Option<[**models::Gender**](Gender.md)> | Gender. | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


