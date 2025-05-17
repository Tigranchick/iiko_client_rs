# \NotificationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_notifications_send_post**](NotificationsApi.md#api1_notifications_send_post) | **POST** /api/1/notifications/send | Send notification to external systems (iikoFront and iikoWeb).



## api1_notifications_send_post

> models::CorrelationIdResponse api1_notifications_send_post(authorization, timeout, send_notification_request)
Send notification to external systems (iikoFront and iikoWeb).

   > Restriction group: `Notifications`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**send_notification_request** | Option<[**SendNotificationRequest**](SendNotificationRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

