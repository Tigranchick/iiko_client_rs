# \MessagesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_loyalty_iiko_check_sms_sending_possibility_post**](MessagesApi.md#api1_loyalty_iiko_check_sms_sending_possibility_post) | **POST** /api/1/loyalty/iiko/check_sms_sending_possibility | Check sms sending possibility.
[**api1_loyalty_iiko_check_sms_status_post**](MessagesApi.md#api1_loyalty_iiko_check_sms_status_post) | **POST** /api/1/loyalty/iiko/check_sms_status | Check SMS status.
[**api1_loyalty_iiko_message_send_email_post**](MessagesApi.md#api1_loyalty_iiko_message_send_email_post) | **POST** /api/1/loyalty/iiko/message/send_email | Send email.
[**api1_loyalty_iiko_message_send_sms_post**](MessagesApi.md#api1_loyalty_iiko_message_send_sms_post) | **POST** /api/1/loyalty/iiko/message/send_sms | Send sms.



## api1_loyalty_iiko_check_sms_sending_possibility_post

> models::SmsSendingPossibilityResponse api1_loyalty_iiko_check_sms_sending_possibility_post(authorization, timeout, sms_sending_possibility_request)
Check sms sending possibility.

Check sms sending possibility before send sms message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**sms_sending_possibility_request** | Option<[**SmsSendingPossibilityRequest**](SmsSendingPossibilityRequest.md)> |  |  |

### Return type

[**models::SmsSendingPossibilityResponse**](SmsSendingPossibilityResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_check_sms_status_post

> models::CheckSmsStatusResponse api1_loyalty_iiko_check_sms_status_post(authorization, timeout, check_sms_status_request)
Check SMS status.

Check the status of sending SMS messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**check_sms_status_request** | Option<[**CheckSmsStatusRequest**](CheckSmsStatusRequest.md)> |  |  |

### Return type

[**models::CheckSmsStatusResponse**](CheckSmsStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_message_send_email_post

> serde_json::Value api1_loyalty_iiko_message_send_email_post(authorization, timeout, send_email_request)
Send email.

Send email message to specified email address. Sending proceed according iikoCard organization's settings.   > Restriction group: `Loyalty: messages`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**send_email_request** | Option<[**SendEmailRequest**](SendEmailRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_message_send_sms_post

> models::SendSmsResponse api1_loyalty_iiko_message_send_sms_post(authorization, timeout, send_sms_request)
Send sms.

Send sms message to specified phone number. Sending proceed according iikoCard organization's settings.   > Restriction group: `Loyalty: messages`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**send_sms_request** | Option<[**SendSmsRequest**](SendSmsRequest.md)> |  |  |

### Return type

[**models::SendSmsResponse**](SendSmsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

