# GetWebHookSettingsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**correlation_id** | [**uuid::Uuid**](uuid::Uuid.md) | Operation ID. | 
**api_login_name** | Option<**String**> | Api login name. | 
**web_hooks_uri** | Option<**String**> | Webhook handler url. | 
**auth_token** | Option<**String**> | Authorization token to pass to the webhook handler. | [optional]
**web_hooks_filter** | Option<[**models::WebHooksFilter**](WebHooksFilter.md)> | Webhooks filter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


