# \WebhooksApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_webhooks_settings_post**](WebhooksApi.md#api1_webhooks_settings_post) | **POST** /api/1/webhooks/settings | Get webhooks settings for specified organization and authorized API login.
[**api1_webhooks_update_settings_post**](WebhooksApi.md#api1_webhooks_update_settings_post) | **POST** /api/1/webhooks/update_settings | Update webhooks settings for specified organization and authorized API login.



## api1_webhooks_settings_post

> models::GetWebHookSettingsResponse api1_webhooks_settings_post(authorization, timeout, get_web_hook_settings_request)
Get webhooks settings for specified organization and authorized API login.

   > Restriction group: `Organizations: settings`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_web_hook_settings_request** | Option<[**GetWebHookSettingsRequest**](GetWebHookSettingsRequest.md)> |  |  |

### Return type

[**models::GetWebHookSettingsResponse**](GetWebHookSettingsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_webhooks_update_settings_post

> models::CorrelationIdResponse api1_webhooks_update_settings_post(authorization, timeout, update_web_hook_settings_request)
Update webhooks settings for specified organization and authorized API login.

   > Restriction group: `Organizations: settings`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**update_web_hook_settings_request** | Option<[**UpdateWebHookSettingsRequest**](UpdateWebHookSettingsRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

